use gauge_sys::ffi::{RawServiceId, ServiceId};

mod pid;
mod controller;
mod gauges;

// #[no_mangle]
// pub extern "C" fn add(a: u32, b: u32) -> u32 {
//     a + b
// }

// #[no_mangle]
// pub extern "C" fn testertwo() {
//     simconnect_sys::SimConnect::new("FdGauge\0");
// }

// #[no_mangle]
// pub extern "C" fn testeroo() -> f64 {
//     let throttle1 = controller::Throttle::read_index(1);
//     let throttle2 = controller::Throttle::read_index(2);

//     let altitude = controller::Altitude::read();

//     controller::Throttle1Mode::set(controller::ThrottleMode::Undefined);
//     controller::Throttle1Position::set(controller::ThrottleValue::MIN.into());
    
//     throttle1 + throttle2 + altitude
// }

static GAUGE: parking_lot::Mutex<Option<gauges::FdGauge>> = parking_lot::const_mutex(None);

#[no_mangle]
pub extern "C" fn FdGauge_gauge_callback(_ctx: gauge_sys::ffi::FsContext, raw_service_id: RawServiceId, extra_data: *const std::ffi::c_void) -> bool {
    if let Some(service_id) = ServiceId::from_ffi(raw_service_id) {
        match service_id {
            ServiceId::PreInstall => true,
            ServiceId::PostInstall => {
                //std::env::set_var("RUST_BACKTRACE", "full");
                let mut gauge = GAUGE.lock();
                if gauge.is_none() {
                    let new_gauge = gauges::FdGauge::new();
                    *gauge = new_gauge.ok();
                    gauge.is_some()
                } else {
                    true
                }
            },
            ServiceId::PreDraw => {
                let draw_data = unsafe { (extra_data as *const gauge_sys::ffi::GaugeDrawData).as_ref() };
                let mut gauge = GAUGE.lock();
                if let (Some(g), Some(data)) = (gauge.as_mut(), draw_data) {
                    g.on_update(data).is_ok()
                } else {
                    false
                }
            },
            ServiceId::PreKill => {
                GAUGE.lock().take();
                true
            },
            _ => false,
        }
    } else { 
        false
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
