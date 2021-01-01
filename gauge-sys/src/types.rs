use crate::ffi;

pub trait Unit {
    fn as_raw_unit() -> ffi::RawUnit;
}

#[macro_export]
macro_rules! gauge_unit {
    ($ty:ident: $name:literal) => {
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

pub trait AircraftVariable {
    type Unit: Unit;
    fn as_raw_aircraft_variable() -> ffi::RawAircraftVariable;
}

#[macro_export]
macro_rules! aircraft_variable {
    ($ty:ident ( $unit:ty ): $name:literal) => {
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

#[macro_export]
macro_rules! indexed_aircraft_variable {
    ($ty:ident ( $unit:ty ): $name:literal) => {
        $crate::aircraft_variable!($ty($unit): $name);

        impl $ty {
            #[inline]
            fn read_raw_by_index(index: u32) -> f64 {
                $crate::ffi::RawAircraftVariable::read(<Self as $crate::AircraftVariable>::as_raw_aircraft_variable(), <<Self as $crate::AircraftVariable>::Unit as $crate::Unit>::as_raw_unit(), index)
            }
        }
    };
}

#[macro_export]
macro_rules! unindexed_aircraft_variable {
    ($ty:ident ( $unit:ty ): $name:literal) => {
        $crate::aircraft_variable!($ty($unit): $name);

        impl $ty {
            #[inline]
            fn read_raw() -> f64 {
                $crate::ffi::RawAircraftVariable::read(<Self as $crate::AircraftVariable>::as_raw_aircraft_variable(), <<Self as $crate::AircraftVariable>::Unit as $crate::Unit>::as_raw_unit(), 0)
            }
        }
    };
}


pub trait NamedVariable {
    type Value: Into<f64>;

    fn as_raw_named_variable() -> ffi::RawNamedVariable;
}

#[macro_export]
macro_rules! named_variable {
    ($ty:ident ($val:ty): $name:literal) => {
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        pub struct $ty;

        impl $ty {
            const VARIABLE_NAME: &'static str = concat!($name, "\0");

            #[inline]
            fn set(value: <Self as $crate::NamedVariable>::Value) {
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
