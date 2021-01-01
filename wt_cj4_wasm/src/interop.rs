use gauge_sys::{gauge_unit, indexed_aircraft_variable, unindexed_aircraft_variable, named_variable};
use uom::si::{
    f64::*,
    force::poundal,
    length::foot,
    mass_density::slug_per_cubic_foot,
    ratio::{percent, ratio},
};
use num_derive::{ToPrimitive, FromPrimitive};
use num_traits::{ToPrimitive, FromPrimitive};
use wt_cj4::{control_params::{ThrottlePercent, ThrottleMode}, engines::EngineNumber};

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
unindexed_aircraft_variable!(Altitude(Feet): "PRESSURE ALTITUDE");
unindexed_aircraft_variable!(AmbientDensity(SluggerSlugs): "AMBIENT DENSITY");
unindexed_aircraft_variable!(OnGround(Number): "SIM ON GROUND");

named_variable!(Throttle1Mode(ThrottleMode): "THROTTLE1_MODE");
named_variable!(Throttle2Mode(ThrottleMode): "THROTTLE2_MODE");

named_variable!(Throttle1Position(ThrottlePercent): "Throttle1_Pos");
named_variable!(Throttle2Position(ThrottlePercent): "Throttle2_Pos");

fn engine_number_to_sim_index(engine: EngineNumber) -> u32 {
    match engine {
        EngineNumber::Engine1 => 1,
        EngineNumber::Engine2 => 2,
    }
}

impl Throttle {
    pub fn read_by_index(engine: EngineNumber) -> Ratio {
        let index = engine_number_to_sim_index(engine);
        Ratio::new::<percent>(Self::read_raw_by_index(index))
    }

    pub fn set_position(engine: EngineNumber, pct: ThrottlePercent) {
        match engine {
            EngineNumber::Engine1 => Throttle1Position::set(pct),
            EngineNumber::Engine2 => Throttle2Position::set(pct),
        }
    }

    pub fn set_mode(engine: EngineNumber, mode: ThrottleMode) {
        match engine {
            EngineNumber::Engine1 => Throttle1Mode::set(mode),
            EngineNumber::Engine2 => Throttle2Mode::set(mode),
        }
    }
}

impl AirspeedMach {
    pub fn read() -> Ratio {
        Ratio::new::<ratio>(Self::read_raw())
    }
}

impl Thrust {
    pub fn read_by_index(engine: EngineNumber) -> Force {
        let index = engine_number_to_sim_index(engine);
        Force::new::<poundal>(Self::read_raw_by_index(index))
    }
}

impl Altitude {
    pub fn read() -> avmath::PressureAltitude {
        avmath::PressureAltitude::new::<foot>(Self::read_raw())
    }
}

impl AmbientDensity {
    pub fn read() -> MassDensity {
        MassDensity::new::<slug_per_cubic_foot>(Self::read_raw())
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
