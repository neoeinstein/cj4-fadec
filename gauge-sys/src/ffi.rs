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

pub fn unregister_named_variables() {
    unsafe { unregister_all_named_vars() }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[repr(transparent)]
pub struct FsContext(u64);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct RawServiceId(u32);

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
    #[inline]
    pub fn to_ffi(self) -> RawServiceId {
        self.to_u32().map(RawServiceId).unwrap()
    }

    #[inline]
    pub fn from_ffi(raw: RawServiceId) -> Option<Self> {
        Self::from_u32(raw.0)
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct GaugeDrawData
{
    pub mx: f64,
    pub my: f64,
    pub t: f64,
    pub dt: f64,
    pub window_width: u32,
    pub window_height: u32,
    pub framebuffer_width: u32,
    pub framebuffer_height: u32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[repr(transparent)]
pub struct RawUnit(u32);

impl RawUnit {
    pub unsafe fn from_units_enum_str(name: &str) -> Self {
        let name = CStr::from_bytes_with_nul_unchecked(name.as_bytes());
        get_units_enum(name.as_ptr())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[repr(transparent)]
pub struct RawAircraftVariable(u32);

impl RawAircraftVariable {
    pub unsafe fn from_aircraft_variable_enum_str(name: &str) -> Self {
        let name = CStr::from_bytes_with_nul_unchecked(name.as_bytes());
        get_aircraft_var_enum(name.as_ptr())
    }

    pub fn read(self, unit: RawUnit, index: u32) -> f64 {
        unsafe { aircraft_varget(self, unit, index) }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[repr(transparent)]
pub struct RawNamedVariable(u32);

impl RawNamedVariable {
    pub unsafe fn register_new(name: &str) -> Self {
        let name = CStr::from_bytes_with_nul_unchecked(name.as_bytes());
        register_named_variable(name.as_ptr())
    }

    pub fn set(self, value: f64) {
        unsafe { set_named_variable_value(self, value)}
    }
}
