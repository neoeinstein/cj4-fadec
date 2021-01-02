use crate::interop;
use simconnect_sys::{ffi::HResult, EventType, NotificationGroup};
use std::cell::Cell;
use std::sync::Arc;
use std::time::{Duration, Instant};
use uom::fmt::DisplayStyle::Abbreviation;
use uom::si::{f64::Time, ratio::ratio, time::second};
use wt_cj4::{
    control_params::{ThrottleAxis, ThrottleMode, ThrottlePercent},
    engines::{EngineData, EngineNumber},
    FadecController,
};

#[derive(Debug)]
pub struct FdGauge {
    simconnect: Arc<simconnect_sys::SimConnect>,
    controller: EngineData<FadecController>,
    last_throttle_axis: Cell<EngineData<ThrottleAxis>>,
    last_update: Instant,
}

impl FdGauge {
    pub fn new() -> Result<Self, HResult> {
        let simconnect = Arc::new(simconnect_sys::SimConnect::new("FdGauge")?);

        simconnect.register_notification_group_enum::<interop::NotificationGroup>()?;
        simconnect.register_data_definition::<interop::EngineDataControl>()?;

        let gauge = FdGauge {
            simconnect,
            controller: EngineData::default(),
            last_throttle_axis: Cell::new(EngineData::new(ThrottleAxis::MIN)),
            last_update: Instant::now(),
        };

        println!("All set up: {:?}", gauge);

        Ok(gauge)
    }

    pub fn on_update(&mut self, draw_data: &gauge_sys::ffi::GaugeDrawData) -> Result<(), ()> {
        let now = Instant::now();

        {
            let sc = Arc::clone(&self.simconnect);
            sc.dispatch(self);
            // let mut dispatcher = FdGaugeDispatcher(self);
            // self.simconnect.dispatch(&mut dispatcher);
        }

        if now.duration_since(self.last_update) > Duration::from_millis(50) {
            self.update(Time::new::<second>(draw_data.dt));
            self.last_update = now;
        }

        Ok(())
    }

    pub fn update(&mut self, delta_t: Time) {
        let last_throttle_axis = self.last_throttle_axis.get();
        println!("Most recent throttle values: {:?}", last_throttle_axis);

        let intermediate = last_throttle_axis.map(|engine, axis| {
            println!("Processing {:?}", engine);
            let mode = select_throttle_mode(axis);
            interop::Throttle::set_mode(engine, mode);
            println!("{:?}: Updated mode to {:?}", engine, mode);

            let engine_thrust = interop::Thrust::read_by_index(engine);
            let mach_number = interop::AirspeedMach::read();
            let ambient_density = interop::AmbientDensity::read();
            let pressure_altitude = interop::Altitude::read();
            let current_throttle = axis.to_ratio();

            let (_, new_throttle) = self.controller[engine].get_desired_throttle(
                current_throttle,
                mode,
                engine_thrust,
                mach_number,
                ambient_density,
                pressure_altitude,
                delta_t,
            );
            println!(
                "{:?}: Current throttle: {:.4}, New throttle: {:+.4}",
                engine,
                current_throttle.into_format_args(ratio, Abbreviation),
                new_throttle
                    .to_ratio()
                    .into_format_args(ratio, Abbreviation)
            );

            let visible_position = calculate_throttle_position(mode, axis);
            println!("{:?}: Updating throttle to {:?}", engine, visible_position);
            interop::Throttle::set_position(engine, visible_position);

            new_throttle
        });

        let update = interop::EngineDataControl {
            throttle_engine1: intermediate[EngineNumber::Engine1],
            throttle_engine2: intermediate[EngineNumber::Engine2],
        };

        println!("Update prepared: {:?}", update);

        if let Err(err) = self.simconnect.update_user_data(&update) {
            println!("Error updating simconnect user data: {:?}", err);
        }
    }

    fn handle_axis_event(&self, event: &simconnect_sys::ffi::ReceiveEvent) {
        //println!("Received event!");
        if let Some(group) = interop::NotificationGroup::from_ffi(event.group_id) {
            println!("Picked a group: {:?}", group);
            match group {
                interop::NotificationGroup::Throttle => {
                    if let Some(event_type) = interop::ThrottleEventType::from_ffi(event.event_id) {
                        println!("Picked an event type: {:?}", event_type);
                        println!(
                            "Associated data: {} {} {:x}",
                            event.data, event.data as i32, event.data
                        );
                        match event_type {
                            interop::ThrottleEventType::AxisThrottleSet
                            | interop::ThrottleEventType::AxisThrottleSetEx => {
                                self.last_throttle_axis.set(EngineData::new(
                                    ThrottleAxis::from_raw_i32(event.data as i32),
                                ));
                            }
                            interop::ThrottleEventType::AxisThrottle1Set
                            | interop::ThrottleEventType::AxisThrottle1SetEx => {
                                self.last_throttle_axis.set(EngineData {
                                    engine1: ThrottleAxis::from_raw_i32(event.data as i32),
                                    ..self.last_throttle_axis.get()
                                });
                            }
                            interop::ThrottleEventType::AxisThrottle2Set
                            | interop::ThrottleEventType::AxisThrottle2SetEx => {
                                self.last_throttle_axis.set(EngineData {
                                    engine2: ThrottleAxis::from_raw_i32(event.data as i32),
                                    ..self.last_throttle_axis.get()
                                });
                            }
                            interop::ThrottleEventType::ThrottleSet => {
                                self.last_throttle_axis
                                    .set(EngineData::new(ThrottleAxis::from_raw_u32(event.data)));
                            }
                            interop::ThrottleEventType::Throttle1Set => {
                                self.last_throttle_axis.set(EngineData {
                                    engine1: ThrottleAxis::from_raw_u32(event.data),
                                    ..self.last_throttle_axis.get()
                                });
                            }
                            interop::ThrottleEventType::Throttle2Set => {
                                self.last_throttle_axis.set(EngineData {
                                    engine2: ThrottleAxis::from_raw_u32(event.data),
                                    ..self.last_throttle_axis.get()
                                });
                            }
                            interop::ThrottleEventType::ThrottleFull => {
                                self.last_throttle_axis
                                    .set(EngineData::new(ThrottleAxis::MAX));
                            }
                            interop::ThrottleEventType::Throttle1Full => {
                                self.last_throttle_axis.set(EngineData {
                                    engine1: ThrottleAxis::MAX,
                                    ..self.last_throttle_axis.get()
                                });
                            }
                            interop::ThrottleEventType::Throttle2Full => {
                                self.last_throttle_axis.set(EngineData {
                                    engine2: ThrottleAxis::MAX,
                                    ..self.last_throttle_axis.get()
                                });
                            }
                            interop::ThrottleEventType::ThrottleCut => {
                                self.last_throttle_axis
                                    .set(EngineData::new(ThrottleAxis::MIN));
                            }
                            interop::ThrottleEventType::Throttle1Cut => {
                                self.last_throttle_axis.set(EngineData {
                                    engine1: ThrottleAxis::MIN,
                                    ..self.last_throttle_axis.get()
                                });
                            }
                            interop::ThrottleEventType::Throttle2Cut => {
                                self.last_throttle_axis.set(EngineData {
                                    engine2: ThrottleAxis::MIN,
                                    ..self.last_throttle_axis.get()
                                });
                            }
                            interop::ThrottleEventType::ThrottleIncr
                            | interop::ThrottleEventType::IncreaseThrottle => {
                                let prior = self.last_throttle_axis.get();
                                self.last_throttle_axis.set(EngineData {
                                    engine1: prior.engine1.inc(),
                                    engine2: prior.engine2.inc(),
                                });
                            }
                            interop::ThrottleEventType::Throttle1Incr => {
                                let prior = self.last_throttle_axis.get();
                                self.last_throttle_axis.set(EngineData {
                                    engine1: prior.engine1.inc(),
                                    ..prior
                                });
                            }
                            interop::ThrottleEventType::Throttle2Incr => {
                                let prior = self.last_throttle_axis.get();
                                self.last_throttle_axis.set(EngineData {
                                    engine2: prior.engine2.inc(),
                                    ..prior
                                });
                            }
                            interop::ThrottleEventType::ThrottleDecr
                            | interop::ThrottleEventType::DecreaseThrottle => {
                                let prior = self.last_throttle_axis.get();
                                self.last_throttle_axis.set(EngineData {
                                    engine1: prior.engine1.dec(),
                                    engine2: prior.engine2.dec(),
                                });
                            }
                            interop::ThrottleEventType::Throttle1Decr => {
                                let prior = self.last_throttle_axis.get();
                                self.last_throttle_axis.set(EngineData {
                                    engine1: prior.engine1.dec(),
                                    ..prior
                                });
                            }
                            interop::ThrottleEventType::Throttle2Decr => {
                                let prior = self.last_throttle_axis.get();
                                self.last_throttle_axis.set(EngineData {
                                    engine1: prior.engine2.dec(),
                                    ..prior
                                });
                            }
                        }

                        let last = self.last_throttle_axis.get();
                        println!(
                            "Updated throttles: {} {}",
                            last[EngineNumber::Engine1],
                            last[EngineNumber::Engine2]
                        );
                    }
                }
            }
        }
    }
}

impl simconnect_sys::SimConnectDispatcher for FdGauge {
    fn handle_event(&self, event: &simconnect_sys::ffi::ReceiveEvent) {
        //println!("Received event! Passing it along...");
        //println!("What am I? {:?}", self as *const Self);
        self.handle_axis_event(event)
    }
}

impl Drop for FdGauge {
    fn drop(&mut self) {
        gauge_sys::ffi::unregister_named_variables();
    }
}

fn select_throttle_mode(axis: ThrottleAxis) -> ThrottleMode {
    if axis > ThrottleAxis::CLIMB_MAX {
        ThrottleMode::Takeoff
    } else if axis > ThrottleAxis::CRUISE_MAX {
        ThrottleMode::Climb
    } else if axis > ThrottleAxis::UNDEF_MAX {
        ThrottleMode::Cruise
    } else {
        ThrottleMode::Undefined
    }
}

fn calculate_throttle_position(mode: ThrottleMode, axis: ThrottleAxis) -> ThrottlePercent {
    let target_throttle = match mode {
        ThrottleMode::Takeoff => ThrottleAxis::TAKEOFF,
        ThrottleMode::Climb => ThrottleAxis::CLIMB,
        ThrottleMode::Cruise | ThrottleMode::Undefined => axis,
    };

    ThrottlePercent::from(target_throttle)
}
