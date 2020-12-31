use crate::ffi;
use std::borrow::Cow;
use std::ffi::{CStr, CString};

#[derive(Debug)]
pub struct SimConnect {
    raw: ffi::SimConnectHandle,
}

impl SimConnect {
    pub fn new(name: &str) -> Result<Self, ffi::HResult> {
        let n = if let Ok(value) = CStr::from_bytes_with_nul(name.as_bytes()) {
            Cow::Borrowed(value)
        } else {
            Cow::Owned(CString::new(name).unwrap())
        };
        let mut handle = ffi::SimConnectHandle::default();
        let result = unsafe {
            ffi::SimConnect_Open(
                &mut handle as *mut ffi::SimConnectHandle,
                n.as_ptr(),
                ffi::WindowHandle::default(),
                0,
                ffi::Handle::default(),
                0)
        };

        if result.is_success() {
            Ok(SimConnect {
                raw: handle,
            })
        } else {
            Err(result)
        }
    }

    pub fn register_notification_group_enum<G: NotificationGroup>(&self) -> Result<(), ffi::HResult> {
        for def in G::group_definitions() {
            if let Err(err) = self.register_notification_group(def) {
                println!("Error registering definition for group");
                return Err(err);
            }
        }
        Ok(())
    }

    pub fn register_notification_group<G: NotificationGroup>(&self, group_def: &NotificationGroupDefinition<G>) -> Result<(), ffi::HResult> {
        for def in G::EventType::event_definitions() {
            let n = if let Ok(value) = CStr::from_bytes_with_nul(def.name.as_bytes()) {
                Cow::Borrowed(value)
            } else {
                Cow::Owned(CString::new(def.name).unwrap())
            };
    
            unsafe {
                let result = ffi::SimConnect_MapClientEventToSimEvent(self.raw, def.event.to_ffi(), n.as_ptr());
                if !result.is_success() {
                    println!("Error registering client event mapping");
                    return Err(result);
                }
                let result = ffi::SimConnect_AddClientEventToNotificationGroup(self.raw, group_def.group.to_ffi(), def.event.to_ffi(), def.is_maskable);
                if !result.is_success() {
                    println!("Error adding client event to a notification group");
                    return Err(result);
                }
            }
        }
        unsafe {
            let result = ffi::SimConnect_SetNotificationGroupPriority(self.raw, group_def.group.to_ffi(), group_def.priority);
            if !result.is_success() {
                println!("Error setting notification group priority");
                return Err(result);
            }
        }
        Ok(())
    }

    pub fn register_data_definition<G: DataDefinitionGroup>(&self) -> Result<(), ffi::HResult> {
        for def in G::data_definitions() {
            let n = if let Ok(value) = CStr::from_bytes_with_nul(def.name.as_bytes()) {
                Cow::Borrowed(value)
            } else {
                Cow::Owned(CString::new(def.name).unwrap())
            };

            let u = if let Ok(value) = CStr::from_bytes_with_nul(def.unit.as_bytes()) {
                Cow::Borrowed(value)
            } else {
                Cow::Owned(CString::new(def.unit).unwrap())
            };

            unsafe {
                let result = ffi::SimConnect_AddToDataDefinition(self.raw, G::group_id(), n.as_ptr(), u.as_ptr(), def.datum_type.to_ffi(), 0., UNSPECIFIED);
                if !result.is_success() {
                    println!("Error adding entry to data definition");
                    return Err(result);
                }    
            }
        }
        Ok(())
    }

    pub fn update_user_data<D: DataDefinitionGroup>(&self, data: &D) -> Result<(), ffi::HResult> {
        unsafe {
            let result = ffi::SimConnect_SetDataOnSimObject(self.raw, D::group_id(), ffi::RawObjectId::USER, ffi::DataSetFlag::Default.to_ffi(), 0, std::mem::size_of::<D>() as u32, data as *const D as *const std::ffi::c_void);
            if !result.is_success() {
                println!("Error setting data on the user object");
                return Err(result);
            }
        }
        Ok(())
    }

    pub fn dispatch<D: std::fmt::Debug + SimConnectDispatcher>(&self, dispatcher: &mut D) {
        //println!("Calling into dispatcher, inner context address: {:?}", context as *const C);

        //println!("Outer context: {:?}", raw_context);

        // println!("Dispatcher address: {:?}", dispatcher as *mut D);

        // unsafe {
        //     ffi::SimConnect_CallDispatch(self.raw, handle_dispatch_callback::<D>, dispatcher as *mut D as *mut std::ffi::c_void);
        // }

        let mut header_ptr: *const ffi::ReceiveHeader = std::ptr::null();
        let mut size = 0_u32;
        #[allow(unused_variables)]
        let mut loops = 0_usize;

        loop {
            unsafe {
                let result = ffi::SimConnect_GetNextDispatch(
                    self.raw, 
                    (&mut header_ptr) as *mut *const ffi::ReceiveHeader , 
                    &mut size as *mut u32
                );

                if !result.is_success() {
                    if result != ffi::HResult::E_FAIL {
                        println!("Error when trying to get next dispatch: {:#08x}", result.raw());
                    }
                    break;
                } else {
                    loops += 1;
                    
                    if handle_dispatch(header_ptr, size, dispatcher) == Loop::Break {
                        break;
                    }
                }
            }
        }

        //println!("Handled {} messages", loops);
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Loop {
    Continue,
    Break,
}

// extern "C" fn handle_dispatch_callback<D: std::fmt::Debug + SimConnectDispatcher>(header_ptr: *const ffi::ReceiveHeader, header_size: u32, context: *mut std::ffi::c_void) {
//     println!("Received dispatch: header: {:?}, size: {}, context: {:?}", header_ptr, header_size, context as *mut D);
//     if context.is_null() {
//         eprintln!(" Context is null");
//         return;
//     }

//     let mut dispatcher = unsafe { Box::from_raw(context as *mut D) };

//     handle_dispatch(header_ptr, header_size, dispatcher.as_mut())
// }

fn handle_dispatch<D: std::fmt::Debug + SimConnectDispatcher>(header_ptr: *const ffi::ReceiveHeader, header_size: u32, dispatcher: &mut D) -> Loop {
    if header_ptr.is_null() {
        eprintln!("Header is null");
        return Loop::Break;
    }

    let header = unsafe { std::ptr::read(header_ptr) };
    
    //println!("Header: {} {} {}", header.version, header.size, header.message_type.0);
    assert_eq!(header_size, header.size);

    //println!("Good header");

    if let Some(message_type) = ffi::MessageType::from_ffi(header.message_type) {
        unsafe { handle_dispatch_inner(header_ptr, header.size, message_type, dispatcher) }
    } else {
        println!("Unknown message type ID: {:?}", header.message_type);
        Loop::Continue
    }
}

/// ## Safety
/// 
/// Tread carefully. This is basically std::mem::transmute with a size check.
/// `ptr` is assumed to be non-null.
unsafe fn convert_with_static_size<T>(ptr: &*const ffi::ReceiveHeader, size: u32) -> &T {
    assert_eq!(std::mem::size_of::<T>(), size as usize);
    &*(*ptr as *const T)
}

/// ## Safety
/// 
/// * `header_ptr` is assumed to be non-null
unsafe fn handle_dispatch_inner<D: std::fmt::Debug + SimConnectDispatcher>(header_ptr: *const ffi::ReceiveHeader, size: u32, message_type: ffi::MessageType, dispatcher: &mut D) -> Loop {
    match message_type {
        ffi::MessageType::Null => {
            //println!("Null message, nothing to do!");
            return Loop::Break;
        }
        ffi::MessageType::Event => {
            //println!("Looks like an event!");
            let message = convert_with_static_size::<ffi::ReceiveEvent>(&header_ptr, size);
            
            //println!("Dispatching");
            println!("Event: {} {} {}", message.group_id.0, message.event_id.0, message.data);
            dispatcher.handle_event(message);
        }
        ffi::MessageType::Exception => {
            println!("Uh-oh, an exception! We don't know how to deal with these yet...");
        }
        ffi::MessageType::Open => {
            //println!("Looks like an open!");
            let message = convert_with_static_size::<ffi::ReceiveOpen>(&header_ptr, size);

            //println!("Dispatching");
            println!(
                "Connection: {} {}.{}.{}.{} ({}.{}.{}.{})", 
                message.application_name(),
                message.application_version.version_major,
                message.application_version.version_minor,
                message.application_version.build_major,
                message.application_version.build_minor,
                message.simconnect_version.version_major,
                message.simconnect_version.version_minor,
                message.simconnect_version.build_major,
                message.simconnect_version.build_minor,
            );
            dispatcher.handle_open(message);
        }
        ignored => {
            println!("Nothing to do for {:?}!", ignored);
        }
    }

    Loop::Continue
}

#[allow(unused_variables)]
pub trait SimConnectDispatcher {
    fn handle_open(&self, event: &ffi::ReceiveOpen) {}
    fn handle_event(&self, event: &ffi::ReceiveEvent) {}
}

const UNSPECIFIED: u32 = 0xFFFFFFFF;

impl Drop for SimConnect {
    fn drop(&mut self) {
        let result = unsafe { ffi::SimConnect_Close(self.raw) };
        if !result.is_success() {
            eprintln!("Failed to close SimConnect");
        }
    }
}

pub struct DataDefinition {
    pub name: &'static str,
    pub unit: &'static str,
    pub datum_type: ffi::DataType,
}

pub trait DataDefinitionGroup: Sized {
    type DataDefsIter: IntoIterator<Item = &'static DataDefinition>;
    fn group_id() -> ffi::RawDataDefinitionId;
    fn data_definitions() -> Self::DataDefsIter;
    fn register(simconnect: &SimConnect) -> Result<(), ffi::HResult> {
        simconnect.register_data_definition::<Self>()
    }
}

pub struct NotificationGroupDefinition<Group> {
    pub group: Group,
    pub priority: ffi::NotificationGroupPriority,
}

pub trait NotificationGroup: Sized + 'static {
    type GroupsIter: IntoIterator<Item = &'static NotificationGroupDefinition<Self>>;
    type EventType: EventType;
    fn to_ffi(&self) -> ffi::RawNotificationGroupId;
    fn from_ffi(raw: ffi::RawNotificationGroupId) -> Option<Self>;
    fn group_definitions() -> Self::GroupsIter;
}

pub struct EventDefinition<EventType> {
    pub event: EventType,
    pub name: &'static str,
    pub is_maskable: bool
}

pub trait EventType: Sized + 'static {
    type EventsIter: IntoIterator<Item = &'static EventDefinition<Self>>;
    fn to_ffi(&self) -> ffi::RawEventId;
    fn from_ffi(raw: ffi::RawEventId) -> Option<Self>;
    fn event_definitions() -> Self::EventsIter;
}