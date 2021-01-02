//! # Working Title CJ4 Web Assembly module
//!
//! This module provides a FADEC replacement for the stock engine management
//! system. The module loads as a gauge through configuration in an aircraft's
//! `panel.cfg`.

#![warn(
    missing_docs,
    unused_import_braces,
    unused_imports,
    unused_qualifications
)]
#![deny(missing_debug_implementations, unused_must_use)]

use gauge_sys::ffi::{RawServiceId, ServiceId};

mod gauges;
mod interop;

static GAUGE: parking_lot::Mutex<Option<gauges::FdGauge>> = parking_lot::const_mutex(None);

/// The primary entry point for Microsoft Flight Simulator modules built on
/// top of the legacy Gauge API. This function will be called externally by
/// the simulator as certain events occur.
#[no_mangle]
pub extern "C" fn FdGauge_gauge_callback(
    _ctx: gauge_sys::ffi::FsContext,
    raw_service_id: RawServiceId,
    extra_data: *const std::ffi::c_void,
) -> bool {
    if let Some(service_id) = ServiceId::from_ffi(raw_service_id) {
        match service_id {
            ServiceId::PreInstall => true,
            ServiceId::PostInstall => {
                let mut gauge = GAUGE.lock();
                if gauge.is_none() {
                    let new_gauge = gauges::FdGauge::new();
                    *gauge = new_gauge.ok();
                    gauge.is_some()
                } else {
                    true
                }
            }
            ServiceId::PreDraw => {
                let draw_data =
                    unsafe { (extra_data as *const gauge_sys::ffi::GaugeDrawData).as_ref() };
                let mut gauge = GAUGE.lock();
                if let (Some(g), Some(data)) = (gauge.as_mut(), draw_data) {
                    g.on_update(data).is_ok()
                } else {
                    false
                }
            }
            ServiceId::PreKill => {
                GAUGE.lock().take();
                true
            }
            _ => false,
        }
    } else {
        false
    }
}
