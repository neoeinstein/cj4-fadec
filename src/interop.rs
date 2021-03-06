use gauge_sys::{
    gauge_unit, indexed_aircraft_variable, named_variable, unindexed_aircraft_variable,
};
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};
use uom::si::{
    f64::*,
    force::poundal,
    length::foot,
    mass_density::slug_per_cubic_foot,
    ratio::ratio,
    velocity::{foot_per_second, knot},
};
use wt_cj4::{
    control_params::{ThrottleMode, ThrottlePercent},
    engines::EngineNumber,
};

gauge_unit!(Percent: "Percent"; "A percentage, expressed as a value between 0 and 100");
gauge_unit!(Pounds: "Pounds"; "Weight measured in pounds or Force measured in poundals of force");
gauge_unit!(Feet: "Feet"; "Distance measured in feet");
gauge_unit!(Number: "Number"; "A dimensionless value");
gauge_unit!(Mach: "Mach"; "Velocity measures as a ratio of the speed of sound");
gauge_unit!(Knots: "Knots"; "Nautical miles per hour");
gauge_unit!(FeetPerSecond: "Feet per second"; "Feet per second");
gauge_unit!(SluggerSlugs: "Slug per cubic feet"; "Pressure measured in slugs per cubic foot");
gauge_unit!(Bool: "Bool"; "A boolean value which is either off (0) or on (1)");

indexed_aircraft_variable!(Throttle(Percent): "GENERAL ENG THROTTLE LEVER POSITION"; "Engine throttle lever position");
indexed_aircraft_variable!(Thrust(Pounds): "TURB ENG JET THRUST"; "Turbine engine jet thrust");
unindexed_aircraft_variable!(AirspeedMach(Mach): "AIRSPEED MACH"; "Airspeed as Mach number");
unindexed_aircraft_variable!(AirspeedIndicated(Knots): "AIRSPEED INDICATED"; "Airspeed as indicated by pitot pressure");
unindexed_aircraft_variable!(AirspeedTrue(Knots): "AIRSPEED TRUE"; "True airspeed");
unindexed_aircraft_variable!(VerticalSpeed(FeetPerSecond): "VERTICAL SPEED"; "Vertical speed");
unindexed_aircraft_variable!(PressureAltitude(Feet): "PRESSURE ALTITUDE"; "Pressure altitude");
unindexed_aircraft_variable!(GeometricAltitude(Feet): "PLANE ALTITUDE"; "Plane altitude");
unindexed_aircraft_variable!(AmbientDensity(SluggerSlugs): "AMBIENT DENSITY"; "Ambient air density");
unindexed_aircraft_variable!(OnGround(Number): "SIM ON GROUND"; "Whether the user's aircraft is on the ground");

named_variable!(Throttle1Mode(ThrottleMode): "THROTTLE1_MODE"; "The FADEC mode of engine 1");
named_variable!(Throttle2Mode(ThrottleMode): "THROTTLE2_MODE"; "The FADEC mode of engine 2");

named_variable!(Throttle1Position(ThrottlePercent): "Throttle1_Pos"; "The visual position of the engine 1 throttle lever");
named_variable!(Throttle2Position(ThrottlePercent): "Throttle2_Pos"; "The visual position of the engine 2 throttle lever");
named_variable!(FlightDataRecorderEnabled(Boolean): "FLIGHT_DATA_RECORDER_ENABLED"; "Whether or not the flight data recorder should be enabled");

fn engine_number_to_sim_index(engine: EngineNumber) -> u32 {
    match engine {
        EngineNumber::Engine1 => 1,
        EngineNumber::Engine2 => 2,
    }
}

impl Throttle {
    // pub fn read_by_index(engine: EngineNumber) -> Ratio {
    //     let index = engine_number_to_sim_index(engine);
    //     Ratio::new::<percent>(Self::read_raw_by_index(index))
    // }

    pub fn set_position(engine: EngineNumber, pct: ThrottlePercent) {
        match engine {
            EngineNumber::Engine1 => Throttle1Position::set_raw(pct),
            EngineNumber::Engine2 => Throttle2Position::set_raw(pct),
        }
    }

    pub fn set_mode(engine: EngineNumber, mode: ThrottleMode) {
        match engine {
            EngineNumber::Engine1 => Throttle1Mode::set_raw(mode),
            EngineNumber::Engine2 => Throttle2Mode::set_raw(mode),
        }
    }
}

impl AirspeedMach {
    pub fn read() -> Ratio {
        Ratio::new::<ratio>(Self::read_raw())
    }
}

impl AirspeedIndicated {
    pub fn read() -> Velocity {
        Velocity::new::<knot>(Self::read_raw())
    }
}

impl AirspeedTrue {
    pub fn read() -> Velocity {
        Velocity::new::<knot>(Self::read_raw())
    }
}

impl VerticalSpeed {
    pub fn read() -> Velocity {
        Velocity::new::<foot_per_second>(Self::read_raw())
    }
}

impl Thrust {
    pub fn read_by_index(engine: EngineNumber) -> Force {
        let index = engine_number_to_sim_index(engine);
        Force::new::<poundal>(Self::read_raw_by_index(index))
    }
}

impl PressureAltitude {
    pub fn read() -> avmath::isa::PressureAltitude {
        avmath::isa::PressureAltitude::new::<foot>(Self::read_raw())
    }
}

impl GeometricAltitude {
    pub fn read() -> avmath::isa::GeometricAltitude {
        avmath::isa::GeometricAltitude::new::<foot>(Self::read_raw())
    }
}

impl AmbientDensity {
    pub fn read() -> MassDensity {
        MassDensity::new::<slug_per_cubic_foot>(Self::read_raw())
    }
}

impl FlightDataRecorderEnabled {
    pub fn read() -> bool {
        Self::read_raw() == Boolean::True
    }
}

/// A boolean value received through the Gauge API
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Boolean {
    /// False (0.0)
    False,

    /// True (1.0)
    True,
}

impl From<f64> for Boolean {
    #[inline]
    fn from(v: f64) -> Self {
        if v > 0. {
            Self::True
        } else {
            Self::False
        }
    }
}

impl From<Boolean> for f64 {
    #[inline]
    fn from(v: Boolean) -> Self {
        match v {
            Boolean::True => 1.,
            Boolean::False => 0.,
        }
    }
}

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
    DecreaseThrottle,
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
        self.to_u32()
            .map(simconnect_sys::ffi::RawNotificationGroupId)
            .unwrap()
    }

    fn from_ffi(raw: simconnect_sys::ffi::RawNotificationGroupId) -> Option<Self> {
        Self::from_u32(raw.0)
    }

    fn group_definitions() -> Self::GroupsIter {
        &[simconnect_sys::NotificationGroupDefinition {
            group: Self::Throttle,
            priority: simconnect_sys::ffi::NotificationGroupPriority::HIGHEST_MASKABLE,
        }]
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct EngineDataControl {
    pub throttle_engine1: ThrottlePercent,
    pub throttle_engine2: ThrottlePercent,
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
