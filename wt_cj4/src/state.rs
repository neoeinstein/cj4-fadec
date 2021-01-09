//! Aircraft state information

use crate::control_params::{ThrottleAxis, ThrottleMode, ThrottlePercent};
use crate::engines::EngineData;
use crate::FadecController;
use avmath::isa::{GeometricAltitude, PressureAltitude};

/// Environmental readings from general instrumentation
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Instruments {
    /// Aircraft speed represented as a percentage of the speed of sound
    pub mach_number: uom::si::f64::Ratio,

    /// Ambient density
    pub ambient_density: uom::si::f64::MassDensity,

    /// Geometric altitude above mean sea level
    pub geometric_altitude: GeometricAltitude,

    /// Pressure altitude
    pub pressure_altitude: PressureAltitude,

    /// Indicated airspeed
    pub airspeed_indicated: uom::si::f64::Velocity,

    /// True airspeed
    pub airspeed_true: uom::si::f64::Velocity,

    /// Vertical speed
    pub vertical_speed: uom::si::f64::Velocity,
}

/// Engine-specific readings
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EngineReadings {
    /// Thrust developed by the engines
    pub thrust: uom::si::f64::Force,
}

/// Overall inputs for the aircraft simulation
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Environment {
    /// Readings from instruments
    pub instruments: Instruments,

    /// Engine readings
    pub engines: EngineData<EngineReadings>,
}

/// Aircraft engine
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Engine {
    /// The current FADEC throttle mode
    pub mode: ThrottleMode,

    /// The throttle command sent to the engines
    pub engine_throttle: ThrottlePercent,

    /// The throttle position on the console
    pub visual_throttle: ThrottlePercent,

    /// Position of the throttle according to the input axis
    pub physical_throttle: ThrottleAxis,

    /// The FADEC controller
    pub fadec: FadecController,
}

/// The state of the entire aircraft
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Aircraft {
    /// Aircraft engines
    pub engines: EngineData<Engine>,
}

/// A snapshot of the aircraft simulation data
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Snapshot {
    /// The time of the snapshot
    ///
    /// Listed as time since the start of the program
    pub sim_time: uom::si::f64::Time,

    /// Change in time between steps
    pub delta_t: uom::si::f64::Time,

    /// Environmental input data
    pub environment: Environment,

    /// Aircraft state after applying all systems
    pub aircraft: Aircraft,
}
