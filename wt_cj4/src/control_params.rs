
use std::fmt;
use uom::si::{
    f64::*,
    force::poundal,
    ratio::{percent, ratio},
};
use uom::num_traits::clamp;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ThrottleMode {
    Undefined,
    Cruise,
    Climb,
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

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ThrottleAxis(pub f64);

impl ThrottleAxis {
    const MIN_VALUE: f64 = -16384.;
    const MAX_VALUE: f64 = 16384.;
    const THRUST_STEP: f64 = 256.;
    const RANGE: f64 = Self::MAX_VALUE - Self::MIN_VALUE;

    const UNDEF_MAX_VALUE: f64 = -15250.;
    const CRUISE_MAX_VALUE: f64 = 9060.; //Visually, 6360. looks better as the boundary here.
    const CRUISE_RANGE: f64 = Self::CRUISE_MAX_VALUE - Self::MIN_VALUE;
    const CLIMB_MAX_VALUE: f64 = 15000.;

    pub const MIN: Self = Self(Self::MIN_VALUE);
    pub const MAX: Self = Self(Self::MAX_VALUE);
    pub const UNDEF_MAX: Self = Self(Self::UNDEF_MAX_VALUE);
    pub const CRUISE_MAX: Self = Self(Self::CRUISE_MAX_VALUE);
    pub const CLIMB_MAX: Self = Self(Self::CLIMB_MAX_VALUE);
    pub const CLIMB: Self = Self((Self::CLIMB_MAX_VALUE - Self::CRUISE_MAX_VALUE) / 2. + Self::CRUISE_MAX_VALUE);
    pub const TAKEOFF: Self = Self::MAX;

    pub fn from_raw(value: f64) -> Self {
        Self(value).clamp()
    }

    pub fn from_raw_i32(value: i32) -> Self {
        Self::from_raw(value as f64)
    }

    pub fn from_raw_u32(value: u32) -> Self {
        Self::from_raw((value as f64) * 2. + ThrottleAxis::MIN_VALUE)
    }

    pub fn inc(self) -> Self {
        Self(self.0 + Self::THRUST_STEP).clamp()
    }

    pub fn dec(self) -> Self {
        Self(self.0 + Self::THRUST_STEP).clamp()
    }

    pub fn clamp(self) -> Self {
        Self(clamp(self.0, Self::MIN_VALUE, Self::MAX_VALUE))
    }

    pub fn to_ratio(self) -> Ratio {
        Ratio::new::<ratio>((self.0 - Self::MIN_VALUE) / Self::RANGE)
    }

    pub fn normalize_cruise(self) -> f64 {
        (self.0 - Self::MIN_VALUE) / Self::CRUISE_RANGE
    }

    pub fn from_ratio(value: Ratio) -> Self {
        Self(value.get::<ratio>() * Self::MAX_VALUE - Self::MIN_VALUE).clamp()
    }
}

impl fmt::Display for ThrottleAxis {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:.1}", self.0)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(transparent)]
pub struct ThrustValue(pub f64);

impl ThrustValue {
    const MIN_VALUE: f64 = 0.;
    const MAX_VALUE: f64 = 3600.;
    const RANGE: f64 = Self::MAX_VALUE - Self::MIN_VALUE;

    pub const MIN: Self = Self(Self::MIN_VALUE);
    pub const MAX: Self = Self(Self::MAX_VALUE);

    pub fn from_force(value: Force) -> Self {
        Self(value.get::<poundal>()).clamp()
    }

    pub fn from_ratio(value: Ratio) -> Self {
        Self(value.get::<ratio>() * Self::MAX_VALUE - Self::MIN_VALUE).clamp()
    }

    pub fn to_ratio(self) -> Ratio {
        Ratio::new::<ratio>((self.0 - Self::MIN_VALUE) / Self::RANGE)
    }

    pub fn clamp(self) -> Self {
        Self(clamp(self.0, Self::MIN_VALUE, Self::MAX_VALUE))
    }
}

impl fmt::Display for ThrustValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:.3} pdl", self.0)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(transparent)]
pub struct ThrottlePercent(pub f64);

impl ThrottlePercent {
    const MIN_VALUE: f64 = 0.;
    const MAX_VALUE: f64 = 100.;

    pub const MIN: Self = Self(Self::MIN_VALUE);
    pub const MAX: Self = Self(Self::MAX_VALUE);

    pub fn from_ratio(value: Ratio) -> Self {
        Self(value.get::<percent>()).clamp()
    }

    pub fn to_ratio(self) -> Ratio {
        Ratio::new::<percent>(self.0)
    }

    pub fn clamp(self) -> Self {
        Self(clamp(self.0, Self::MIN_VALUE, Self::MAX_VALUE))
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
