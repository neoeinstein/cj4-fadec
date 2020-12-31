use std::time::{Instant, Duration};
use num_traits::{ToPrimitive, clamp};
use simconnect_sys::{ffi::HResult, NotificationGroup, EventType};
use std::sync::Arc;

#[derive(Debug)]
pub struct FdGauge {
    simconnect: Arc<simconnect_sys::SimConnect>,
    controller: super::controller::FdController,
    last_update: Instant,
    throttle_axis: parking_lot::Mutex<[i32; 2]>,
}

const MIN_THRUST: i32 = -16384;
const THRUST_STEP: i32 = 256;
const MAX_THRUST: i32 = 16384;

impl FdGauge {
    pub fn new() -> Result<Self, HResult> {
        let simconnect= Arc::new(simconnect_sys::SimConnect::new("FdGauge")?);

        simconnect.register_notification_group_enum::<super::controller::NotificationGroup>()?;
        simconnect.register_data_definition::<super::controller::EngineDataControl>()?;

        let gauge = FdGauge {
            simconnect,
            controller: super::controller::FdController::new(),
            last_update: Instant::now(),
            throttle_axis: parking_lot::Mutex::new([MIN_THRUST; 2]),
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
            let throttle_values = {
                let lock = self.throttle_axis.lock();
                [
                    crate::controller::ThrottleValue(lock[0].to_f64().unwrap()),
                    crate::controller::ThrottleValue(lock[1].to_f64().unwrap()),
                ]
            };
            self.controller.update(throttle_values, &self.simconnect, draw_data.dt);
            self.last_update = now;
        }

        Ok(())
    }

    fn handle_axis_event(&self, event: &simconnect_sys::ffi::ReceiveEvent) {
        //println!("Received event!");
        if let Some(group) = crate::controller::NotificationGroup::from_ffi(event.group_id) {
            println!("Picked a group: {:?}", group);
            match group {
                crate::controller::NotificationGroup::Throttle => {
                    if let Some(event_type) = crate::controller::ThrottleEventType::from_ffi(event.event_id) {
                        println!("Picked an event type: {:?}", event_type);
                        let mut lock = self.throttle_axis.lock();
                        //println!("Locked...");
                        println!("Associated data: {} {} {:x}", event.data, event.data as i32, event.data);
                        match event_type {
                            crate::controller::ThrottleEventType::AxisThrottleSet |
                            crate::controller::ThrottleEventType::AxisThrottleSetEx => {
                                *lock = [event.data as i32; 2];
                            }
                            crate::controller::ThrottleEventType::AxisThrottle1Set |
                            crate::controller::ThrottleEventType::AxisThrottle1SetEx => {
                                lock[0] = event.data as i32;
                            }
                            crate::controller::ThrottleEventType::AxisThrottle2Set |
                            crate::controller::ThrottleEventType::AxisThrottle2SetEx => {
                                lock[1] = event.data as i32;
                            }
                            crate::controller::ThrottleEventType::ThrottleSet => {
                                *lock = [event.data as i32 * 2 + MIN_THRUST; 2];
                            }
                            crate::controller::ThrottleEventType::Throttle1Set => {
                                lock[0] = event.data as i32 * 2 + MIN_THRUST;
                            }
                            crate::controller::ThrottleEventType::Throttle2Set => {
                                lock[1] = event.data as i32 * 2 + MIN_THRUST;
                            }
                            crate::controller::ThrottleEventType::ThrottleFull => {
                                *lock = [MAX_THRUST; 2];
                            }
                            crate::controller::ThrottleEventType::Throttle1Full => {
                                lock[0] = MAX_THRUST;
                            }
                            crate::controller::ThrottleEventType::Throttle2Full => {
                                lock[1] = MAX_THRUST;
                            }
                            crate::controller::ThrottleEventType::ThrottleCut => {
                                *lock = [MIN_THRUST; 2];
                            }
                            crate::controller::ThrottleEventType::Throttle1Cut => {
                                lock[0] = MIN_THRUST;
                            }
                            crate::controller::ThrottleEventType::Throttle2Cut => {
                                lock[1] = MIN_THRUST;
                            }
                            crate::controller::ThrottleEventType::ThrottleIncr |
                            crate::controller::ThrottleEventType::IncreaseThrottle => {
                                lock[0] += THRUST_STEP;
                                lock[1] += THRUST_STEP;
                            }
                            crate::controller::ThrottleEventType::Throttle1Incr => {
                                lock[0] += THRUST_STEP;
                            }
                            crate::controller::ThrottleEventType::Throttle2Incr => {
                                lock[1] += THRUST_STEP;
                            }
                            crate::controller::ThrottleEventType::ThrottleDecr |
                            crate::controller::ThrottleEventType::DecreaseThrottle => {
                                lock[0] -= THRUST_STEP;
                                lock[1] -= THRUST_STEP;
                            }
                            crate::controller::ThrottleEventType::Throttle1Decr => {
                                lock[0] -= THRUST_STEP;
                            }
                            crate::controller::ThrottleEventType::Throttle2Decr => {
                                lock[1] -= THRUST_STEP;
                            }
                        }

                        lock[0] = clamp(lock[0], MIN_THRUST, MAX_THRUST);
                        lock[1] = clamp(lock[1], MIN_THRUST, MAX_THRUST);
                        
                        println!("Updated throttles: {} {}", lock[0], lock[1]);
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
