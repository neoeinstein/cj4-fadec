//! PID implementation based on the the model described by Tim Wescott
//!
//! Follows the model described in the free [_PID Without a Ph. D._][Wes18]
//! authored by Tim Wescott.
//!
//!   [Wes18]: https://www.wescottdesign.com/articles/pid/pidWithoutAPhd.pdf

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
/// use wt_systems::pid::wescott::PidConfiguration;
/// use uom::si::f64::{Velocity, Ratio, Time};
/// use uom::si::velocity::meter_per_second;
/// use uom::si::ratio::{basis_point, ratio};
/// use uom::si::time::second;
///
/// let config = PidConfiguration::<Velocity> {
///     gain_proportion: Ratio::new::<basis_point>(1.) / Velocity::new::<meter_per_second>(10.),
///     gain_integral: Ratio::new::<basis_point>(10.) / (Velocity::new::<meter_per_second>(3.) * Time::new::<second>(1.)),
///     gain_derivative: Time::new::<second>(1.0) / Velocity::new::<meter_per_second>(0.2),
///     output_range: (Ratio::new::<ratio>(-1.), Ratio::new::<ratio>(1.)),
///     integral_range: (
///         Velocity::new::<meter_per_second>(-30.) * Time::new::<second>(1.),
///         Velocity::new::<meter_per_second>(30.) * Time::new::<second>(1.)
///     ),
/// };
/// ```
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "serde",
    serde(bound(
        serialize = "In: Serialize, Proportion<Ratio, In>: Serialize, Integral<Ratio, In, Time>: Serialize, Derivative<Time, In>: Serialize, RetainedError<Time, In>: Serialize",
        deserialize = "for<'d> In: Deserialize<'d>, for<'d> Proportion<Ratio, In>: Deserialize<'d>, for<'d> Integral<Ratio, In, Time>: Deserialize<'d>, for<'d> Derivative<Time, In>: Deserialize<'d>, for<'d> RetainedError<Time, In>: Deserialize<'d>",
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
    pub integral_range: (RetainedError<Time, In>, RetainedError<Time, In>),
}

impl<In> Clone for PidConfiguration<In>
where
    In: Clone,
    Ratio: ops::Div<In> + ops::Div<RetainedError<Time, In>> + Clone,
    Time: ops::Mul<In> + ops::Div<In>,
    Proportion<Ratio, In>: Clone,
    Integral<Ratio, In, Time>: Clone,
    Derivative<Time, In>: Clone,
    RetainedError<Time, In>: Clone,
{
    #[inline(always)]
    fn clone(&self) -> Self {
        Self {
            gain_proportion: self.gain_proportion.clone(),
            gain_integral: self.gain_integral.clone(),
            gain_derivative: self.gain_derivative.clone(),
            output_range: self.output_range,
            integral_range: self.integral_range.clone(),
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
    RetainedError<Time, In>: Copy,
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
    RetainedError<Time, In>: PartialEq,
{
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.gain_derivative == other.gain_derivative
            && self.gain_integral == other.gain_integral
            && self.gain_proportion == other.gain_proportion
            && self.output_range == other.output_range
            && self.integral_range == other.integral_range
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
    RetainedError<Time, In>: fmt::Debug,
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
                "integral_range",
                &format_args!(
                    "[{:?}, {:?}]",
                    &self.integral_range.0, &self.integral_range.1
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
/// Follows the model described in the free [_PID Without a Ph. D._][Wes18]
/// authored by Tim Wescott.
///
///   [Wes18]: https://www.wescottdesign.com/articles/pid/pidWithoutAPhd.pdf
///
/// ```
/// use wt_systems::pid::{Pid, wescott::{PidConfiguration, PidController}};
/// use uom::si::f64::{Velocity, Ratio, Time};
/// use uom::si::velocity::{meter_per_second};
/// use uom::si::ratio::{basis_point, ratio};
/// use uom::si::time::second;
///
/// let config = PidConfiguration {
///     gain_proportion: Ratio::new::<basis_point>(1.) / Velocity::new::<meter_per_second>(10.),
///     gain_integral: Ratio::new::<basis_point>(10.) / (Velocity::new::<meter_per_second>(3.) * Time::new::<second>(1.)),
///     gain_derivative: Time::new::<second>(1.0) / Velocity::new::<meter_per_second>(0.2),
///     output_range: (Ratio::new::<ratio>(-1.), Ratio::new::<ratio>(1.)),
///     integral_range: (
///         Velocity::new::<meter_per_second>(-30.) * Time::new::<second>(1.),
///         Velocity::new::<meter_per_second>(30.) * Time::new::<second>(1.)
///     ),
/// };
///
/// let mut pid = PidController::default();
///
/// let result = pid.step(
///     Velocity::new::<meter_per_second>(500.),
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
    /// Plant value from the last step
    pub prior_plant_value: In,

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
            prior_plant_value: self.prior_plant_value.clone(),
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
            .field("prior_plant_value", &self.prior_plant_value)
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
            prior_plant_value: zero(),
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
        self.prior_plant_value == other.prior_plant_value
            && self.retained_error == other.retained_error
    }
}

impl<In> PidController<In>
where
    Ratio: ops::Div<In> + ops::Div<RetainedError<Time, In>>,
    Time: ops::Mul<In> + ops::Div<In>,
{
    /// Constructs a PID controller with existing values
    #[inline]
    pub fn with_initial(initial_plant_value: In, retained_error: RetainedError<Time, In>) -> Self {
        Self {
            prior_plant_value: initial_plant_value,
            retained_error,
        }
    }

    /// Obtains a reference to the plant value from the previous step
    #[inline]
    pub fn prior_plant_value_ref(&self) -> &In {
        &self.prior_plant_value
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
    /// Gets the current plant value from the previous step
    #[inline]
    pub fn prior_plant_value(&self) -> In {
        self.prior_plant_value.clone()
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
    In: PartialOrd
        + Zero
        + ops::Sub<In, Output = In>
        + ops::Div<Time>
        + ops::Mul<Time, Output = RetainedError<Time, In>>
        + Copy,
    Ratio: ops::Div<In> + ops::Div<RetainedError<Time, In>>,
    Time: ops::Mul<In> + ops::Div<In>,
    Proportion<Ratio, In>: ops::Mul<In, Output = Ratio> + Copy,
    Integral<Ratio, In, Time>: ops::Mul<RetainedError<Time, In>, Output = Ratio> + Copy,
    Derivative<Time, In>: ops::Mul<ErrorRate<In, Time>, Output = Ratio> + Copy,
    RetainedError<Time, In>: ops::AddAssign + PartialOrd + Copy,
{
    type Configuration = PidConfiguration<In>;

    fn step_with_components(
        &mut self,
        error: In,
        config: &Self::Configuration,
        plant_value: In,
        delta_t: Time,
    ) -> PidComponents {
        // Proportional
        let proportional: Ratio = config.gain_proportion * error;

        // Integral
        self.retained_error += error * delta_t;
        self.retained_error = clamp(
            self.retained_error,
            config.integral_range.0,
            config.integral_range.1,
        );
        let integral: Ratio = config.gain_integral * self.retained_error;

        // Derivative
        let rate_of_change = (plant_value - self.prior_plant_value) / delta_t;
        let derivative: Ratio = config.gain_derivative * rate_of_change;

        self.prior_plant_value = plant_value;

        // println!("Output: {} ({}): Derivative: {} ({}), Integral: {}, proportion: {}", output.into_format_args(ratio, uom::fmt::DisplayStyle::Abbreviation), raw_output.into_format_args(ratio, uom::fmt::DisplayStyle::Abbreviation), gained_derivative.into_format_args(ratio, uom::fmt::DisplayStyle::Abbreviation), raw_gained_derivative.into_format_args(ratio, uom::fmt::DisplayStyle::Abbreviation), gained_integral.into_format_args(ratio, uom::fmt::DisplayStyle::Abbreviation), gained_error.into_format_args(ratio, uom::fmt::DisplayStyle::Abbreviation));

        PidComponents {
            proportional,
            integral,
            derivative,
        }
    }
}
