//! Control parameters for managing the CJ4

use std::{fmt, ops};
use uom::si::{
    f64::*,
    force::poundal,
    ratio::{percent, ratio},
};
use uom::num_traits::clamp;

/// The FADEC throttle mode
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ThrottleMode {
    /// An engine at effectively idle state
    Undefined,
    
    /// Cruise mode
    Cruise,

    /// Climb mode 
    Climb,

    /// Takeoff mode
    Takeoff
}

impl Default for ThrottleMode {
    fn default() -> Self {
        Self::Undefined
    }
}

impl From<ThrottleMode> for f64 {
    fn from(m: ThrottleMode) -> Self {
        match m {
            ThrottleMode::Undefined => 0.,
            ThrottleMode::Cruise => 1.,
            ThrottleMode::Climb => 2.,
            ThrottleMode::Takeoff => 3.,
        }
    }
}

impl fmt::Display for ThrottleMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Self::Undefined => "UNDEF",
            Self::Cruise => "CRU",
            Self::Climb => "CLB",
            Self::Takeoff => "TO",
        };
        f.write_str(s)
    }
}

/// The position of the throttle axis
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ThrottleAxis(f64);

impl ThrottleAxis {
    const MIN_VALUE: f64 = -16384.;
    const MAX_VALUE: f64 = 16384.;
    const THRUST_STEP: f64 = 256.;
    const RANGE: f64 = Self::MAX_VALUE - Self::MIN_VALUE;

    const UNDEF_MAX_VALUE: f64 = -15250.;
    const CRUISE_MAX_VALUE: f64 = 9060.; //Visually, 6360. looks better as the boundary here.
    const CRUISE_RANGE: f64 = Self::CRUISE_MAX_VALUE - Self::MIN_VALUE;
    const CLIMB_MAX_VALUE: f64 = 15000.;

    /// Minimum value
    pub const MIN: Self = Self(Self::MIN_VALUE);
    /// Maximum value
    pub const MAX: Self = Self(Self::MAX_VALUE);
    /// The top limit for the undefined range
    pub const UNDEF_MAX: Self = Self(Self::UNDEF_MAX_VALUE);
    /// The top limit for the cruise range
    pub const CRUISE_MAX: Self = Self(Self::CRUISE_MAX_VALUE);
    /// The top limit for the climb range
    pub const CLIMB_MAX: Self = Self(Self::CLIMB_MAX_VALUE);
    /// The throttle level value corresponding to the Climb detent
    pub const CLIMB: Self = Self((Self::CLIMB_MAX_VALUE - Self::CRUISE_MAX_VALUE) / 2. + Self::CRUISE_MAX_VALUE);
    /// The throttle level value corresponding to the Takeoff detent
    pub const TAKEOFF: Self = Self::MAX;

    /// Interprets a raw value as a throttle axis, saturating to the valid
    /// range
    pub fn from_raw(value: f64) -> Self {
        Self(value).clamp()
    }

    /// Interprets a raw signed integer as a throttle axis, saturating to the
    /// valid range
    pub fn from_raw_i32(value: i32) -> Self {
        Self::from_raw(value as f64).clamp()
    }

    /// Interprets a raw unsigned integer as a throttle axis, translating and
    /// saturating to the valid range
    pub fn from_raw_u32(value: u32) -> Self {
        Self::from_raw((value as f64) * 2. + ThrottleAxis::MIN_VALUE).clamp()
    }

    /// Increases the thrust axis by 1 / 128 of the full axis range
    pub fn inc(self) -> Self {
        Self(self.0 + Self::THRUST_STEP).clamp()
    }

    /// Decreases the thrust axis by 1 / 128 of the full axis range
    pub fn dec(self) -> Self {
        Self(self.0 + Self::THRUST_STEP).clamp()
    }

    /// Clamps the value to the valid range
    fn clamp(self) -> Self {
        Self(clamp(self.0, Self::MIN_VALUE, Self::MAX_VALUE))
    }

    /// Reinterprets the axis as a ratio between the minimum and maximum values
    pub fn to_ratio(self) -> Ratio {
        Ratio::new::<ratio>((self.0 - Self::MIN_VALUE) / Self::RANGE)
    }

    /// Reinterprets the axis as a ratio between the minimum and maximum values
    /// for cruise flight
    pub fn normalize_cruise(self) -> Ratio {
        Ratio::new::<ratio>((self.0 - Self::MIN_VALUE) / Self::CRUISE_RANGE)
    }

    /// Creates an axis value where the throttle is position between minimum and
    /// maximum is provided
    pub fn from_ratio(value: Ratio) -> Self {
        Self(value.get::<ratio>() * Self::RANGE + Self::MIN_VALUE).clamp()
    }
}

impl fmt::Display for ThrottleAxis {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:.1}", self.0)
    }
}

/// A thrust value for the CJ4 in poundals
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(transparent)]
pub struct ThrustValue(f64);

impl ThrustValue {
    const MIN_VALUE: f64 = 0.;
    const MAX_VALUE: f64 = 3600.;
    const RANGE: f64 = Self::MAX_VALUE - Self::MIN_VALUE;

    /// The minimun thrust value
    pub const MIN: Self = Self(Self::MIN_VALUE);
    /// The maximum rated thrust value
    pub const MAX: Self = Self(Self::MAX_VALUE);

    /// Reinterprets a force as engine thrust
    pub fn from_force(value: Force) -> Self {
        Self(value.get::<poundal>()).clamp()
    }

    /// Creates an engine thrust value equivalent to the ratio between
    /// the minimum and maximum rated thrust values
    pub fn from_ratio(value: Ratio) -> Self {
        Self(value.get::<ratio>() * Self::RANGE + Self::MIN_VALUE).clamp()
    }

    /// Reinterprets the engine thrust value as a ratio between the
    /// minimum and maxiumum rated thrust values
    pub fn to_ratio(self) -> Ratio {
        Ratio::new::<ratio>((self.0 - Self::MIN_VALUE) / Self::RANGE)
    }

    /// Clamps the value to valid rated values
    fn clamp(self) -> Self {
        Self(clamp(self.0, Self::MIN_VALUE, Self::MAX_VALUE))
    }
}

impl fmt::Display for ThrustValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:.3} pdl", self.0)
    }
}

/// A throttle position as a percentage of full
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(transparent)]
pub struct ThrottlePercent(f64);

impl ThrottlePercent {
    const MIN_VALUE: f64 = 0.;
    const MAX_VALUE: f64 = 100.;

    /// The throttle minimum position
    pub const MIN: Self = Self(Self::MIN_VALUE);
    /// The throttle full position
    pub const MAX: Self = Self(Self::MAX_VALUE);

    /// Creates a throttle percent a ratio between the minimum and full
    /// positions
    pub fn from_ratio(value: Ratio) -> Self {
        Self(value.get::<percent>()).clamp()
    }

    /// Reinterprets the throttle percentage as a ratio between the minimum
    /// and full positions
    pub fn to_ratio(self) -> Ratio {
        Ratio::new::<percent>(self.0)
    }

    /// Clamps the value to valid values
    fn clamp(self) -> Self {
        Self(clamp(self.0, Self::MIN_VALUE, Self::MAX_VALUE))
    }
}

impl ops::Add for ThrottlePercent {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0)
    }
}

impl From<ThrottleAxis> for ThrottlePercent {
    fn from(v: ThrottleAxis) -> Self {
        Self::from_ratio(v.to_ratio())
    }
}

impl From<ThrottlePercent> for ThrottleAxis {
    fn from(v: ThrottlePercent) -> Self {
        Self::from_ratio(v.to_ratio())
    }
}

impl From<ThrottlePercent> for f64 {
    fn from(pos: ThrottlePercent) -> Self {
        pos.0
    }
}

impl fmt::Display for ThrottlePercent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:.3} pct", self.0)
    }
}
