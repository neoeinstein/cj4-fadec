//! Proportional-Integral-Derivative controller implementations

use std::ops;
use uom::si::f64::{Ratio, Time};

pub mod integral_zeroing;
pub mod wescott;

/// Over * In
pub type RetainedError<Over, In> = <Over as ops::Mul<In>>::Output;
/// Out / In
pub type Proportion<Out, In> = <Out as ops::Div<In>>::Output;
/// Out / (Over * In)
pub type Integral<Out, In, Over> = <Out as ops::Div<RetainedError<Over, In>>>::Output;
/// Over / In
pub type Derivative<Over, In> = <Over as ops::Div<In>>::Output;
/// In / Over
pub type ErrorRate<In, Over> = <In as ops::Div<Over>>::Output;

/// Outputs from a PID controller
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PidComponents {
    /// The proportional output from the PID
    ///
    /// The proportional term is calculated by multiplying the error signal by a
    /// constant value.
    pub proportional: Ratio,

    /// The integral output from the PID
    ///
    /// The integral term is calculated by accumulating the error signal over
    /// previous steps (retained error) and multiplying it by a constant value.
    pub integral: Ratio,

    /// The derivative output from the PID
    ///
    /// The derivative term is calculated by taking the rate of change in the
    /// system (either change of the plant value or change of the error signal)
    /// and multiplying it by a constant value.
    ///
    /// Using the derivative over the plant value rather than the error can
    /// provide for smoother transitions as the command value changes.
    pub derivative: Ratio,
}

/// Configuration for a PID controller
pub trait Configuration {
    /// Constrains the output command value
    #[inline(always)]
    fn clamp_output(&self, output: Ratio) -> Ratio {
        output
    }
}

/// A PID Controller
pub trait Pid<In> {
    /// The configuration type required for this PID
    type Configuration: Configuration;

    /// Steps the PID controller forward in time with intermediate outputs
    ///
    /// There may be times where it is useful (perhaps due to a change in
    /// state of the system) to switch out the PID configuration. This can
    /// allow for switching to PIDs with different behavior that better match
    /// different phases.
    fn step_with_components(
        &mut self,
        error: In,
        config: &Self::Configuration,
        plant_value: In,
        delta_t: Time,
    ) -> PidComponents;

    /// Steps the PID controller forward in time
    ///
    /// There may be times where it is useful (perhaps due to a change in
    /// state of the system) to switch out the PID configuration. This can
    /// allow for switching to PIDs with different behavior that better match
    /// different phases.
    #[inline]
    fn step(
        &mut self,
        error: In,
        config: &Self::Configuration,
        plant_value: In,
        delta_t: Time,
    ) -> Ratio {
        let PidComponents {
            proportional,
            integral,
            derivative,
        } = self.step_with_components(error, config, plant_value, delta_t);

        config.clamp_output(proportional + integral + derivative)
    }
}
