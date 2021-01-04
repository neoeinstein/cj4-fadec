//! The CJ4 FADEC controller module

use crate::control_params::{ThrottleAxis, ThrottleMode, ThrottlePercent, ThrustValue};
use avmath::isa::PressureAltitude;
use serde::{Deserialize, Serialize};
use uom::fmt::DisplayStyle::Abbreviation;
use uom::num_traits::{clamp, clamp_min};
use uom::si::{
    acceleration::foot_per_second_squared,
    f64::*,
    force::poundal,
    length::foot,
    mass_density::slug_per_cubic_foot,
    mass_rate::pound_per_second,
    momentum::pound_foot_per_second,
    ratio::{percent, ratio},
    time::second,
    volume::cubic_foot,
};
use wt_systems::{PidConfiguration, PidController};

/// The CJ4 FADEC controller
#[derive(Debug, Serialize, Deserialize)]
pub struct FadecController {
    climb_pid_config: PidConfiguration<Force>,
    pid_state: PidController<Force>,
    throttle_selected: Ratio,
    enabled: bool,
}

impl Default for FadecController {
    fn default() -> Self {
        Self {
            climb_pid_config: ClimbFadecPidConfiguration::default(),
            pid_state: PidController::default(),
            throttle_selected: Ratio::new::<ratio>(0.),
            enabled: true,
        }
    }
}

impl FadecController {
    /// Steps the FADEC controller to command the virtual throttle lever
    /// position changes required to obtain the desired thrust based on the
    /// current throttle mode
    #[allow(clippy::too_many_arguments)] // TODO reduce this out some
    pub fn get_desired_throttle(
        &mut self,
        current_throttle: Ratio,
        throttle_mode: ThrottleMode,
        engine_thrust: Force,
        mach_number: Ratio,
        ambient_density: MassDensity,
        pressure_altitude: PressureAltitude,
        delta_t: Time,
    ) -> (ThrustValue, ThrottlePercent) {
        if !self.enabled {
            self.throttle_selected = current_throttle;
            let throttle_exp = Ratio::new::<ratio>(self.throttle_selected.get::<ratio>().powf(3.5));
            return (
                ThrustValue::from_ratio(throttle_exp),
                ThrottlePercent::from_ratio(self.throttle_selected),
            );
        }

        let thrust_efficiency = Ratio::new::<percent>(93.0);

        match throttle_mode {
            ThrottleMode::Takeoff => {
                //self.pid_state.reset();
                (ThrustValue::MAX, ThrottlePercent::MAX)
            }
            ThrottleMode::Climb => {
                let gross_thrust = convert_to_gross_thrust(engine_thrust, mach_number);
                let max_density_thrust = get_max_density_thrust(ambient_density);
                let max_effective_thrust = max_density_thrust * thrust_efficiency;

                // println!("Raw thrust: {:.3}, Airspeed: {:.3} M, Gross thrust: {:.3}, Ambient density: {:.4}, Max density thrust: {:.3}, altitude: {:.0}", engine_thrust.into_format_args(poundal, Abbreviation), mach_number.into_format_args(ratio, Abbreviation), gross_thrust.into_format_args(poundal, Abbreviation), ambient_density.into_format_args(slug_per_cubic_foot, Abbreviation), max_density_thrust.into_format_args(poundal, Abbreviation), pressure_altitude.remove_context().into_format_args(foot, Abbreviation));

                let base_thrust = Force::new::<poundal>(2050.);
                let low_altitude_thrust_gain =
                    calculate_low_altitude_thrust_gain(pressure_altitude);
                let low_altitude_thrust_target: Force = base_thrust + low_altitude_thrust_gain;

                let thrust_target: Force = if max_effective_thrust < low_altitude_thrust_target {
                    let high_altitude_thrust_loss =
                        calculate_high_altitude_thrust_loss(pressure_altitude);
                    let high_altitude_thrust_target =
                        max_effective_thrust - high_altitude_thrust_loss;

                    // println!(
                    //     "High altitude thrust target: {:.3}",
                    //     high_altitude_thrust_target.into_format_args(poundal, Abbreviation)
                    // );

                    high_altitude_thrust_target
                } else {
                    // println!(
                    //     "Low altitude thrust target: {:.3}",
                    //     low_altitude_thrust_target.into_format_args(poundal, Abbreviation)
                    // );

                    low_altitude_thrust_target
                };

                let output = self.pid_state.step(
                    thrust_target,
                    &self.climb_pid_config,
                    gross_thrust,
                    delta_t,
                );

                self.throttle_selected += output;
                // println!("Thrust target: {:.4} (error: {:+.4}); commanding change of {:+.4} to {:.4} of maximum", thrust_target.into_format_args(poundal, Abbreviation), self.pid_state.prior_error().into_format_args(poundal, Abbreviation), output.into_format_args(ratio, Abbreviation), self.throttle_selected.into_format_args(ratio, Abbreviation));

                (
                    ThrustValue::from_force(thrust_target),
                    ThrottlePercent::from_ratio(self.throttle_selected),
                )
            }
            ThrottleMode::Cruise | ThrottleMode::Undefined => {
                self.throttle_selected = current_throttle;
                let cruise_normalized_throttle =
                    ThrottleAxis::from_ratio(current_throttle).normalize_cruise();
                let effective_thrust = cruise_normalized_throttle * thrust_efficiency;

                //self.pid_state.reset();
                // println!("Current throttle: {:.4} ({:.4} of cruise; {:.4} effective); Commanding engine to {:.4} of maximum", current_throttle.into_format_args(ratio, Abbreviation), cruise_normalized_throttle.into_format_args(ratio, Abbreviation), effective_thrust.into_format_args(ratio, Abbreviation), effective_thrust.into_format_args(ratio, Abbreviation));

                (
                    ThrustValue::from_ratio(effective_thrust),
                    ThrottlePercent::from_ratio(effective_thrust),
                )
            }
        }
    }
}

fn calculate_low_altitude_thrust_gain(pressure_altitude: PressureAltitude) -> Force {
    let minimum_thrust_gain = Force::new::<poundal>(0.);
    let thrust_gain_rate = MassRate::new::<pound_per_second>(1.) / Time::new::<second>(24.);
    let low_altitude_ceiling = PressureAltitude::new::<foot>(7000.);

    if pressure_altitude > low_altitude_ceiling {
        return minimum_thrust_gain;
    }

    let altitude_reduction: Length = low_altitude_ceiling - pressure_altitude;
    let low_altitude_thrust_gain: Force = altitude_reduction * thrust_gain_rate;

    clamp_min(low_altitude_thrust_gain, minimum_thrust_gain)
}

fn calculate_high_altitude_thrust_loss(pressure_altitude: PressureAltitude) -> Force {
    let minimum_thrust_loss = Force::new::<poundal>(0.);
    let maximum_thrust_loss = Force::new::<poundal>(110.);
    let thrust_loss_rate = MassRate::new::<pound_per_second>(1.) / Time::new::<second>(64.);
    let high_altitude_floor = PressureAltitude::new::<foot>(35000.);

    if pressure_altitude < high_altitude_floor {
        return minimum_thrust_loss;
    }

    let altitude_reduction: Length = pressure_altitude - high_altitude_floor;
    let high_altitude_thrust_loss: Force = altitude_reduction * thrust_loss_rate;

    clamp(
        high_altitude_thrust_loss,
        minimum_thrust_loss,
        maximum_thrust_loss,
    )
}

fn convert_to_gross_thrust(thrust_in: Force, mach_in: Ratio) -> Force {
    thrust_in * (1. + (mach_in.get::<ratio>().powi(2) / 5.)).powf(3.5)
}

fn get_max_density_thrust(ambient_density: MassDensity) -> Force {
    let density_factor = Volume::new::<cubic_foot>(42_009.0345696695)
        * Acceleration::new::<foot_per_second_squared>(1.);
    let f: Force = ambient_density * density_factor;
    f + Force::new::<poundal>(250.)
}

struct ClimbFadecPidConfiguration;

impl ClimbFadecPidConfiguration {
    #[inline]
    fn default() -> PidConfiguration<Force> {
        PidConfiguration {
            gain_proportion: Ratio::new::<percent>(1.2) / Force::new::<poundal>(1_000.),
            gain_integral: Ratio::new::<percent>(0.0001)
                / Momentum::new::<pound_foot_per_second>(1.),
            gain_derivative: Time::new::<second>(0.018) / Force::new::<poundal>(1_000.),
            output_range: (Ratio::new::<percent>(-2.), Ratio::new::<percent>(2.)),
            derivative_range: (Ratio::new::<percent>(-20.), Ratio::new::<percent>(20.)),
            tolerance: Force::new::<poundal>(0.),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uom::si::mass_density::slug_per_cubic_foot;
    use wt_systems::testing;

    #[test]
    fn t_get_max_density_thrust() {
        let input = MassDensity::new::<slug_per_cubic_foot>(0.00241899350658059);

        //0.03108096668

        let expected = 0.00241899350658059 * 1000. * 1351.6 + 250.;
        let actual = get_max_density_thrust(input).get::<poundal>();

        testing::assert_equal_in_significant_figures(expected, actual, 12)
    }

    #[test]
    fn t_get_max_density_thrust_2() {
        let input = MassDensity::new::<slug_per_cubic_foot>(0.00141899350658059);

        let expected: f64 = 0.00141899350658059 * 1000. * 1351.6 + 250.;
        let actual = get_max_density_thrust(input).get::<poundal>();

        testing::assert_equal_in_significant_figures(expected, actual, 12)
    }

    testing::pid::step_tests! {
        name: basic_test,
        config: ClimbFadecPidConfiguration::default(),
        initial: PidController::default(),
        steps: [
            {
                inputs: (Force::new::<poundal>(200.), Time::new::<second>(0.0166666666666666)),
                expect: (Ratio::new::<percent>(2.), Momentum::new::<pound_foot_per_second>(4.9999999999999805))
            },
            {
                inputs: (Force::new::<poundal>(180.), Time::new::<second>(0.0166666666666666)),
                expect: (Ratio::new::<percent>(-1.9432166666666753), Momentum::new::<pound_foot_per_second>(7.833333333333302))
            },
            {
                inputs: (Force::new::<poundal>(20.), Time::new::<second>(0.0466666666666666)),
                expect: (Ratio::new::<percent>(-2.0), Momentum::new::<pound_foot_per_second>(5.033333333333306))
            },
            {
                inputs: (Force::new::<poundal>(50.), Time::new::<second>(0.0166666666666666)),
                expect: (Ratio::new::<percent>(2.0), Momentum::new::<pound_foot_per_second>(6.116666666666635))
            },
            {
                inputs: (Force::new::<poundal>(90.), Time::new::<second>(0.0136666666666666)),
                expect: (Ratio::new::<percent>(2.0), Momentum::new::<pound_foot_per_second>(7.619999999999961))
            },
            {
                inputs: (Force::new::<poundal>(-100.), Time::new::<second>(0.0166666666666666)),
                expect: (Ratio::new::<percent>(-2.0), Momentum::new::<pound_foot_per_second>(0.0))
            },
            {
                inputs: (Force::new::<poundal>(-10.), Time::new::<second>(0.0166666666666666)),
                expect: (Ratio::new::<percent>(2.0), Momentum::new::<pound_foot_per_second>(0.583333333333331))
            },
            {
                inputs: (Force::new::<poundal>(-9.), Time::new::<second>(0.0166666666666666)),
                expect: (Ratio::new::<percent>(0.0972441666666671), Momentum::new::<pound_foot_per_second>(0.44166666666666493))
            },
            {
                inputs: (Force::new::<poundal>(-3.), Time::new::<second>(0.0166666666666666)),
                expect: (Ratio::new::<percent>(0.6444441666666693), Momentum::new::<pound_foot_per_second>(0.44166666666666493))
            },
            {
                inputs: (Force::new::<poundal>(-1.), Time::new::<second>(0.0166666666666666)),
                expect: (Ratio::new::<percent>(0.21484416666666753), Momentum::new::<pound_foot_per_second>(0.44166666666666493))
            },
            {
                inputs: (Force::new::<poundal>(0.5), Time::new::<second>(1.)),
                expect: (Ratio::new::<percent>(0.0033), Momentum::new::<pound_foot_per_second>(0.0))
            },
        ],
        tolerances: {
            output: Ratio::new::<percent>(0.001),
            retained_error: Momentum::new::<pound_foot_per_second>(0.00001),
        },
    }
}
