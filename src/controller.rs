//use std::marker::PhantomData;
use std::fmt;
use gauge_sys::{IndexedAircraftVariable, UnindexedAircraftVariable, NamedVariable, gauge_unit, indexed_aircraft_variable, unindexed_aircraft_variable, named_variable};
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive, clamp};
use uom::si::{
    f64::*,
    volume::cubic_foot,
    force::poundal,
    momentum::pound_foot_per_second,
    ratio::{ratio, percent},
    time::second,
    length::foot,
    mass_rate::pound_per_second,
    mass_density::slug_per_cubic_foot,
    acceleration::foot_per_second_squared,
};
use super::pid::{PidConfiguration, PidState};

// fn speed_of_sound(altitude: Alt) -> Velocity {
//     let x = Altitude::new::<foot>(1.);
//     let t = Time::new::<second>(3.);
//     x / t
// }

gauge_unit!(Percent: "Percent");
gauge_unit!(Pounds: "Pounds");
gauge_unit!(Feet: "Feet");
gauge_unit!(FootPounds: "Foot pounds");
gauge_unit!(Number: "Number");
gauge_unit!(Mach: "Mach");
gauge_unit!(SluggerSlugs: "Slug per cubic feet");

indexed_aircraft_variable!(Throttle(Percent): "GENERAL ENG THROTTLE LEVER POSITION");
indexed_aircraft_variable!(Thrust(Pounds): "TURB ENG JET THRUST");
unindexed_aircraft_variable!(AirspeedMach(Mach): "AIRSPEED MACH");
unindexed_aircraft_variable!(Altitude(Feet): "PLANE ALTITUDE");
unindexed_aircraft_variable!(AmbientDensity(SluggerSlugs): "AMBIENT DENSITY");
unindexed_aircraft_variable!(OnGround(Number): "SIM ON GROUND");

impl Throttle {
    fn read_index_typed(index: u32) -> Ratio {
        Ratio::new::<percent>(Self::read_index(index))
    }
}

impl Thrust {
    fn read_index_typed(index: u32) -> Force {
        Force::new::<poundal>(Self::read_index(index))
    }
}

impl Altitude {
    fn read_typed() -> avmath::GeopotentialAltitude {
        avmath::GeopotentialAltitude::new::<foot>(Self::read())
    }
}

impl AmbientDensity {
    fn read_typed() -> MassDensity {
        MassDensity::new::<slug_per_cubic_foot>(Self::read())
    }
}

named_variable!(Throttle1Mode(ThrottleMode): "THROTTLE1_MODE");
named_variable!(Throttle2Mode(ThrottleMode): "THROTTLE2_MODE");

named_variable!(Throttle1Position(ThrottlePercent): "Throttle1_Pos");
named_variable!(Throttle2Position(ThrottlePercent): "Throttle2_Pos");

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, FromPrimitive, ToPrimitive)]
pub enum ThrottleEventType {
    AxisThrottleSet,
    AxisThrottle1Set,
    AxisThrottle2Set,
    AxisThrottleSetEx,
    AxisThrottle1SetEx,
    AxisThrottle2SetEx,
    ThrottleSet,
    Throttle1Set,
    Throttle2Set,
    ThrottleFull,
    Throttle1Full,
    Throttle2Full,
    ThrottleCut,
    Throttle1Cut,
    Throttle2Cut,
    ThrottleIncr,
    Throttle1Incr,
    Throttle2Incr,
    ThrottleDecr,
    Throttle1Decr,
    Throttle2Decr,
    IncreaseThrottle,
    DecreaseThrottle
}

impl simconnect_sys::EventType for ThrottleEventType {
    type EventsIter = &'static [simconnect_sys::EventDefinition<Self>];

    fn to_ffi(&self) -> simconnect_sys::ffi::RawEventId {
        self.to_u32().map(simconnect_sys::ffi::RawEventId).unwrap()
    }

    fn from_ffi(raw: simconnect_sys::ffi::RawEventId) -> Option<Self> {
        Self::from_u32(raw.0)
    }

    fn event_definitions() -> Self::EventsIter {
        &[
            simconnect_sys::EventDefinition {
                name: "AXIS_THROTTLE_SET",
                event: Self::AxisThrottleSet,
                is_maskable: true,
            },
            simconnect_sys::EventDefinition {
                name: "AXIS_THROTTLE1_SET",
                event: Self::AxisThrottle1Set,
                is_maskable: true,
            },
            simconnect_sys::EventDefinition {
                name: "AXIS_THROTTLE2_SET",
                event: Self::AxisThrottle2Set,
                is_maskable: true,
            },
            simconnect_sys::EventDefinition {
                name: "THROTTLE_AXIS_SET_EX1",
                event: Self::AxisThrottleSetEx,
                is_maskable: true,
            },
            simconnect_sys::EventDefinition {
                name: "THROTTLE1_AXIS_SET_EX1",
                event: Self::AxisThrottle1SetEx,
                is_maskable: true,
            },
            simconnect_sys::EventDefinition {
                name: "THROTTLE2_AXIS_SET_EX1",
                event: Self::AxisThrottle2SetEx,
                is_maskable: true,
            },
            simconnect_sys::EventDefinition {
                name: "THROTTLE_SET",
                event: Self::ThrottleSet,
                is_maskable: true,
            },
            simconnect_sys::EventDefinition {
                name: "THROTTLE1_SET",
                event: Self::Throttle1Set,
                is_maskable: true,
            },
            simconnect_sys::EventDefinition {
                name: "THROTTLE2_SET",
                event: Self::Throttle2Set,
                is_maskable: true,
            },
            simconnect_sys::EventDefinition {
                name: "THROTTLE_FULL",
                event: Self::ThrottleFull,
                is_maskable: true,
            },
            simconnect_sys::EventDefinition {
                name: "THROTTLE_INCR",
                event: Self::ThrottleIncr,
                is_maskable: true,
            },
            simconnect_sys::EventDefinition {
                name: "THROTTLE_DECR",
                event: Self::ThrottleDecr,
                is_maskable: true,
            },
            simconnect_sys::EventDefinition {
                name: "THROTTLE_CUT",
                event: Self::ThrottleCut,
                is_maskable: true,
            },
            simconnect_sys::EventDefinition {
                name: "INCREASE_THROTTLE",
                event: Self::IncreaseThrottle,
                is_maskable: true,
            },
            simconnect_sys::EventDefinition {
                name: "DECREASE_THROTTLE",
                event: Self::DecreaseThrottle,
                is_maskable: true,
            },
            simconnect_sys::EventDefinition {
                name: "THROTTLE1_FULL",
                event: Self::Throttle1Full,
                is_maskable: true,
            },
            simconnect_sys::EventDefinition {
                name: "THROTTLE1_INCR",
                event: Self::Throttle1Incr,
                is_maskable: true,
            },
            simconnect_sys::EventDefinition {
                name: "THROTTLE1_DECR",
                event: Self::Throttle1Decr,
                is_maskable: true,
            },
            simconnect_sys::EventDefinition {
                name: "THROTTLE1_CUT",
                event: Self::Throttle1Cut,
                is_maskable: true,
            },
            simconnect_sys::EventDefinition {
                name: "THROTTLE2_FULL",
                event: Self::Throttle2Full,
                is_maskable: true,
            },
            simconnect_sys::EventDefinition {
                name: "THROTTLE2_INCR",
                event: Self::Throttle2Incr,
                is_maskable: true,
            },
            simconnect_sys::EventDefinition {
                name: "THROTTLE2_DECR",
                event: Self::Throttle2Decr,
                is_maskable: true,
            },
            simconnect_sys::EventDefinition {
                name: "THROTTLE2_CUT",
                event: Self::Throttle2Cut,
                is_maskable: true,
            },
        ]
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, FromPrimitive, ToPrimitive)]
pub enum NotificationGroup {
    Throttle,
}

impl simconnect_sys::NotificationGroup for NotificationGroup {
    type GroupsIter = &'static [simconnect_sys::NotificationGroupDefinition<Self>];
    type EventType = ThrottleEventType;
    fn to_ffi(&self) -> simconnect_sys::ffi::RawNotificationGroupId {
        self.to_u32().map(simconnect_sys::ffi::RawNotificationGroupId).unwrap()
    }

    fn from_ffi(raw: simconnect_sys::ffi::RawNotificationGroupId) -> Option<Self> {
        Self::from_u32(raw.0)
    }

    fn group_definitions() -> Self::GroupsIter {
        &[
            simconnect_sys::NotificationGroupDefinition {
                group: Self::Throttle,
                priority: simconnect_sys::ffi::NotificationGroupPriority::HIGHEST_MASKABLE,
            }
        ]
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ThrottleMode {
    Undefined,
    Cruise,
    Climb,
    Takeoff
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
pub struct ThrottleValue(pub f64);

impl ThrottleValue {
    const MIN_VALUE: f64 = -16384.;
    const MAX_VALUE: f64 = 16384.;
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

    fn from_raw(value: f64) -> Self {
        Self(value).clamp()
    }

    fn clamp(self) -> Self {
        Self(clamp(self.0, Self::MIN_VALUE, Self::MAX_VALUE))
    }

    fn normalize(self) -> f64 {
        (self.0 - Self::MIN_VALUE) / Self::RANGE
    }

    fn normalize_cruise(self) -> f64 {
        (self.0 - Self::MIN_VALUE) / Self::CRUISE_RANGE
    }

    fn from_normalized(value: f64) -> Self {
        Self(value * Self::MAX_VALUE - Self::MIN_VALUE)
    }
}

impl fmt::Display for ThrottleValue {
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

    fn from_force(value: Force) -> Self {
        Self(value.get::<poundal>()).clamp()
    }

    fn clamp(self) -> Self {
        Self(clamp(self.0, Self::MIN_VALUE, Self::MAX_VALUE))
    }

    fn normalize(self) -> f64 {
        (self.0 - Self::MIN_VALUE) / Self::RANGE
    }

    fn from_normalized(value: f64) -> Self {
        Self(value * Self::MAX_VALUE - Self::MIN_VALUE)
    }
}

impl fmt::Display for ThrustValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:.3} lbf", self.0)
    }
}

// #[derive(Clone, Copy, Debug, PartialEq)]
// #[repr(transparent)]
// pub struct Percents<X> {
//     value: f64,
//     _phantom: PhantomData<X>,
// }

// impl<X> Percents<X> {
//     const MIN_VALUE: f64 = 0.;
//     const MAX_VALUE: f64 = 100.;
//     const RANGE: f64 = Self::MAX_VALUE - Self::MIN_VALUE;

//     pub const MIN: Self = Self { value: Self::MIN_VALUE, _phantom: PhantomData };
//     pub const MAX: Self = Self { value: Self::MAX_VALUE, _phantom: PhantomData };

//     fn from_raw(value: f64) -> Self {
//         Self { value, _phantom: PhantomData }.clamp()
//     }

//     fn clamp(self) -> Self {
//         Self {
//             value: clamp(self.value, Self::MIN_VALUE, Self::MAX_VALUE),
//             _phantom: PhantomData,
//         }
//     }

//     fn normalize(self) -> f64 {
//         (self.value - Self::MIN_VALUE) / Self::RANGE
//     }

//     fn from_normalized(value: f64) -> Self {
//         Self(value * Self::MAX_VALUE - Self::MIN_VALUE)
//     }
// }

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(transparent)]
pub struct ThrottlePercent(pub f64);

impl ThrottlePercent {
    const MIN_VALUE: f64 = 0.;
    const MAX_VALUE: f64 = 100.;
    const RANGE: f64 = Self::MAX_VALUE - Self::MIN_VALUE;

    pub const MIN: Self = Self(Self::MIN_VALUE);
    pub const MAX: Self = Self(Self::MAX_VALUE);

    fn from_ratio(value: Ratio) -> Self {
        Self(value.get::<percent>()).clamp()
    }

    fn clamp(self) -> Self {
        Self(clamp(self.0, Self::MIN_VALUE, Self::MAX_VALUE))
    }

    fn normalize(self) -> f64 {
        (self.0 - Self::MIN_VALUE) / Self::RANGE
    }

    fn from_normalized(value: f64) -> Self {
        Self(value * Self::MAX_VALUE - Self::MIN_VALUE)
    }
}

impl From<ThrottleValue> for ThrottlePercent {
    fn from(v: ThrottleValue) -> Self {
        Self::from_normalized(v.normalize())
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

#[derive(Debug)]
#[repr(C)]
pub struct EngineDataControl {
    throttle_left: ThrottlePercent,
    throttle_right: ThrottlePercent,
}

impl simconnect_sys::DataDefinitionGroup for EngineDataControl {
    type DataDefsIter = &'static [simconnect_sys::DataDefinition];

    fn group_id() -> simconnect_sys::ffi::RawDataDefinitionId {
        simconnect_sys::ffi::RawDataDefinitionId(0)
    }

    fn data_definitions() -> Self::DataDefsIter {
        &[
            simconnect_sys::DataDefinition {
                name: "GENERAL ENG THROTTLE LEVER POSITION:1",
                unit: "Percent",
                datum_type: simconnect_sys::ffi::DataType::Float64,
            },
            simconnect_sys::DataDefinition {
                name: "GENERAL ENG THROTTLE LEVER POSITION:2",
                unit: "Percent",
                datum_type: simconnect_sys::ffi::DataType::Float64,
            },
        ]
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EngineNumber {
    Engine1,
    Engine2
}

impl EngineNumber {
    const fn index(self) -> usize {
        match self {
            Self::Engine1 => 0,
            Self::Engine2 => 1,
        }
    }

    const fn sim_index(self) -> u32 {
        match self {
            Self::Engine1 => 1,
            Self::Engine2 => 2,
        }
    }

    fn set_throttle_position(self, pct: ThrottlePercent) {
        match self {
            Self::Engine1 => Throttle1Position::set(pct),
            Self::Engine2 => Throttle2Position::set(pct),
        }
    }

    fn set_throttle_mode(self, mode: ThrottleMode) {
        match self {
            Self::Engine1 => Throttle1Mode::set(mode),
            Self::Engine2 => Throttle2Mode::set(mode),
        }
    }
}

#[derive(Debug)]
pub struct FdController {
    pid_config: PidConfiguration<Force>,
    pid_state: [PidState<Force>; 2],
    throttle_axes: [ThrottleValue; 2],
    throttle_mode: [ThrottleMode; 2],
    enabled: bool,
}

impl FdController {
    pub fn new() -> Self {
        Self {
            pid_config: ClimbFadecPidConfiguration::default(),
            pid_state: [PidState::default(); 2],
            throttle_axes: [ThrottleValue::MIN; 2],
            throttle_mode: [ThrottleMode::Undefined; 2],
            enabled: true,
        }
    }

    pub fn update(&mut self, throttle_axes: [ThrottleValue; 2], simconnect: &simconnect_sys::SimConnect, delta_t: Time) {
        //println!("Updating");
        self.throttle_axes = throttle_axes;
        self.update_throttle_mode(EngineNumber::Engine1);
        self.update_throttle_mode(EngineNumber::Engine2);
        let update = self.get_thrust_update(delta_t);
        if let Err(err) = simconnect.update_user_data(&update) {
            println!("Error updating simconnect user data: {:?}", err);
        }
        self.update_visible_throttle(EngineNumber::Engine1);
        self.update_visible_throttle(EngineNumber::Engine2);
    }

    fn get_thrust_update(&mut self, delta_t: Time) -> EngineDataControl {
        //println!("Updating thrust");
        let left = self.get_desired_throttle(EngineNumber::Engine1, delta_t);
        let right = self.get_desired_throttle(EngineNumber::Engine2, delta_t);
        //println!("Thrust target: {}/{}, Throttle percent: {}/{}", left.0, right.0, left.1, right.1);

        EngineDataControl {
            throttle_left: left.1,
            throttle_right: right.1,
        }
    }

    fn get_desired_throttle(&mut self, engine: EngineNumber, delta_t: Time) -> (ThrustValue, ThrottlePercent) {
        let normalized_throttle = self.throttle_axes[engine.index()].normalize();

        if !self.enabled {
            let throttle_exp = normalized_throttle.powf(3.5);
            return (ThrustValue::from_normalized(throttle_exp), ThrottlePercent::from_normalized(normalized_throttle))
        }

        const THRUST_FACTOR: f64 = 0.93;

        match self.throttle_mode[engine.index()] {
            ThrottleMode::Takeoff => {
                (ThrustValue::MAX, ThrottlePercent::MAX)
            }
            ThrottleMode::Climb => {
                let gross_thrust = convert_to_gross_thrust(Thrust::read_index_typed(engine.sim_index()), AirspeedMach::read());
                let max_density_thrust = get_max_density_thrust(AmbientDensity::read_typed());
                let plane_altitude = Altitude::read_typed();

                println!("{:?}: Gross thrust: {}, Max density thrust: {}, altitude: {}", engine, gross_thrust.into_format_args(poundal, uom::fmt::DisplayStyle::Abbreviation), max_density_thrust.into_format_args(poundal, uom::fmt::DisplayStyle::Abbreviation), plane_altitude.remove_context().into_format_args(foot, uom::fmt::DisplayStyle::Abbreviation));

                let low_altitude_limit = avmath::GeopotentialAltitude::new::<foot>(7000.);
                let altitude_reduction: Length = low_altitude_limit - plane_altitude;
                let low_altitude_thrust: Force =
                    (altitude_reduction * MassRate::new::<pound_per_second>(1.) / Time::new::<second>(24.))
                        .max(Force::new::<poundal>(0.));
                let low_thrust_target: Force = Force::new::<poundal>(2050.) + low_altitude_thrust;

                let target_thrust: Force = if (max_density_thrust * THRUST_FACTOR) < low_thrust_target {
                    let high_altitude_limit = avmath::GeopotentialAltitude::new::<foot>(35000.);
                    let altitude_reduction: Length = plane_altitude - high_altitude_limit;
                    let high_altitude_thrust_reduction: Force =
                        (altitude_reduction * MassRate::new::<pound_per_second>(1.) / Time::new::<second>(64.))
                            .max(Force::new::<poundal>(0.))
                            .min(Force::new::<poundal>(110.));

                    (max_density_thrust * THRUST_FACTOR) - high_altitude_thrust_reduction
                } else {
                    low_thrust_target
                };

                let output = self.pid_state[engine.index()].tick(target_thrust, &self.pid_config, gross_thrust, delta_t);

                let next_throttle = Throttle::read_index_typed(engine.sim_index()) + output;
                //println!("{:?}: Target thrust: {} (error: {}); adjusting throttle {} to {} of maximum", engine, target_thrust.into_format_args(poundal, uom::fmt::DisplayStyle::Abbreviation), error.into_format_args(poundal, uom::fmt::DisplayStyle::Abbreviation), next_state.output.into_format_args(ratio, uom::fmt::DisplayStyle::Abbreviation), next_throttle.into_format_args(ratio, uom::fmt::DisplayStyle::Abbreviation));

                (ThrustValue::from_force(target_thrust), ThrottlePercent::from_ratio(next_throttle))
            }
            ThrottleMode::Cruise | ThrottleMode::Undefined => {
                let cruise_normalized_throttle = self.throttle_axes[engine.index()].normalize_cruise();
                let cruise_throttle_exp = cruise_normalized_throttle;
                let throttle_exp = cruise_throttle_exp * THRUST_FACTOR;

                (ThrustValue::from_normalized(throttle_exp), ThrottlePercent::from_normalized(throttle_exp))
            }
        }
    }

    fn update_visible_throttle(&self, engine: EngineNumber) {
        let target_throttle = match self.throttle_mode[engine.index()] {
            ThrottleMode::Takeoff => ThrottleValue::TAKEOFF,
            ThrottleMode::Climb => ThrottleValue::CLIMB,
            ThrottleMode::Cruise | ThrottleMode::Undefined => self.throttle_axes[engine.index()],
        };

        engine.set_throttle_position(ThrottlePercent::from_normalized(target_throttle.normalize()));
    }

    fn update_throttle_mode(&mut self, engine: EngineNumber) {
        let value = self.throttle_axes[engine.index()];

        let mode = if value > ThrottleValue::CLIMB_MAX {
            ThrottleMode::Takeoff
        } else if value > ThrottleValue::CRUISE_MAX {
            ThrottleMode::Climb
        } else if value > ThrottleValue::UNDEF_MAX {
            ThrottleMode::Cruise
        } else {
            ThrottleMode::Undefined
        };

        self.throttle_mode[engine.index()] = mode;
        engine.set_throttle_mode(mode);
    }
}

fn convert_to_gross_thrust(thrust_in: Force, mach_in: f64) -> Force {
    thrust_in * (1. + (mach_in.powi(2) / 5.)).powf(3.5)
}

fn get_max_density_thrust(ambient_density: MassDensity) -> Force {
    let DENSITY_FACTOR = Volume::new::<cubic_foot>(1.) * Acceleration::new::<foot_per_second_squared>(1_351_600.);
    let f: Force = ambient_density * DENSITY_FACTOR;
    f + Force::new::<poundal>(250.)
}

struct ClimbFadecPidConfiguration;

impl ClimbFadecPidConfiguration {
    #[inline]
    fn default() -> PidConfiguration<Force> {
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
    use crate::pid::testing::pid_tick_tests;

    pid_tick_tests! {
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
            output: Ratio::new::<ratio>(0.00001),
            integral: Momentum::new::<pound_foot_per_second>(0.00001),
        },
    }
}