//! Implementations of units based on the International Standard of Units
//! using quantities relevant to the field of aviation

use crate::constants;
use uom::si::f64::*;
use uom::si::{
    length::foot,
    pressure::{inch_of_mercury, pascal},
    ratio::ratio,
};

mod atmosphere;
#[cfg(feature = "experimental")]
mod experimental;

pub use atmosphere::Layer;

/// Altitude above mean sea level
///
/// This is the ruler-measured value for altitude. Calculations for standard
/// atmosphere may require conversion into a [`GeopotentialAltitude`].
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct GeometricAltitude(Length);

impl GeometricAltitude {
    /// Constructs a new altitude
    #[inline(always)]
    pub fn new<N>(v: f64) -> Self
    where
        N: uom::si::length::Unit + uom::Conversion<f64, T = f64>,
    {
        Self(Length::new::<N>(v))
    }

    /// Retrieve the value in the requested measurement unit
    #[inline(always)]
    pub fn get<N>(self) -> f64
    where
        N: uom::si::length::Unit + uom::Conversion<f64, T = f64>,
    {
        self.0.get::<N>()
    }

    /// Creates a struct that can be used to format a compatible quantity for display
    #[inline(always)]
    pub fn format_args<N>(
        unit: N,
        style: uom::fmt::DisplayStyle,
    ) -> uom::si::fmt::Arguments<uom::si::length::Dimension, N>
    where
        N: uom::si::length::Unit,
    {
        Length::format_args(unit, style)
    }

    /// Creates a struct that can be used to format a compatible quantity for display
    #[inline(always)]
    pub fn into_format_args<N>(
        self,
        unit: N,
        style: uom::fmt::DisplayStyle,
    ) -> uom::si::fmt::QuantityArguments<uom::si::length::Dimension, uom::si::SI<f64>, f64, N>
    where
        N: uom::si::length::Unit,
    {
        self.0.into_format_args(unit, style)
    }

    /// Reinterprets a length as a geometric altitude
    #[inline(always)]
    pub fn interpret(length: Length) -> Self {
        Self(length)
    }

    /// Removes the context that this value refers to a geometric altitude
    #[inline(always)]
    pub fn remove_context(self) -> Length {
        self.0
    }
}

impl std::ops::Add for GeometricAltitude {
    type Output = Self;
    #[inline(always)]
    fn add(self, rhs: Self) -> Self::Output {
        GeometricAltitude(self.0 + rhs.0)
    }
}

impl std::ops::Sub for GeometricAltitude {
    type Output = GeometricAltitude;
    #[inline(always)]
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl std::ops::Mul<Ratio> for GeometricAltitude {
    type Output = Self;
    #[inline(always)]
    fn mul(self, rhs: Ratio) -> Self::Output {
        Self(self.0 * rhs.get::<ratio>())
    }
}

impl std::ops::Mul<GeometricAltitude> for Ratio {
    type Output = GeometricAltitude;
    #[inline(always)]
    fn mul(self, rhs: GeometricAltitude) -> Self::Output {
        GeometricAltitude(self.get::<ratio>() * rhs.0)
    }
}

impl std::ops::Div for GeometricAltitude {
    type Output = Ratio;
    #[inline(always)]
    fn div(self, rhs: Self) -> Self::Output {
        self.0 / rhs.0
    }
}

impl std::ops::Div<Ratio> for GeometricAltitude {
    type Output = Self;
    #[inline(always)]
    fn div(self, rhs: Ratio) -> Self::Output {
        Self(self.0 / rhs.get::<ratio>())
    }
}
/// Height above mean sea level corrected for variations variations in gravity
///
/// Most standard calculations are based on geopotential altitudes. To obtain
/// actual linear distances, convert to a [`GeometricAltitude`] prior to
/// use as a raw length.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct GeopotentialAltitude(Length);

impl GeopotentialAltitude {
    /// Constructs a new gravity-corrected altitude
    #[inline(always)]
    pub fn new<N>(v: f64) -> Self
    where
        N: uom::si::length::Unit + uom::Conversion<f64, T = f64>,
    {
        Self(Length::new::<N>(v))
    }

    /// Retrieve the value in the requested measurement unit
    #[inline(always)]
    pub fn get<N>(self) -> f64
    where
        N: uom::si::length::Unit + uom::Conversion<f64, T = f64>,
    {
        self.0.get::<N>()
    }

    /// Creates a struct that can be used to format a compatible quantity for display
    #[inline(always)]
    pub fn format_args<N>(
        unit: N,
        style: uom::fmt::DisplayStyle,
    ) -> uom::si::fmt::Arguments<uom::si::length::Dimension, N>
    where
        N: uom::si::length::Unit,
    {
        Length::format_args(unit, style)
    }

    /// Creates a struct that can be used to format a compatible quantity for display
    #[inline(always)]
    pub fn into_format_args<N>(
        self,
        unit: N,
        style: uom::fmt::DisplayStyle,
    ) -> uom::si::fmt::QuantityArguments<uom::si::length::Dimension, uom::si::SI<f64>, f64, N>
    where
        N: uom::si::length::Unit,
    {
        self.0.into_format_args(unit, style)
    }

    /// Interprets a raw length as a gravity-corrected altitude over mean sea level
    #[inline(always)]
    pub fn interpret(length: Length) -> Self {
        Self(length)
    }

    /// Removes the context that this is a gravity-corrected altitude over mean sea level
    #[inline(always)]
    pub fn remove_context(self) -> Length {
        self.0
    }

    // pub fn to_pressure_altitude(self, altimeter: AltimeterSetting) -> Option<PressureAltitude> {
    //     //let layer = isa::layer_at_pressure(altimeter.remove_context())?;
    //     let layer = isa::layer_at_altitude(PressureAltitude::new::<foot>(0.))?;
    //     let std_temperature = isa::standard_temperature();

    //     let relative_pressure = dbg!(altimeter.remove_context() / layer.base_pressure);
    //     let relative_temperature = dbg!(layer.base_temperature / std_temperature);

    //     let relative_pressure_temperature = relative_pressure * relative_temperature;

    //     if let Some(lapse_rate) = layer.lapse_rate {
    //         let temperature_height: Length = dbg!(layer.base_temperature / lapse_rate);

    //         let pressure_exp_m1 = dbg!(lapse_rate * *isa::NEG_R_OVER_standard_gravity_msl).get::<ratio>();
    //         let temp_ratio = dbg!(1.0_f64 - relative_pressure_temperature.get::<ratio>().powf(pressure_exp_m1));

    //         let layer_height: Length = dbg!(temp_ratio * temperature_height);
    //         Some(PressureAltitude(layer_height + layer.base_altitude.remove_context()))
    //     } else {
    //         let pressure_exp_m1 = relative_pressure_temperature.get::<ratio>();
    //         let temp_ratio = pressure_exp_m1.ln();
    //         let height_gradient: Length = layer.base_temperature * *isa::NEG_R_OVER_standard_gravity_msl;
    //         let layer_height: Length = height_gradient * temp_ratio;
    //         Some(PressureAltitude(layer_height + layer.base_altitude.remove_context()))
    //     }
    // }

    /// Given an altimeter setting, produces the pressure measured by a
    /// station at this altitude
    #[doc(hidden)]
    pub fn to_pressure(self, altimeter: AltimeterSetting) -> Option<Pressure> {
        let layer = Layer::find_by_altitude(self)?;

        let lapse_rate = layer.lapse_rate.unwrap_or_default();
        let k1 = (-constants::Rd_over_standard_gravity_msl() * -lapse_rate).get::<ratio>();
        let k2 = (lapse_rate * self.0 / layer.base_temperature).get::<ratio>();

        Some(Pressure::new::<pascal>(
            (altimeter.remove_context().get::<pascal>().powf(k1)
                + layer.pressure.start.get::<pascal>().powf(k1) * k2)
                .powf(k1.recip()),
        ))
    }

    /// Using the method used by ASOS stations and given an altimeter setting,
    /// produces the pressure measured by an ASOS station at this altitude
    pub fn to_pressure_asos(self, altimeter: AltimeterSetting) -> Pressure {
        Pressure::new::<inch_of_mercury>(
            (altimeter
                .remove_context()
                .get::<inch_of_mercury>()
                .powf(0.1903)
                - (1.313e-5 * self.0.get::<foot>()))
            .powf(5.255),
        )
    }

    /// Using the method used by ASOS stations, finds the altimeter setting
    /// for an ASOS station at this altitude
    pub fn to_altimeter_setting_asos(self, pressure: Pressure) -> AltimeterSetting {
        AltimeterSetting::new::<inch_of_mercury>(
            (pressure.get::<inch_of_mercury>().powf(0.1903) + (1.313e-5 * self.0.get::<foot>()))
                .powf(5.255),
        )
    }
}

impl std::ops::Add for GeopotentialAltitude {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        GeopotentialAltitude(self.0 + rhs.0)
    }
}

impl std::ops::Sub for GeopotentialAltitude {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl std::ops::Mul<Ratio> for GeopotentialAltitude {
    type Output = Self;
    fn mul(self, rhs: Ratio) -> Self::Output {
        Self(self.0 * rhs.get::<ratio>())
    }
}

impl std::ops::Mul<GeopotentialAltitude> for Ratio {
    type Output = GeopotentialAltitude;
    fn mul(self, rhs: GeopotentialAltitude) -> Self::Output {
        GeopotentialAltitude(self.get::<ratio>() * rhs.0)
    }
}

impl std::ops::Div for GeopotentialAltitude {
    type Output = Ratio;
    fn div(self, rhs: Self) -> Self::Output {
        self.0 / rhs.0
    }
}

impl std::ops::Div<Ratio> for GeopotentialAltitude {
    type Output = Self;
    fn div(self, rhs: Ratio) -> Self::Output {
        Self(self.0 / rhs.get::<ratio>())
    }
}

impl From<GeometricAltitude> for GeopotentialAltitude {
    fn from(alt: GeometricAltitude) -> Self {
        Self::interpret(
            constants::earth_radius() * alt.remove_context()
                / (constants::earth_radius() + alt.remove_context()),
        )
    }
}

impl From<GeopotentialAltitude> for GeometricAltitude {
    fn from(alt: GeopotentialAltitude) -> Self {
        Self::interpret(
            constants::earth_radius() * alt.remove_context()
                / (constants::earth_radius() - alt.remove_context()),
        )
    }
}

/// Altitude above mean sea level corrected for non-standard pressure
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct PressureAltitude(Length);

impl PressureAltitude {
    /// Constructs a new pressure-corrected altitude
    #[inline(always)]
    pub fn new<N>(v: f64) -> Self
    where
        N: uom::si::length::Unit + uom::Conversion<f64, T = f64>,
    {
        Self(Length::new::<N>(v))
    }

    /// Retrieve the value in the requested measurement unit
    #[inline(always)]
    pub fn get<N>(self) -> f64
    where
        N: uom::si::length::Unit + uom::Conversion<f64, T = f64>,
    {
        self.0.get::<N>()
    }

    /// Creates a struct that can be used to format a compatible quantity for display
    #[inline(always)]
    pub fn format_args<N>(
        unit: N,
        style: uom::fmt::DisplayStyle,
    ) -> uom::si::fmt::Arguments<uom::si::length::Dimension, N>
    where
        N: uom::si::length::Unit,
    {
        Length::format_args(unit, style)
    }

    /// Creates a struct that formats `self` for display.
    #[inline(always)]
    pub fn into_format_args<N>(
        self,
        unit: N,
        style: uom::fmt::DisplayStyle,
    ) -> uom::si::fmt::QuantityArguments<uom::si::length::Dimension, uom::si::SI<f64>, f64, N>
    where
        N: uom::si::length::Unit,
    {
        self.0.into_format_args(unit, style)
    }

    /// Interprets a raw length as a pressure-corrected altitude over mean sea level
    #[inline(always)]
    pub fn interpret(length: Length) -> Self {
        Self(length)
    }

    /// Removes the context that this is a pressure-corrected altitude over mean sea level
    #[inline(always)]
    pub fn remove_context(self) -> Length {
        self.0
    }
}

impl std::ops::Sub for PressureAltitude {
    type Output = Length;
    fn sub(self, rhs: Self) -> Self::Output {
        self.0 - rhs.0
    }
}

/// Pressure altitude corrected for non-standard temperature and pressure
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct DensityAltitude(Length);

impl DensityAltitude {
    /// Constructs a new pressure- and temperature-corrected altitude
    #[inline(always)]
    pub fn new<N>(v: f64) -> Self
    where
        N: uom::si::length::Unit + uom::Conversion<f64, T = f64>,
    {
        Self(Length::new::<N>(v))
    }

    /// Retrieve the value in the requested measurement unit
    #[inline(always)]
    pub fn get<N>(self) -> f64
    where
        N: uom::si::length::Unit + uom::Conversion<f64, T = f64>,
    {
        self.0.get::<N>()
    }

    /// Creates a struct that can be used to format a compatible quantity for display
    #[inline(always)]
    pub fn format_args<N>(
        unit: N,
        style: uom::fmt::DisplayStyle,
    ) -> uom::si::fmt::Arguments<uom::si::length::Dimension, N>
    where
        N: uom::si::length::Unit,
    {
        Length::format_args(unit, style)
    }

    /// Creates a struct that formats `self` for display.
    #[inline(always)]
    pub fn into_format_args<N>(
        self,
        unit: N,
        style: uom::fmt::DisplayStyle,
    ) -> uom::si::fmt::QuantityArguments<uom::si::length::Dimension, uom::si::SI<f64>, f64, N>
    where
        N: uom::si::length::Unit,
    {
        self.0.into_format_args(unit, style)
    }

    /// Interprets a raw length as a pressure- and temperature-corrected altitude over mean sea level
    #[inline(always)]
    pub fn interpret(length: Length) -> Self {
        Self(length)
    }

    /// Removes the context that this is a pressure- and temperature-corrected altitude over mean sea level
    #[inline(always)]
    pub fn remove_context(self) -> Length {
        self.0
    }
}

impl std::ops::Sub for DensityAltitude {
    type Output = Length;
    fn sub(self, rhs: Self) -> Self::Output {
        self.0 - rhs.0
    }
}

/// Altimeter setting
///
/// An altimeter set to the QNH value will display
/// the current geopotential altitude above mean sea level.
///
/// An altimeter set to standard pressure (29.92 inHg / 1013.25 hPa) will
/// display the current pressure altitude.
///
/// An altimeter set to the QFE value for an airfield will display the
/// current geopotential altitude above the airfield.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct AltimeterSetting(Pressure);

impl AltimeterSetting {
    /// Constructs a new altimeter setting
    #[inline(always)]
    pub fn new<N>(v: f64) -> Self
    where
        N: uom::si::pressure::Unit + uom::Conversion<f64, T = f64>,
    {
        Self(Pressure::new::<N>(v))
    }

    /// Interprets the pressure value provided as an altimeter setting
    #[inline(always)]
    pub fn interpret(pressure: Pressure) -> Self {
        Self(pressure)
    }

    /// Retrieve the value in the requested measurement unit
    #[inline(always)]
    pub fn get<N>(self) -> f64
    where
        N: uom::si::pressure::Unit + uom::Conversion<f64, T = f64>,
    {
        self.0.get::<N>()
    }

    /// Creates a struct that can be used to format a compatible quantity for display
    #[inline(always)]
    pub fn format_args<N>(
        unit: N,
        style: uom::fmt::DisplayStyle,
    ) -> uom::si::fmt::Arguments<uom::si::pressure::Dimension, N>
    where
        N: uom::si::pressure::Unit,
    {
        Pressure::format_args(unit, style)
    }

    /// Creates a struct that formats `self` for display.
    #[inline(always)]
    pub fn into_format_args<N>(
        self,
        unit: N,
        style: uom::fmt::DisplayStyle,
    ) -> uom::si::fmt::QuantityArguments<uom::si::pressure::Dimension, uom::si::SI<f64>, f64, N>
    where
        N: uom::si::pressure::Unit,
    {
        self.0.into_format_args(unit, style)
    }

    /// Removes the context that this is an altimeter setting
    #[inline(always)]
    pub fn remove_context(self) -> Pressure {
        self.0
    }
}
