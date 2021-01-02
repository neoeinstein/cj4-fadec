//! Low-level implementation details providing the required FFI bindings for
//! the legacy Gauge API.

#![allow(missing_docs)]

use std::ffi::CStr;
use std::os::raw::c_char;
use num_derive::{ToPrimitive, FromPrimitive};
use num_traits::{ToPrimitive, FromPrimitive};

extern "C" {
    fn get_units_enum(name: *const c_char) -> RawUnit;
    fn get_aircraft_var_enum(name: *const c_char) -> RawAircraftVariable;
    fn unregister_all_named_vars();
    fn register_named_variable(name: *const c_char) -> RawNamedVariable;
    fn set_named_variable_value(var: RawNamedVariable, value: f64);
    fn aircraft_varget(var: RawAircraftVariable, unit: RawUnit, index: u32) -> f64;
}

/// Unregisters all named variables
/// 
/// This function sould be called when the gauge is unloaded.
pub fn unregister_named_variables() {
    unsafe { unregister_all_named_vars() }
}

/// The Flight Simulator context
/// 
/// This is required for NanoVG operations.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[repr(transparent)]
pub struct FsContext(u64);

/// A raw service ID provided by the Gauge API on callbacks
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct RawServiceId(u32);

/// Events provided by the Gauge API
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, ToPrimitive, FromPrimitive)]
pub enum ServiceId {
    PreQuery,
    PostQuery,
    PreInstall, // extra_data = resource_handle
    PostInstall, // extra_data = resource_handle
    PreInitialize,
    PostInitialize,
    PreUpdate,
    PostUpdate,
    PreGenerate, // extra_data = phase
    PostGenerate, // extra_data = phase
    PreDraw,
    PostDraw,
    PreKill,
    PostKill,
    ConnectToWindow, // extra_data = PANEL_WND
    Disconnect, // extra_data = PANEL_WND
    PanelOpen,
    PanelClose,
}

impl ServiceId {
    /// Converts a `ServiceId` into the raw value used for FFI interactions
    #[inline]
    pub fn to_ffi(self) -> RawServiceId {
        self.to_u32().map(RawServiceId).unwrap()
    }

    /// Attempts to convert the raw service ID into a known `ServiceId`
    #[inline]
    pub fn from_ffi(raw: RawServiceId) -> Option<Self> {
        Self::from_u32(raw.0)
    }
}

/// Frame data from the Gauge API on `PreDraw` and `PostDraw` events
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct GaugeDrawData
{
    /// Mouse X coordinate
    pub mx: f64,
    /// Mouse Y coordinate
    pub my: f64,
    /// Simulator time (seconds)
    pub t: f64,
    /// Time since last draw in seconds
    pub dt: f64,
    /// Width of the window in pixels
    pub window_width: u32,
    /// Height of the window in pixels
    pub window_height: u32,
    /// Width of the framebuffer in pixels
    pub framebuffer_width: u32,
    /// Height of the framebuffer in pixels
    pub framebuffer_height: u32,
}

/// A raw unit identifier for Gauge API FFI
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[repr(transparent)]
pub struct RawUnit(u32);

impl RawUnit {
    /// Requests an identifer to use when requesting data in a specific unit
    /// 
    /// # Safety
    /// 
    /// The `name` string _must_ be null-terminated.
    pub unsafe fn from_units_enum_str(name: &str) -> Self {
        let name = CStr::from_bytes_with_nul_unchecked(name.as_bytes());
        get_units_enum(name.as_ptr())
    }
}

/// A raw aircraft variable identifier for Gauge API FFI
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[repr(transparent)]
pub struct RawAircraftVariable(u32);

impl RawAircraftVariable {
    /// Registers an interest in an aircraft variable
    /// 
    /// Returns an aircraft variable ID used for later interactions with this
    /// variable.
    /// 
    /// # Safety
    /// 
    /// The `name` string _must_ be null-terminated.
    pub unsafe fn from_aircraft_variable_enum_str(name: &str) -> Self {
        let name = CStr::from_bytes_with_nul_unchecked(name.as_bytes());
        get_aircraft_var_enum(name.as_ptr())
    }

    /// Reads the associated aircraft variable
    /// 
    /// For uninexed variables, pass `0` as `index`.
    pub fn read(self, unit: RawUnit, index: u32) -> f64 {
        unsafe { aircraft_varget(self, unit, index) }
    }
}

/// A raw named variable identifier for Gauge API FFI
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[repr(transparent)]
pub struct RawNamedVariable(u32);

impl RawNamedVariable {
    /// Registers a new named variable
    /// 
    /// Returns a named variable ID used for later interactions with this
    /// variable.
    /// 
    /// # Safety
    /// 
    /// The `name` string _must_ be null-terminated.
    pub unsafe fn register_new(name: &str) -> Self {
        let name = CStr::from_bytes_with_nul_unchecked(name.as_bytes());
        register_named_variable(name.as_ptr())
    }

    /// Sets the associated named variable to the provided value
    pub fn set(self, value: f64) {
        unsafe { set_named_variable_value(self, value)}
    }
}
