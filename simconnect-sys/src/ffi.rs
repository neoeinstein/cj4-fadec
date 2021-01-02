//! Low-level FFI SimConnect APIs

#![allow(dead_code, missing_docs)]

use std::ffi::c_void;
use std::fmt;
use std::os::raw::c_char;
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};

extern "C" {
    pub fn SimConnect_Open(handle: *mut SimConnectHandle, name: *const c_char, wnd: WindowHandle, event: u32, event_handle: Handle, config_index: u32) -> HResult;
    pub fn SimConnect_Close(handle: SimConnectHandle) -> HResult;
    pub fn SimConnect_CallDispatch(handle: SimConnectHandle, dispatch: DispatchProc, context: *mut c_void) -> HResult;
    pub fn SimConnect_GetNextDispatch(handle: SimConnectHandle, header: *mut *const ReceiveHeader, size: *mut u32) -> HResult;
    pub fn SimConnect_AddToDataDefinition(handle: SimConnectHandle, data_definition_id: RawDataDefinitionId, name: *const c_char, units: *const c_char, datum_type: RawDataType, epsilon: f64, datum_id: u32) -> HResult;
    pub fn SimConnect_MapClientEventToSimEvent(handle: SimConnectHandle, event_id: RawEventId, event_name: *const c_char) -> HResult;
    pub fn SimConnect_AddClientEventToNotificationGroup(handle: SimConnectHandle, group_id: RawNotificationGroupId, event_id: RawEventId, maskable: bool) -> HResult;
    pub fn SimConnect_SetNotificationGroupPriority(handle: SimConnectHandle, group_id: RawNotificationGroupId, priority: NotificationGroupPriority) -> HResult;
    pub fn SimConnect_SetDataOnSimObject(handle: SimConnectHandle, data_definition: RawDataDefinitionId, object_id: RawObjectId, flags: RawDataSetFlag, array_count: u32, unit_size: u32, data_set: *const c_void) -> HResult;
}

type DispatchProc = extern fn(*const ReceiveHeader, u32, *mut c_void);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C)]
pub struct ReceiveHeader {
    pub size: u32,
    pub version: u32,
    pub message_type: RawMessageType,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C)]
pub struct ReceiveOpen {
    pub header: ReceiveHeader,
    pub application_name: [u8; 256],
    pub application_version: Version,
    pub simconnect_version: Version,
    pub reserved1: u32,
    pub reserved2: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C)]
pub struct Version {
    pub version_major: u32,
    pub version_minor: u32,
    pub build_major: u32,
    pub build_minor: u32,
}

impl ReceiveOpen {
    pub fn application_name(&self) -> &str {
        let null = self.application_name.iter().copied().position(|x| x == 0x00).unwrap_or(self.application_name.len());
        let slice = &self.application_name[0..null];
        std::str::from_utf8(slice).expect("Application name should be valid UTF-8")
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C)]
pub struct ReceiveEvent {
    pub header: ReceiveHeader,
    pub group_id: RawNotificationGroupId,
    pub event_id: RawEventId,
    pub data: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct RawDataDefinitionId(pub u32);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct RawObjectId(u32);

impl RawObjectId {
    pub const USER: Self = RawObjectId(0);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct RawNotificationGroupId(pub u32);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct RawEventId(pub u32);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct NotificationGroupPriority(u32);

impl NotificationGroupPriority {
    pub const HIGHEST: Self              = Self(         1);      // highest priority
    pub const HIGHEST_MASKABLE: Self     = Self(  10000000);      // highest priority that allows events to be masked
    pub const STANDARD: Self             = Self(1900000000);      // standard priority
    pub const DEFAULT: Self              = Self(2000000000);      // default priority
    pub const LOWEST: Self               = Self(4000000000);      // priorities lower than this will be ignored

    pub const fn custom(priority: u32) -> Self {
        Self(priority)
    }
}

impl Default for NotificationGroupPriority {
    #[inline]
    fn default() -> Self {
        Self::DEFAULT
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct RawDataSetFlag(u32);

#[derive(Clone, Copy, Debug, PartialEq, Eq, FromPrimitive, ToPrimitive)]
pub enum DataSetFlag {
    Default = 0x00000000,
    Tagged = 0x00000001,      // data is in tagged format
}

impl DataSetFlag {
    #[inline]
    pub fn to_ffi(self) -> RawDataSetFlag {
        self.to_u32().map(RawDataSetFlag).unwrap()
    }

    #[inline]
    pub fn from_ffi(raw: RawDataSetFlag) -> Option<Self> {
        Self::from_u32(raw.0)
    }
}

impl Default for DataSetFlag {
    #[inline]
    fn default() -> Self {
        Self::Default
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct RawMessageType(pub u32);

#[derive(Clone, Copy, Debug, PartialEq, Eq, FromPrimitive, ToPrimitive)]
pub enum MessageType {
    Null,
    Exception,
    Open,
    Quit,
    Event,
    EventObjectAddRemove,
    EventFilename,
    EventName,
    SimObjectData,
    SimObjectDataByType,
    WeatherObservation,
    CloudState,
    AssignedObjectId,
    ReservedKey,
    CustomAction,
    SystemState,
    ClientData,
    EventWeatherMode,
    AirportList,
    VorList,
    NdbList,
    WaypointList,
    EventMultiplayerServerStarted,
    EventMultiplayerClientStarted,
    EventMultiplayerSessionEnded,
    EventRaceEnd,
    EventRaceLap,
    Pick,
}

impl MessageType {
    #[inline]
    pub fn to_ffi(self) -> RawMessageType {
        self.to_u32().map(RawMessageType).unwrap()
    }

    #[inline]
    pub fn from_ffi(raw: RawMessageType) -> Option<Self> {
        Self::from_u32(raw.0)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct RawDataType(u32);

#[derive(Clone, Copy, Debug, PartialEq, Eq, FromPrimitive, ToPrimitive)]
pub enum DataType {
    Invalid,        // invalid data type
    Int32,          // 32-bit integer number
    Int64,          // 64-bit integer number
    Float32,        // 32-bit floating-point number (float)
    Float64,        // 64-bit floating-point number (double)
    String8,        // 8-byte string
    String32,       // 32-byte string
    String64,       // 64-byte string
    String128,      // 128-byte string
    String256,      // 256-byte string
    String260,      // 260-byte string
    StringV,        // variable-length string

    InitPosition,   // see SIMCONNECT_DATA_INITPOSITION
    MarkerState,    // see SIMCONNECT_DATA_MARKERSTATE
    Waypoint,       // see SIMCONNECT_DATA_WAYPOINT
    LatLongAlt,      // see SIMCONNECT_DATA_LATLONALT
    Xyz,            // see SIMCONNECT_DATA_XYZ

    Max,             // enum limit
}

impl DataType {
    #[inline]
    pub fn to_ffi(self) -> RawDataType {
        self.to_u32().map(RawDataType).unwrap()
    }

    #[inline]
    pub fn from_ffi(raw: RawDataType) -> Option<Self> {
        Self::from_u32(raw.0)
    }
}

impl Default for DataType {
    #[inline]
    fn default() -> Self {
        Self::Float64
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct SimConnectHandle(u64);

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Handle(u64);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct WindowHandle(*const Handle);

impl Default for WindowHandle {
    #[inline]
    fn default() -> Self {
        Self(std::ptr::null())
    }
}

/// Integer result value returned by most SimConnect APIs
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(transparent)]
#[must_use]
pub struct HResult(i32);

impl HResult {
    /// Success
    pub const S_OK: Self = Self(0);

    /// A generic failure
    pub const E_FAIL: Self = Self(0x80004005_u32 as i32);

    /// Indicates whether or not the operation was successful
    #[inline]
    pub fn is_success(self) -> bool {
        self.0 >= 0
    }

    /// Gets the raw HRESULT code
    #[inline]
    pub fn raw(self) -> i32 {
        self.0
    }

    /// Converts an HResult value into a Result
    #[inline]
    pub fn to_result(self) -> Result<(), Self> {
        if self.is_success() {
            Ok(())
        } else {
            Err(self)
        }
    }
}

impl fmt::Display for HResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let info = if *self == Self::S_OK {
            " (S_OK)"
        } else if *self == Self::E_FAIL {
            " (E_FAIL)"
        } else {
            ""
        };

        write!(f, "{:08x}{}", self.0, info)
    }
}