//! A PID implementation that removes the integral component on error sign changes

use super::{Derivative, ErrorRate, Integral, PidComponents, Proportion, RetainedError};
use serde::{Deserialize, Serialize};
use std::{fmt, ops};
use uom::num_traits::{clamp, zero, Zero};
use uom::si::f64::*;

/// Configuration for a PID controller
///
/// # Example
///
/// Tuning a PID controler is a non trivial task. The values specified below
/// are only for demonstration and are not assured to be convergent of stable.
///
/// ```
/// use wt_systems::pid::integral_zeroing::PidConfiguration;
/// use uom::si::f64::{Velocity, Ratio, Time};
/// use uom::si::velocity::meter_per_second;
/// use uom::si::ratio::{basis_point, ratio};
/// use uom::si::time::second;
///
/// let config = PidConfiguration {
///     gain_proportion: Ratio::new::<basis_point>(1.) / Velocity::new::<meter_per_second>(10.),
///     gain_integral: Ratio::new::<basis_point>(10.) / (Velocity::new::<meter_per_second>(3.) * Time::new::<second>(1.)),
///     gain_derivative: Time::new::<second>(1.0) / Velocity::new::<meter_per_second>(0.2),
///     output_range: (Ratio::new::<ratio>(-1.), Ratio::new::<ratio>(1.)),
///     derivative_range: (Ratio::new::<ratio>(-3.), Ratio::new::<ratio>(3.)),
///     tolerance: Velocity::new::<meter_per_second>(0.5),
/// };
/// ```
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "serde",
    serde(bound(
        serialize = "In: Serialize, Proportion<Ratio, In>: Serialize, Integral<Ratio, In, Time>: Serialize, Derivative<Time, In>: Serialize",
        deserialize = "for<'d> In: Deserialize<'d>, for<'d> Proportion<Ratio, In>: Deserialize<'d>, for<'d> Integral<Ratio, In, Time>: Deserialize<'d>, for<'d> Derivative<Time, In>: Deserialize<'d>",
    ))
)]
pub struct PidConfiguration<In>
where
    Ratio: ops::Div<In> + ops::Div<RetainedError<Time, In>>,
    Time: ops::Mul<In> + ops::Div<In>,
{
    /// The gain applied to the proportional component of error
    ///
    /// This is the primary coefficient that attempts to correct for the
    /// presence of an error.
    pub gain_proportion: Proportion<Ratio, In>,

    /// The gain applied to the integral component of error
    ///
    /// This is the _momentum_ that the PID gains over the course of continued
    /// errors over time.
    pub gain_integral: Integral<Ratio, In, Time>,

    /// The gain applied to the derivative component of error
    pub gain_derivative: Derivative<Time, In>,

    /// Output value limits (inclusive)
    ///
    /// Outputs from the PID controller will be clamped to range specified.
    pub output_range: (Ratio, Ratio),

    /// Derivative contribution limits (inclusive)
    ///
    /// Contributions to the output value from the derivative component will be
    /// clamped to the range specified.
    pub derivative_range: (Ratio, Ratio),

    /// Tolerance for deviations from the target value.
    ///
    /// When a value is within `tolerance` of the target value, the PID will
    /// deactivate and will not command any change in the output value and will
    /// slough off any remaining momentum from the integral component.
    ///
    /// When the deviation from the target value next exceeds the tolerance,
    /// the PID will again reactivate and command corrections.
    pub tolerance: In,
}

impl<In> Clone for PidConfiguration<In>
where
    In: Clone,
    Ratio: ops::Div<In> + ops::Div<RetainedError<Time, In>> + Clone,
    Time: ops::Mul<In> + ops::Div<In>,
    Proportion<Ratio, In>: Clone,
    Integral<Ratio, In, Time>: Clone,
    Derivative<Time, In>: Clone,
{
    #[inline(always)]
    fn clone(&self) -> Self {
        Self {
            gain_proportion: self.gain_proportion.clone(),
            gain_integral: self.gain_integral.clone(),
            gain_derivative: self.gain_derivative.clone(),
            output_range: self.output_range,
            derivative_range: self.derivative_range,
            tolerance: self.tolerance.clone(),
        }
    }
}

impl<In> Copy for PidConfiguration<In>
where
    In: Copy,
    Ratio: ops::Div<In> + ops::Div<RetainedError<Time, In>> + Copy,
    Time: ops::Mul<In> + ops::Div<In>,
    Proportion<Ratio, In>: Copy,
    Integral<Ratio, In, Time>: Copy,
    Derivative<Time, In>: Copy,
{
}

impl<In> PartialEq for PidConfiguration<In>
where
    In: PartialEq,
    Ratio: ops::Div<In> + ops::Div<RetainedError<Time, In>>,
    Time: ops::Mul<In> + ops::Div<In>,
    Proportion<Ratio, In>: PartialEq,
    Integral<Ratio, In, Time>: PartialEq,
    Derivative<Time, In>: PartialEq,
{
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.gain_derivative == other.gain_derivative
            && self.gain_integral == other.gain_integral
            && self.gain_proportion == other.gain_proportion
            && self.output_range == other.output_range
            && self.derivative_range == other.derivative_range
    }
}

impl<In> fmt::Debug for PidConfiguration<In>
where
    In: fmt::Debug,
    Ratio: ops::Div<In> + ops::Div<RetainedError<Time, In>> + fmt::Debug,
    Time: ops::Mul<In> + ops::Div<In>,
    Proportion<Ratio, In>: fmt::Debug,
    Integral<Ratio, In, Time>: fmt::Debug,
    Derivative<Time, In>: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("PidConfiguration")
            .field("gain_proportion", &self.gain_proportion)
            .field("gain_integral", &self.gain_integral)
            .field("gain_derivative", &self.gain_derivative)
            .field(
                "output_range",
                &format_args!("[{:?}, {:?}]", &self.output_range.0, &self.output_range.1),
            )
            .field(
                "derivative_range",
                &format_args!(
                    "[{:?}, {:?}]",
                    &self.derivative_range.0, &self.derivative_range.1
                ),
            )
            .finish()
    }
}

impl<In> super::Configuration for PidConfiguration<In>
where
    Ratio: ops::Div<In> + ops::Div<RetainedError<Time, In>>,
    Time: ops::Mul<In> + ops::Div<In>,
{
    #[inline(always)]
    fn clamp_output(&self, output: Ratio) -> Ratio {
        clamp(output, self.output_range.0, self.output_range.1)
    }
}

/// The PID controller
///
/// # Example
///
/// Tuning a PID controler is a non trivial task. The values specified below
/// are only for demonstration and are not assured to be convergent of stable.
///
/// ```
/// use wt_systems::pid::{Pid, integral_zeroing::{PidConfiguration, PidController}};
/// use uom::si::f64::{Velocity, Ratio, Time};
/// use uom::si::velocity::meter_per_second;
/// use uom::si::ratio::{basis_point, ratio};
/// use uom::si::time::second;
///
/// let config = PidConfiguration {
///     gain_proportion: Ratio::new::<basis_point>(1.) / Velocity::new::<meter_per_second>(10.),
///     gain_integral: Ratio::new::<basis_point>(10.) / (Velocity::new::<meter_per_second>(3.) * Time::new::<second>(1.)),
///     gain_derivative: Time::new::<second>(1.0) / Velocity::new::<meter_per_second>(0.2),
///     output_range: (Ratio::new::<ratio>(-1.), Ratio::new::<ratio>(1.)),
///     derivative_range: (Ratio::new::<ratio>(-3.), Ratio::new::<ratio>(3.)),
///     tolerance: Velocity::new::<meter_per_second>(0.5),
/// };
///
/// let mut pid = PidController::default();
///
/// let result = pid.step(
///     Velocity::new::<meter_per_second>(5000.),
///     &config,
///     Velocity::new::<meter_per_second>(4500.),
///     Time::new::<second>(5.),
/// );
/// ```
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "serde",
    serde(bound(
        serialize = "In: Serialize, RetainedError<Time, In>: Serialize",
        deserialize = "for<'d> In: Deserialize<'d>, for<'d> RetainedError<Time, In>: Deserialize<'d>",
    ))
)]
pub struct PidController<In>
where
    Ratio: ops::Div<In> + ops::Div<RetainedError<Time, In>>,
    Time: ops::Mul<In> + ops::Div<In>,
{
    /// Error identified during the last step
    pub prior_error: In,

    /// Retained error (momentum) due to accumulated errors over time
    pub retained_error: RetainedError<Time, In>,
}

impl<In> Clone for PidController<In>
where
    In: Clone,
    Ratio: ops::Div<In> + ops::Div<RetainedError<Time, In>>,
    Time: ops::Mul<In> + ops::Div<In>,
    RetainedError<Time, In>: Clone,
{
    #[inline(always)]
    fn clone(&self) -> Self {
        Self {
            prior_error: self.prior_error.clone(),
            retained_error: self.retained_error.clone(),
        }
    }
}

impl<In> Copy for PidController<In>
where
    In: Copy,
    Ratio: ops::Div<In> + ops::Div<RetainedError<Time, In>>,
    Time: ops::Mul<In> + ops::Div<In>,
    RetainedError<Time, In>: Copy,
{
}

impl<In> fmt::Debug for PidController<In>
where
    In: fmt::Debug,
    Ratio: ops::Div<In> + ops::Div<RetainedError<Time, In>>,
    Time: ops::Mul<In> + ops::Div<In>,
    RetainedError<Time, In>: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("PidController")
            .field("prior_error", &self.prior_error)
            .field("retained_error", &self.retained_error)
            .finish()
    }
}

impl<In> Default for PidController<In>
where
    In: Zero,
    Ratio: ops::Div<In> + ops::Div<RetainedError<Time, In>>,
    Time: ops::Mul<In> + ops::Div<In>,
    RetainedError<Time, In>: Zero,
{
    #[inline]
    fn default() -> Self {
        Self {
            prior_error: zero(),
            retained_error: zero(),
        }
    }
}

impl<In> PartialEq for PidController<In>
where
    In: PartialEq,
    Ratio: ops::Div<In> + ops::Div<RetainedError<Time, In>>,
    Time: ops::Mul<In> + ops::Div<In>,
    RetainedError<Time, In>: PartialEq,
{
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.prior_error == other.prior_error && self.retained_error == other.retained_error
    }
}

impl<In> PidController<In>
where
    Ratio: ops::Div<In> + ops::Div<RetainedError<Time, In>>,
    Time: ops::Mul<In> + ops::Div<In>,
{
    /// Constructs a PID controller with existing values
    #[inline]
    pub fn with_initial(prior_error: In, retained_error: RetainedError<Time, In>) -> Self {
        Self {
            prior_error,
            retained_error,
        }
    }

    /// Obtains a reference to the current prior error value
    #[inline]
    pub fn prior_error_ref(&self) -> &In {
        &self.prior_error
    }

    /// Obtains a reference to the current retained error value
    #[inline]
    pub fn retained_error_ref(&self) -> &RetainedError<Time, In> {
        &self.retained_error
    }
}

impl<In> PidController<In>
where
    In: Clone,
    Ratio: ops::Div<In> + ops::Div<RetainedError<Time, In>>,
    Time: ops::Mul<In> + ops::Div<In>,
{
    /// Gets the current prior error value
    #[inline]
    pub fn prior_error(&self) -> In {
        self.prior_error.clone()
    }
}

impl<In> PidController<In>
where
    Ratio: ops::Div<In> + ops::Div<RetainedError<Time, In>>,
    Time: ops::Mul<In> + ops::Div<In>,
    RetainedError<Time, In>: Clone,
{
    /// Gets the current retained error value
    #[inline]
    pub fn retained_error(&self) -> RetainedError<Time, In> {
        self.retained_error.clone()
    }
}

impl<In> PidController<In>
where
    Ratio: ops::Div<In> + ops::Div<RetainedError<Time, In>>,
    Time: ops::Mul<In> + ops::Div<In>,
    Self: Default,
{
    /// Resets the PID controller to a zeroed state
    ///
    /// If the PID controller was initialized with initial values, reset
    /// _will not_ use them for reset.
    #[inline]
    pub fn reset(&mut self) {
        *self = Self::default();
    }
}

impl<In> super::Pid<In> for PidController<In>
where
    In: PartialOrd + Zero + ops::Neg<Output = In> + ops::Sub<Output = In> + ops::Div<Time> + Copy,
    Ratio: Zero + PartialOrd + ops::Div<In> + ops::Div<RetainedError<Time, In>> + Copy,
    Proportion<Ratio, In>: ops::Mul<In, Output = Ratio> + Copy,
    Integral<Ratio, In, Time>: Copy,
    Time: ops::Mul<In> + ops::Div<In> + Copy,
    Derivative<Time, In>: ops::Mul<ErrorRate<In, Time>, Output = Ratio> + Copy,
    RetainedError<Time, In>: Zero
        + ops::Div<f64, Output = RetainedError<Time, In>>
        + ops::Mul<Integral<Ratio, In, Time>, Output = Ratio>
        + Copy,
    ErrorRate<In, Time>: ops::Mul<ErrorRate<In, Time>>,
{
    type Configuration = PidConfiguration<In>;

    fn step_with_components(
        &mut self,
        error: In,
        config: &Self::Configuration,
        _plant_value: In,
        delta_t: Time,
    ) -> PidComponents {
        // Proportional
        let proportional: Ratio = config.gain_proportion * error;

        // Integral
        // If the new error has changed signs, remove momentum
        let retained_error: RetainedError<Time, In> = if (error > zero())
            != (self.prior_error >= zero())
        {
            zero()
        } else {
            self.retained_error + (delta_t * error) + (delta_t * (error - self.prior_error) / 2.)
        };
        let integral: Ratio = retained_error * config.gain_integral;

        // Derivative
        let error_over_time: ErrorRate<In, Time> = (error - self.prior_error) / delta_t;
        let raw_gained_derivative: Ratio = config.gain_derivative * error_over_time;
        let derivative: Ratio = clamp(
            raw_gained_derivative,
            config.derivative_range.0,
            config.derivative_range.1,
        );

        // println!("Output: {} ({}): Derivative: {} ({}), Integral: {}, proportion: {}", output.into_format_args(ratio, uom::fmt::DisplayStyle::Abbreviation), raw_output.into_format_args(ratio, uom::fmt::DisplayStyle::Abbreviation), gained_derivative.into_format_args(ratio, uom::fmt::DisplayStyle::Abbreviation), raw_gained_derivative.into_format_args(ratio, uom::fmt::DisplayStyle::Abbreviation), gained_integral.into_format_args(ratio, uom::fmt::DisplayStyle::Abbreviation), gained_error.into_format_args(ratio, uom::fmt::DisplayStyle::Abbreviation));

        self.prior_error = error;
        self.retained_error = retained_error;
        PidComponents {
            proportional,
            integral,
            derivative,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pid::Pid;
    use uom::si::f64::{Ratio, Time, Velocity};
    use uom::si::ratio::ratio;
    use uom::si::time::second;
    use uom::si::velocity::meter_per_second;

    #[test]
    fn test_valid_with_uom_1() {
        let config = PidConfiguration {
            gain_proportion: Ratio::new::<ratio>(1.) / Velocity::new::<meter_per_second>(10.),
            gain_integral: Ratio::new::<ratio>(10.)
                / (Velocity::new::<meter_per_second>(3.) * Time::new::<second>(1.)),
            gain_derivative: Time::new::<second>(1.0) / Velocity::new::<meter_per_second>(0.2),
            output_range: (Ratio::new::<ratio>(-1_000.), Ratio::new::<ratio>(1_000.)),
            derivative_range: (Ratio::new::<ratio>(-1_000.), Ratio::new::<ratio>(1_000.)),
            tolerance: Velocity::new::<meter_per_second>(0.5),
        };

        let mut pid = PidController::default();

        let _output = pid.step(
            Velocity::new::<meter_per_second>(500.),
            &config,
            Velocity::new::<meter_per_second>(450.),
            Time::new::<second>(5.),
        );
    }
}

pub(crate) mod testing {
    /// Provides a way to easily set up a test of a PID controller with certain
    /// initial conditions, and verify the behavior at each intermediate step
    #[macro_export]
    macro_rules! pid_step_tests {
        {
            name: $name:ident,
            config: $config:expr,
            initial: $initial:expr,
            steps: [
                $({
                    inputs: ($set_point: expr, $plant_value:expr, $delta_t:expr),
                    expect: ($expected_output:expr, $expected_retained_error:expr)$(,)?
                }),*$(,)?
            ],
            tolerances: {
                output: $output_tolerance:expr,
                retained_error: $retained_error_tolerance:expr$(,)?
            }$(,)?
        } => {
            #[test]
            fn $name() {
                use ::uom::num_traits::Zero;
                use $crate::pid::{Pid, integral_zeroing::{PidController, PidConfiguration}};

                let config = $config;
                let mut state = $initial;
                println!("Initial:    {:?}", state);

                let mut step = 0;
                let mut failed = false;
                $(
                    #[allow(unused_assignments)]
                    {
                        step += 1;
                        let set_point = $set_point;
                        let plant_value = $plant_value;
                        let delta_t = $delta_t;
                        let error = set_point - plant_value;

                        println!("Step {:>2} Set point: {:?}; Plant Value: {:?}; Error: {:?}, Delta Time: {:?}", step, set_point, plant_value, error, delta_t);

                        let actual = state.step(error, &config, plant_value, $delta_t);
                        let expected = (
                            PidController::with_initial(error, $expected_retained_error),
                            $expected_output
                        );

                        let difference = (
                            PidController::with_initial(
                                expected.0.prior_error() - state.prior_error(),
                                expected.0.retained_error() - state.retained_error(),
                            ),
                            expected.1 - actual
                        );

                        println!("    Expected:   {:?}", expected);
                        println!("    Actual:     {:?}", (state, actual));
                        println!("    Difference: {:?}", difference);

                        #[allow(clippy::float_cmp)]
                        if state.prior_error() != expected.0.prior_error() && state.prior_error().is_zero() {
                            eprintln!(" !!! error mismatch !!!");
                            failed = true
                        }

                        if difference.0.retained_error() > $retained_error_tolerance || difference.0.retained_error() < -$retained_error_tolerance  {
                            eprintln!(" !!! retained error mismatch !!!");
                            failed = true;
                        }

                        if difference.1 > $output_tolerance || difference.1 < -$output_tolerance {
                            eprintln!(" !!! output mismatch !!!");
                            failed = true;
                        }
                    }
                )*

                if failed {
                    panic!("One of the test steps had a result outside of tolerances");
                }
            }
        };
    }
}
