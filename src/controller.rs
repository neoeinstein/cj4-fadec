//use std::marker::PhantomData;
use std::fmt;
use gauge_sys::{IndexedAircraftVariable, UnindexedAircraftVariable, NamedVariable, gauge_unit, indexed_aircraft_variable, unindexed_aircraft_variable, named_variable};
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive, clamp};

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

    fn from_raw(value: f64) -> Self {
        Self(value).clamp()
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

    fn from_raw(value: f64) -> Self {
        Self(value).clamp()
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

    fn set_throttle_position(self, percent: ThrottlePercent) {
        match self {
            Self::Engine1 => Throttle1Position::set(percent),
            Self::Engine2 => Throttle2Position::set(percent),
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
    pid_config: PidConfiguration,
    pid_state: [PidState; 2],
    throttle_axes: [ThrottleValue; 2],
    throttle_mode: [ThrottleMode; 2],
    enabled: bool,
}

impl FdController {
    pub const fn new() -> Self {
        Self {
            pid_config: PidConfiguration::DEFAULT,
            pid_state: [PidState::INITIAL; 2],
            throttle_axes: [ThrottleValue::MIN; 2],
            throttle_mode: [ThrottleMode::Undefined; 2],
            enabled: true,
        }
    }

    pub fn update(&mut self, throttle_axes: [ThrottleValue; 2], simconnect: &simconnect_sys::SimConnect, delta_t: f64) {
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

    fn get_thrust_update(&mut self, delta_t: f64) -> EngineDataControl {
        //println!("Updating thrust");
        let left = self.get_desired_throttle(EngineNumber::Engine1, delta_t);
        let right = self.get_desired_throttle(EngineNumber::Engine2, delta_t);
        //println!("Done");
        let f = format!("Thrust target: {}/{}, Throttle percent: {}/{}", left.0, right.0, left.1, right.1);
        //let f = left.0.to_string();//format!("Thrust target: {}/{}", left.0, right.0);
        println!("{}", f);

        EngineDataControl {
            throttle_left: left.1,
            throttle_right: right.1,
        }
    }

    fn get_desired_throttle(&mut self, engine: EngineNumber, delta_t: f64) -> (ThrustValue, ThrottlePercent) {
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
                let gross_thrust = convert_to_gross_thrust(Thrust::read_index(engine.sim_index()), AirspeedMach::read());
                let max_density_thrust = get_max_density_thrust(AmbientDensity::read());
                let plane_altitude = Altitude::read();

                let low_altitude_thrust = ((7000. - plane_altitude) / 24.).max(0.);
                let low_thrust_target = 2050. + low_altitude_thrust;

                let target_thrust = if (max_density_thrust * THRUST_FACTOR) < low_thrust_target {
                    let high_altitude_thrust = clamp((-35000. + plane_altitude) / 64., 0., 110.);
                    (max_density_thrust * THRUST_FACTOR) - high_altitude_thrust
                } else {
                    low_thrust_target
                };

                let error = target_thrust - gross_thrust;
                let next_state = self.pid_state[engine.index()].tick(&self.pid_config, error, delta_t);
                self.pid_state[engine.index()] = next_state;
                (ThrustValue::from_raw(target_thrust), ThrottlePercent::from_raw(Throttle::read_index(engine.sim_index()) + next_state.output))
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

pub fn convert_to_gross_thrust(thrust_in: f64, mach_in: f64) -> f64 {
    thrust_in * (1. + (mach_in.powi(2) / 5.)).powf(3.5)
}

pub fn get_max_density_thrust(ambient_density: f64) -> f64 {
    // Slugs per cubic ft
    // 1 lbf = 1 slug * ft/s^2
    let density = ambient_density * 1000.;
    const DENSITY_FACTOR: f64 = 1351.6;
    density * DENSITY_FACTOR + 250.
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PidConfiguration {
    pub gain_proportion: f64,
    pub gain_integral: f64,
    pub gain_derivative: f64,
    pub min_output: f64,
    pub max_output: f64,
}

impl PidConfiguration {
    const DEFAULT: Self = Self {
        gain_proportion: 0.0012,
        gain_integral: 0.0001,
        gain_derivative: 0.0018,
        min_output: -2.,
        max_output: 2.,
    };
}

impl Default for PidConfiguration {
    fn default() -> Self {
        Self::DEFAULT
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PidState {
    error: f64,
    output: f64,
    integral: f64,
}

impl PidState {
    const INITIAL: Self = Self {
        error: 0.,
        output: 0.,
        integral: 0.,
    };
}

impl Default for PidState {
    fn default() -> Self {
        Self::INITIAL
    }
}

impl PidState {
    fn tick(self, config: &PidConfiguration, error: f64, delta_t: f64) -> Self {
        let proportion = config.gain_proportion * error;

        #[allow(clippy::float_cmp)]
        let integral = if error != self.error && error.signum() != self.error.signum() {
            0.
        } else {
            self.integral + (error * delta_t) + ((delta_t * (error - self.error)) / 2.)
        };

        let derivative = clamp(config.gain_derivative * ((error - self.error) / delta_t), -20., 20.);

        let output = clamp(proportion + config.gain_integral * integral + derivative, config.min_output, config.max_output);

        Self {
            error,
            output,
            integral,
        }
    }
}