use crate::interop;
use simconnect_sys::{ffi::HResult, EventType, NotificationGroup};
use std::sync::Arc;
use std::time::{Duration, Instant};
use uom::si::{f64::Time, time::second};
use wt_cj4::{
    control_params::{ThrottleAxis, ThrottleMode, ThrottlePercent},
    engines::{EngineData, EngineNumber},
    Aircraft, EngineReadings, Environment, Instruments, Snapshot,
};

#[derive(Debug)]
pub struct FdGauge {
    simconnect: Arc<simconnect_sys::SimConnect>,
    state: Aircraft,
    last_update: Instant,
    sim_start: Option<Time>,
    recorder: Option<wt_flight_recorder::FlightDataRecorder<Snapshot>>,
}

impl FdGauge {
    pub fn new() -> Result<Self, HResult> {
        let recorder = match wt_flight_recorder::FlightDataRecorder::new() {
            Ok(recorder) => Some(recorder),
            Err(err) => {
                eprintln!("Error creating flight data recorder: {:?}", err);
                None
            }
        };
        let simconnect = Arc::new(simconnect_sys::SimConnect::new("FdGauge")?);

        simconnect.register_notification_group_enum::<interop::NotificationGroup>()?;
        simconnect.register_data_definition::<interop::EngineDataControl>()?;

        let gauge = FdGauge {
            simconnect,
            state: Aircraft::default(),
            last_update: Instant::now(),
            sim_start: None,
            recorder,
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

        let duration = now.duration_since(self.last_update);

        if duration > Duration::from_millis(50) {
            let delta_t = Time::new::<second>(duration.as_secs_f64());
            let start_time = *self
                .sim_start
                .get_or_insert(Time::new::<second>(draw_data.t));
            let sim_time = Time::new::<second>(draw_data.t) - start_time;

            let instruments = Instruments {
                mach_number: interop::AirspeedMach::read(),
                ambient_density: interop::AmbientDensity::read(),
                geometric_altitude: interop::GeometricAltitude::read(),
                pressure_altitude: interop::PressureAltitude::read(),
            };

            let engines = EngineData::new_from(|e| EngineReadings {
                thrust: interop::Thrust::read_by_index(e),
            });

            let environment = Environment {
                instruments,
                engines,
            };

            self.step(&environment, delta_t);

            self.record(environment, sim_time, delta_t);

            self.update_sim();

            self.last_update = now;
        }

        Ok(())
    }

    fn step(&mut self, environment: &Environment, delta_t: Time) {
        self.state
            .engines
            .zip(&environment.engines, |_, engine, input| {
                engine.mode = select_throttle_mode(engine.physical_throttle);
                let (_, throttle_command) = engine.fadec.get_desired_throttle(
                    engine.physical_throttle.to_ratio(),
                    engine.mode,
                    input.thrust,
                    environment.instruments.mach_number,
                    environment.instruments.ambient_density,
                    environment.instruments.pressure_altitude,
                    delta_t,
                );
                engine.engine_throttle = throttle_command;
                engine.visual_throttle =
                    calculate_throttle_position(engine.mode, engine.physical_throttle);
            });
    }

    fn record(&mut self, environment: Environment, sim_time: Time, delta_t: Time) {
        if let Some(r) = &mut self.recorder {
            r.publish(&Snapshot {
                aircraft: self.state,
                environment,
                sim_time,
                delta_t,
            })
            .ok();
        }
    }

    fn update_sim(&self) {
        self.state.engines.for_each(|n, e| {
            interop::Throttle::set_position(n, e.visual_throttle);
            interop::Throttle::set_mode(n, e.mode);
        });

        let update = interop::EngineDataControl {
            throttle_engine1: self.state.engines[EngineNumber::Engine1].engine_throttle,
            throttle_engine2: self.state.engines[EngineNumber::Engine2].engine_throttle,
        };

        if let Err(err) = self.simconnect.update_user_data(&update) {
            println!("Error updating simconnect user data: {:?}", err);
        }
    }

    fn handle_axis_event(&mut self, event: &simconnect_sys::ffi::ReceiveEvent) {
        //println!("Received event!");
        if let Some(group) = interop::NotificationGroup::from_ffi(event.group_id) {
            // println!("Picked a group: {:?}", group);
            match group {
                interop::NotificationGroup::Throttle => {
                    if let Some(event_type) = interop::ThrottleEventType::from_ffi(event.event_id) {
                        // println!("Picked an event type: {:?}", event_type);
                        // println!(
                        //     "Associated data: {} {} {:x}",
                        //     event.data, event.data as i32, event.data
                        // );
                        match event_type {
                            interop::ThrottleEventType::AxisThrottleSet
                            | interop::ThrottleEventType::AxisThrottleSetEx => {
                                self.state.engines.update(|_, eng| {
                                    eng.physical_throttle =
                                        ThrottleAxis::from_raw_i32(event.data as i32)
                                });
                            }
                            interop::ThrottleEventType::AxisThrottle1Set
                            | interop::ThrottleEventType::AxisThrottle1SetEx => {
                                self.state.engines.engine1.physical_throttle =
                                    ThrottleAxis::from_raw_i32(event.data as i32);
                            }
                            interop::ThrottleEventType::AxisThrottle2Set
                            | interop::ThrottleEventType::AxisThrottle2SetEx => {
                                self.state.engines.engine2.physical_throttle =
                                    ThrottleAxis::from_raw_i32(event.data as i32);
                            }
                            interop::ThrottleEventType::ThrottleSet => {
                                self.state.engines.update(|_, eng| {
                                    eng.physical_throttle = ThrottleAxis::from_raw_u32(event.data)
                                });
                            }
                            interop::ThrottleEventType::Throttle1Set => {
                                self.state.engines.engine1.physical_throttle =
                                    ThrottleAxis::from_raw_u32(event.data);
                            }
                            interop::ThrottleEventType::Throttle2Set => {
                                self.state.engines.engine2.physical_throttle =
                                    ThrottleAxis::from_raw_u32(event.data);
                            }
                            interop::ThrottleEventType::ThrottleFull => {
                                self.state.engines.update(|_, eng| {
                                    eng.physical_throttle = ThrottleAxis::MAX;
                                });
                            }
                            interop::ThrottleEventType::Throttle1Full => {
                                self.state.engines.engine1.physical_throttle = ThrottleAxis::MAX;
                            }
                            interop::ThrottleEventType::Throttle2Full => {
                                self.state.engines.engine2.physical_throttle = ThrottleAxis::MAX;
                            }
                            interop::ThrottleEventType::ThrottleCut => {
                                self.state.engines.update(|_, eng| {
                                    eng.physical_throttle = ThrottleAxis::MIN;
                                });
                            }
                            interop::ThrottleEventType::Throttle1Cut => {
                                self.state.engines.engine1.physical_throttle = ThrottleAxis::MIN;
                            }
                            interop::ThrottleEventType::Throttle2Cut => {
                                self.state.engines.engine2.physical_throttle = ThrottleAxis::MIN;
                            }
                            interop::ThrottleEventType::ThrottleIncr
                            | interop::ThrottleEventType::IncreaseThrottle => {
                                self.state.engines.update(|_, eng| {
                                    eng.physical_throttle.inc();
                                });
                            }
                            interop::ThrottleEventType::Throttle1Incr => {
                                self.state.engines.engine1.physical_throttle.inc();
                            }
                            interop::ThrottleEventType::Throttle2Incr => {
                                self.state.engines.engine2.physical_throttle.inc();
                            }
                            interop::ThrottleEventType::ThrottleDecr
                            | interop::ThrottleEventType::DecreaseThrottle => {
                                self.state.engines.update(|_, eng| {
                                    eng.physical_throttle.dec();
                                });
                            }
                            interop::ThrottleEventType::Throttle1Decr => {
                                self.state.engines.engine1.physical_throttle.dec();
                            }
                            interop::ThrottleEventType::Throttle2Decr => {
                                self.state.engines.engine2.physical_throttle.dec();
                            }
                        }

                        // let last = self.last_throttle_axis.get();
                        // println!(
                        //     "Updated throttles: {} {}",
                        //     last[EngineNumber::Engine1],
                        //     last[EngineNumber::Engine2]
                        // );
                    }
                }
            }
        }
    }
}

impl simconnect_sys::SimConnectDispatcher for FdGauge {
    fn handle_event(&mut self, event: &simconnect_sys::ffi::ReceiveEvent) {
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
