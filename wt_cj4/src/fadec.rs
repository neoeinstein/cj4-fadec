use crate::control_params::{ThrottleMode, ThrottlePercent, ThrottleAxis, ThrustValue};
use uom::fmt::DisplayStyle::Abbreviation;
use uom::si::{
    f64::*,
    acceleration::foot_per_second_squared,
    force::poundal,
    length::foot,
    mass_density::slug_per_cubic_foot,
    mass_rate::pound_per_second,
    momentum::pound_foot_per_second,
    ratio::{percent, ratio},
    time::second,
    volume::cubic_foot,
};
use avmath::PressureAltitude;
use wt_systems::{PidConfiguration, PidState};

#[derive(Debug)]
pub struct FadecController {
    pid_config: PidConfiguration<Force, Ratio>,
    pid_state: PidState<Force>,
    throttle_axis: ThrottleAxis,
    throttle_mode: ThrottleMode,
    enabled: bool,
}

impl Default for FadecController {
    fn default() -> Self {
        Self {
            pid_config: ClimbFadecPidConfiguration::default(),
            pid_state: PidState::default(),
            throttle_axis: ThrottleAxis::MIN,
            throttle_mode: ThrottleMode::Undefined,
            enabled: true,
        }
    }
}

impl FadecController {
    pub fn get_desired_throttle(&mut self, current_throttle: ThrottlePercent, engine_thrust: Force, mach_number: Ratio, ambient_density: MassDensity, altitude: PressureAltitude, delta_t: Time) -> (ThrustValue, ThrottlePercent) {
        let normalized_throttle = self.throttle_axis.to_ratio();

        if !self.enabled {
            let throttle_exp = Ratio::new::<ratio>(normalized_throttle.value.powf(3.5));
            return (ThrustValue::from_ratio(throttle_exp), ThrottlePercent::from_ratio(normalized_throttle))
        }

        let thrust_efficiency = Ratio::new::<ratio>(0.93);

        match self.throttle_mode {
            ThrottleMode::Takeoff => {
                (ThrustValue::MAX, ThrottlePercent::MAX)
            }
            ThrottleMode::Climb => {
                let gross_thrust = convert_to_gross_thrust(engine_thrust, mach_number);
                let max_density_thrust = get_max_density_thrust(ambient_density);
                let max_effective_thrust = max_density_thrust * thrust_efficiency;

                println!("Raw thrust: {:.3}, Airspeed: {:.3} M, Gross thrust: {:.3}, Ambient density: {:.4}, Max density thrust: {:.3}, altitude: {:.0}", engine_thrust.into_format_args(poundal, Abbreviation), mach_number.into_format_args(ratio, Abbreviation), gross_thrust.into_format_args(poundal, Abbreviation), ambient_density.into_format_args(slug_per_cubic_foot, Abbreviation), max_density_thrust.into_format_args(poundal, Abbreviation), altitude.remove_context().into_format_args(foot, Abbreviation));

                let low_altitude_limit = PressureAltitude::new::<foot>(7000.);
                let altitude_reduction: Length = low_altitude_limit - altitude;
                let low_altitude_thrust: Force =
                    (altitude_reduction * MassRate::new::<pound_per_second>(1.) / Time::new::<second>(24.))
                        .max(Force::new::<poundal>(0.));
                let low_thrust_target: Force = Force::new::<poundal>(2050.) + low_altitude_thrust;

                let target_thrust: Force = if max_effective_thrust < low_thrust_target {
                    let high_altitude_limit = PressureAltitude::new::<foot>(35000.);
                    let altitude_reduction: Length = altitude - high_altitude_limit;
                    let high_altitude_thrust_reduction: Force =
                        (altitude_reduction * MassRate::new::<pound_per_second>(1.) / Time::new::<second>(64.))
                            .max(Force::new::<poundal>(0.))
                            .min(Force::new::<poundal>(110.));

                    max_effective_thrust - high_altitude_thrust_reduction
                } else {
                    low_thrust_target
                };

                let output = self.pid_state.tick(target_thrust, &self.pid_config, gross_thrust, delta_t);

                let next_throttle = current_throttle.to_ratio() + output;
                println!("Target thrust: {:.4} (error: {:+.4}); adjusting throttle {:+.4} to {:.4} of maximum", target_thrust.into_format_args(poundal, Abbreviation), self.pid_state.prior_error().into_format_args(poundal, Abbreviation), output.into_format_args(ratio, Abbreviation), next_throttle.into_format_args(ratio, Abbreviation));

                (ThrustValue::from_force(target_thrust), ThrottlePercent::from_ratio(next_throttle))
            }
            ThrottleMode::Cruise | ThrottleMode::Undefined => {
                let cruise_normalized_throttle = self.throttle_axis.normalize_cruise();
                let cruise_throttle_exp = cruise_normalized_throttle;
                let effective_thrust = cruise_throttle_exp * thrust_efficiency;

                (ThrustValue::from_ratio(effective_thrust), ThrottlePercent::from_ratio(effective_thrust))
            }
        }
    }
}

fn convert_to_gross_thrust(thrust_in: Force, mach_in: Ratio) -> Force {
    thrust_in * (1. + (mach_in.get::<ratio>().powi(2) / 5.)).powf(3.5)
}

fn get_max_density_thrust(ambient_density: MassDensity) -> Force {
    let density_factor = Volume::new::<cubic_foot>(42_009.0345696695) * Acceleration::new::<foot_per_second_squared>(1.);
    let f: Force = ambient_density * density_factor;
    f + Force::new::<poundal>(250.)
}

struct ClimbFadecPidConfiguration;

impl ClimbFadecPidConfiguration {
    #[inline]
    fn default() -> PidConfiguration<Force, Ratio> {
        PidConfiguration {
            gain_proportion: Ratio::new::<percent>(1.2) / Force::new::<poundal>(1_000.),
            gain_integral: Ratio::new::<percent>(0.0001) / Momentum::new::<pound_foot_per_second>(1.),
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
    use uom::si::{
        mass_density::slug_per_cubic_foot,
    };
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

    testing::pid::tick_tests! {
        name: basic_test,
        config: ClimbFadecPidConfiguration::default(),
        initial: PidState::default(),
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
            integral: Momentum::new::<pound_foot_per_second>(0.00001),
        },
    }
}