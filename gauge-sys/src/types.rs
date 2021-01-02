use crate::ffi;

/// A unit used with the Gauge API.
pub trait Unit {
    /// Obtains a raw FFI identifer for this unit type
    fn as_raw_unit() -> ffi::RawUnit;
}

/// Produces a new unit type, used for communicating with the gauge API.
#[macro_export]
macro_rules! gauge_unit {
    ($ty:ident: $name:literal; $description:literal) => {
        #[doc = $description]
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        pub struct $ty;

        impl $ty {
            const UNIT_NAME: &'static str = concat!($name, "\0");
        }

        impl $crate::Unit for $ty {
            #[inline]
            fn as_raw_unit() -> $crate::ffi::RawUnit {
                static RAW_UNIT_VALUE: $crate::once_cell::Lazy<$crate::ffi::RawUnit> = $crate::once_cell::Lazy::new(|| unsafe {
                    $crate::ffi::RawUnit::from_units_enum_str($ty::UNIT_NAME)
                });
                *RAW_UNIT_VALUE
            }
        }
    };
}

/// Identifies an aircraft variable that can be interacted with through the
/// Gauge API.
pub trait AircraftVariable {
    /// The Gauge API Unit
    type Unit: Unit;
    /// Obtains the raw FFI identifier for this aircraft variable
    fn as_raw_aircraft_variable() -> ffi::RawAircraftVariable;
}

/// Produces a base new aircraft variable
/// 
/// Generally you want to use `indexed_aircraft_variable` or
/// `unindexed_aircraft_variable` instead.
#[macro_export]
macro_rules! aircraft_variable {
    ($ty:ident ( $unit:ty ): $name:literal ; $description:literal) => {
        //#[doc = $description]
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        pub struct $ty;

        impl $ty {
            const VARIABLE_NAME: &'static str = concat!($name, "\0");
        }
        
        impl $crate::AircraftVariable for $ty {
            type Unit = $unit;

            #[inline]
            fn as_raw_aircraft_variable() -> $crate::ffi::RawAircraftVariable {
                static RAW_UNIT_VALUE: $crate::once_cell::Lazy<$crate::ffi::RawAircraftVariable> = $crate::once_cell::Lazy::new(|| unsafe { 
                    $crate::ffi::RawAircraftVariable::from_aircraft_variable_enum_str($ty::VARIABLE_NAME)
                });
                *RAW_UNIT_VALUE
            }
        }
    };
}

/// Provides access to an aircraft variable that is indexed
/// 
/// Many variables relating to aircraft engines will be indexed.
#[macro_export]
macro_rules! indexed_aircraft_variable {
    ($ty:ident ( $unit:ty ): $name:literal; $description:literal) => {
        $crate::aircraft_variable!($ty($unit): $name; $description);

        impl $ty {
            /// Reads the raw value from an indexed variable value for the index specified
            #[inline]
            fn read_raw_by_index(index: u32) -> f64 {
                $crate::ffi::RawAircraftVariable::read(<Self as $crate::AircraftVariable>::as_raw_aircraft_variable(), <<Self as $crate::AircraftVariable>::Unit as $crate::Unit>::as_raw_unit(), index)
            }
        }
    };
}

/// Provides access to variables that do not require indexing
#[macro_export]
macro_rules! unindexed_aircraft_variable {
    ($ty:ident ( $unit:ty ): $name:literal; $description:literal) => {
        $crate::aircraft_variable!($ty($unit): $name; $description);

        impl $ty {
            /// Reads the raw variable value
            #[inline]
            fn read_raw() -> f64 {
                $crate::ffi::RawAircraftVariable::read(<Self as $crate::AircraftVariable>::as_raw_aircraft_variable(), <<Self as $crate::AircraftVariable>::Unit as $crate::Unit>::as_raw_unit(), 0)
            }
        }
    };
}

/// A custom named variable that can be interacted with in the Gauge API
pub trait NamedVariable {
    /// The type of the variable (must be convertable to `f64`)
    type Value: Into<f64>;

    /// Obtains the raw FFI identifier for this named variable
    fn as_raw_named_variable() -> ffi::RawNamedVariable;
}

/// Constructs bindings for interacting with a named variable
#[macro_export]
macro_rules! named_variable {
    ($ty:ident ($val:ty): $name:literal; $description:literal) => {
        #[doc = $description]
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        pub struct $ty;

        impl $ty {
            const VARIABLE_NAME: &'static str = concat!($name, "\0");

            /// Sets the variable as a raw value
            #[inline]
            fn set_raw(value: <Self as $crate::NamedVariable>::Value) {
                $crate::ffi::RawNamedVariable::set(<Self as $crate::NamedVariable>::as_raw_named_variable(), value.into())
            }        
        }
        
        impl $crate::NamedVariable for $ty {
            type Value = $val;

            #[inline]
            fn as_raw_named_variable() -> $crate::ffi::RawNamedVariable {
                static RAW_UNIT_VALUE: $crate::once_cell::Lazy<$crate::ffi::RawNamedVariable> = $crate::once_cell::Lazy::new(|| unsafe {
                    $crate::ffi::RawNamedVariable::register_new($ty::VARIABLE_NAME)
                });
                *RAW_UNIT_VALUE
            }
        }
    };
}
