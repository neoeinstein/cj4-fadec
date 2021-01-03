use uom::{
    si::{f64::*, Quantity, ISQ, SI},
    typenum::{N1, N2, P1, Z0},
};

pub mod marker {
    mod sealed {
        pub trait Sealed {}
    }

    /// Altimeter setting for geopotential altitude above mean sea level
    pub trait QNHKind: sealed::Sealed {}

    /// Altimeter setting for geopotential altitude above an airfield
    pub trait QFEKind: sealed::Sealed {}

    /// Altimeter setting for pressure altitude above mean sea level
    pub trait StandardKind: sealed::Sealed {}
}

/// Altitude above mean sea level (base unit meter, m)
///
/// This is the ruler-measured value for altitude.
pub type GeometricAltitude = Length;

/// Altitude above mean sea level corrected for variations in gravity
pub type GeopotentialAltitude = AvailableEnergy;

// /// Altitude above mean sea level corrected for non-standard pressure
// pub type PressureAltitude = Pressure;

// /// Pressure altitude corrected for non-standard temperature and pressure
// pub type DensityAltitude = MassDensity;

/// Change of temperature over a change in geopotential altitude (s²·K / m²)
pub type LapseRate = <TemperatureInterval as std::ops::Div<GeopotentialAltitude>>::Output;

/// Change of geopotential altitude over a change in temperature (m² / s²·K)
pub type InvLapseRate = <GeopotentialAltitude as std::ops::Div<TemperatureInterval>>::Output;

/// Altimeter setting for geopotential altitude above mean sea level
pub type AltimeterQNH =
    Quantity<ISQ<N1, P1, N2, Z0, Z0, Z0, Z0, dyn marker::QNHKind>, SI<f64>, f64>;

/// Altimeter setting for geopotential altitude above an airfield
pub type AltimeterQFE =
    Quantity<ISQ<N1, P1, N2, Z0, Z0, Z0, Z0, dyn marker::QFEKind>, SI<f64>, f64>;

/// Altimeter setting for pressure altitude above mean sea level
pub type AltimeterStandard =
    Quantity<ISQ<N1, P1, N2, Z0, Z0, Z0, Z0, dyn marker::StandardKind>, SI<f64>, f64>;

macro_rules! unit {
    (
        system: $system:path;
        quantity: $quantity:path;

        $($(#[$unit_attr:meta])* @$unit:ident: $($conversion:expr),+;
            $abbreviation:expr, $singular:expr, $plural:expr;)+
    ) => {
        use $system as __system;
        use $quantity as __quantity;
        use __quantity::{Conversion, Unit};

        unit!(@units $($(#[$unit_attr])* @$unit: $($conversion),+;
            $abbreviation, $singular, $plural;)+);
    };
    (
        @units $($(#[$unit_attr:meta])* @$unit:ident: $($conversion:expr),+;
            $abbreviation:expr, $singular:expr, $plural:expr;)+
    ) => {
        $(unit!(@unit $(#[$unit_attr])* @$unit $plural);

        impl __system::Unit for $unit {
            #[inline(always)]
            fn abbreviation() -> &'static str {
                $abbreviation
            }

            #[inline(always)]
            fn singular() -> &'static str {
                $singular
            }

            #[inline(always)]
            fn plural() -> &'static str {
                $plural
            }
        }

        impl Unit for $unit {})+

        storage_types! {
            types: Float;

            $(impl uom::Conversion<V> for super::$unit {
                type T = V;

                #[inline(always)]
                #[allow(clippy::inconsistent_digit_grouping)]
                fn coefficient() -> Self::T {
                    unit!(@coefficient $($conversion),+)
                }

                #[inline(always)]
                #[allow(unused_variables)]
                #[allow(clippy::inconsistent_digit_grouping)]
                fn constant(op: uom::ConstantOp) -> Self::T {
                    unit!(@constant op $($conversion),+)
                }
            }

            impl super::Conversion<V> for super::$unit {})+
        }

        storage_types! {
            types: PrimInt, BigInt;
            pub type T = $crate::num::rational::Ratio<V>;

            #[inline(always)]
            fn from_f64(value: f64) -> T {
                <T as $crate::num::FromPrimitive>::from_f64(value).unwrap()
            }

            $(impl $crate::Conversion<V> for super::$unit {
                type T = T;

                #[inline(always)]
                fn coefficient() -> Self::T {
                    from_f64(unit!(@coefficient $($conversion),+))
                }

                #[inline(always)]
                #[allow(unused_variables)]
                fn constant(op: $crate::ConstantOp) -> Self::T {
                    from_f64(unit!(@constant op $($conversion),+))
                }
            }

            impl super::Conversion<V> for super::$unit {})+
        }

        storage_types! {
            types: BigUint;
            pub type T = $crate::num::rational::Ratio<V>;

            #[inline(always)]
            fn from_f64(value: f64) -> T {
                use $crate::num::FromPrimitive;

                let c = $crate::num::rational::Ratio::<$crate::num::BigInt>::from_f64(value)
                    .unwrap();

                T::new(c.numer().to_biguint().unwrap(), c.denom().to_biguint().unwrap())
            }

            $(impl $crate::Conversion<V> for super::$unit {
                type T = T;

                #[inline(always)]
                fn coefficient() -> Self::T {
                    from_f64(unit!(@coefficient $($conversion),+))
                }

                #[inline(always)]
                #[allow(unused_variables)]
                fn constant(op: $crate::ConstantOp) -> Self::T {
                    from_f64(unit!(@constant op $($conversion),+))
                }
            }

            impl super::Conversion<V> for super::$unit {})+
        }

        storage_types! {
            types: Ratio;

            #[inline(always)]
            fn from_f64(value: f64) -> V {
                <V as $crate::num::FromPrimitive>::from_f64(value).unwrap()
            }

            $(impl $crate::Conversion<V> for super::$unit {
                type T = V;

                #[inline(always)]
                fn coefficient() -> Self::T {
                    from_f64(unit!(@coefficient $($conversion),+))
                }

                #[inline(always)]
                #[allow(unused_variables)]
                fn constant(op: $crate::ConstantOp) -> Self::T {
                    from_f64(unit!(@constant op $($conversion),+))
                }
            }

            impl super::Conversion<V> for super::$unit {})+
        }
    };
    (@unit $(#[$unit_attr:meta])+ @$unit:ident $plural:expr) => {
        $(#[$unit_attr])*
        #[allow(non_camel_case_types)]
        #[derive(Clone, Copy, Debug, Hash)]
        pub struct $unit;
    };
    (@unit @$unit:ident $plural:expr) => {
        #[doc = $plural]
        #[allow(non_camel_case_types)]
        #[derive(Clone, Copy, Debug, Hash)]
        pub struct $unit;
    };
    (@coefficient $factor:expr, $const:expr) => { $factor };
    (@coefficient $factor:expr) => { $factor };
    (@constant $op:ident $factor:expr, $const:expr) => { $const };
    (@constant $op:ident $factor:expr) => {
        match $op {
            uom::ConstantOp::Add => -0.0,
            uom::ConstantOp::Sub => 0.0,
        }
    };
}

pub mod geometric_altitude {
    use super::geopotential_altitude;
    use super::{GeometricAltitude, GeopotentialAltitude};

    unit! {
        system: uom::si;
        quantity: uom::si::length;

        @meter: 1.0; "m MSL", "meter above mean sea level", "meters above mean sea level";
        @kilometer: 1.0e3; "km MSL", "kilometer above mean sea level", "kilometers above mean sea level";
        @foot: 3.048_E-1; "ft MSL", "foot above mean sea level", "feet above mean sea level";
        @statute_mile: 3.048_E-1 * 5_280.; "mi MSL", "mile above mean sea level", "miles above mean sea level";
    }

    pub trait GeometricAltitudeExtensions: Sized {
        fn from_geopotential_altitude(alt: GeopotentialAltitude) -> Self;
        fn into_geopotential_altitude(self) -> GeopotentialAltitude;
    }

    impl GeometricAltitudeExtensions for GeometricAltitude {
        fn from_geopotential_altitude(alt: GeopotentialAltitude) -> Self {
            GeometricAltitude::new::<meter>(
                crate::constants::earth_radius().get::<meter>()
                    * alt.get::<geopotential_altitude::meter>()
                    / (crate::constants::earth_radius().get::<meter>()
                        - alt.get::<geopotential_altitude::meter>()),
            )
        }

        fn into_geopotential_altitude(self) -> GeopotentialAltitude {
            GeopotentialAltitude::new::<geopotential_altitude::meter>(
                crate::constants::earth_radius().get::<meter>() * self.get::<meter>()
                    / (crate::constants::earth_radius().get::<meter>() + self.get::<meter>()),
            )
        }
    }

    // /// Given an altimeter setting, produces the pressure measured by a
    // /// station at this altitude
    // fn to_pressure(self, altimeter: super::quantities::QNH<V>) -> Option<Pressure> {
    //     let layer = Layer::find_by_altitude(self)?;

    //     let lapse_rate = layer.lapse_rate.unwrap_or_default();
    //     let k1 = (-constants::Rd_over_standard_gravity_msl() * -lapse_rate).get::<ratio>();
    //     let k2 = (lapse_rate * self.0 / layer.base_temperature).get::<ratio>();

    //     Some(Pressure::new::<pascal>(
    //         (altimeter.remove_context().get::<pascal>().powf(k1)
    //             + layer.pressure.start.get::<pascal>().powf(k1) * k2)
    //             .powf(k1.recip()),
    //     ))
    // }

    // /// Using the method used by ASOS stations and given an altimeter setting,
    // /// produces the pressure measured by an ASOS station at this altitude
    // fn to_pressure_asos(self, altimeter: QNH) -> Pressure {
    //     Pressure::new::<inch_of_mercury>(
    //         (altimeter
    //             .remove_context()
    //             .get::<inch_of_mercury>()
    //             .powf(0.1903)
    //             - (1.313e-5 * self.0.get::<foot>()))
    //         .powf(5.255),
    //     )
    // }

    // /// Using the method used by ASOS stations, finds the altimeter setting
    // /// for an ASOS station at this altitude
    // fn to_altimeter_setting_asos(self, pressure: Pressure) -> QNH {
    //     QNH::new::<inch_of_mercury>(
    //         (pressure.get::<inch_of_mercury>().powf(0.1903) + (1.313e-5 * self.0.get::<foot>()))
    //             .powf(5.255),
    //     )
    // }
}

pub use geometric_altitude::GeometricAltitudeExtensions;

pub mod geopotential_altitude {
    use super::geometric_altitude::GeometricAltitudeExtensions as _;
    use super::{GeometricAltitude, GeopotentialAltitude};

    unit! {
        system: uom::si;
        quantity: uom::si::available_energy;

        @meter: 9.806_65; "m MSL", "meter above mean sea level", "meters above mean sea level";
        @kilometer: 9.806_65E3; "km MSL", "kilometer above mean sea level", "kilometers above mean sea level";
        @foot: 9.806_65 * 3.048_E-1; "ft MSL", "foot above mean sea level", "feet above mean sea level";
        @statute_mile: 9.806_65 * 3.048_E-1 * 5_280.; "mi MSL", "mile above mean sea level", "miles above mean sea level";
    }

    pub trait GeopotentialAltitudeExtensions: Sized {
        fn from_geometric_altitude(alt: GeometricAltitude) -> Self;
        fn into_geometric_altitude(self) -> GeometricAltitude;
    }

    impl GeopotentialAltitudeExtensions for GeopotentialAltitude {
        fn from_geometric_altitude(alt: GeometricAltitude) -> Self {
            alt.into_geopotential_altitude()
        }

        fn into_geometric_altitude(self) -> GeometricAltitude {
            GeometricAltitude::from_geopotential_altitude(self)
        }
    }
}

pub use geopotential_altitude::GeopotentialAltitudeExtensions;
