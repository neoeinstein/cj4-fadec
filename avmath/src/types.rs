use uom::{si::{f64::*, ratio::ratio, pressure::{pascal, inch_of_mercury, self}, length::{foot, self}}, Conversion};
use crate::{atmosphere::Layer, constants};

pub type LapseRate = <TemperatureInterval as std::ops::Div<Length>>::Output;
pub type InvLapseRate = <Length as std::ops::Div<TemperatureInterval>>::Output;
pub type SpecificWeight = <MassDensity as std::ops::Mul<Acceleration>>::Output;
pub type NumberDensity = <f64 as std::ops::Div<Volume>>::Output;
pub type DynamicViscosity = <Pressure as std::ops::Mul<Time>>::Output;
pub type KinematicViscosity = <Area as std::ops::Div<Time>>::Output;
pub type FrequencyByArea = <Frequency as std::ops::Div<Area>>::Output;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct GeopotentialAltitude(Length);

impl GeopotentialAltitude {
    #[inline]
    pub fn new<N>(v: f64) -> Self
    where
        N: length::Unit + Conversion<f64, T = f64>,
    {
        Self(Length::new::<N>(v))
    }

    #[inline]
    pub fn interpret(length: Length) -> Self {
        Self(length)
    }

    #[inline]
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

    //         let pressure_exp_m1 = dbg!(lapse_rate * *isa::NEG_R_OVER_g0).get::<ratio>();
    //         let temp_ratio = dbg!(1.0_f64 - relative_pressure_temperature.get::<ratio>().powf(pressure_exp_m1));

    //         let layer_height: Length = dbg!(temp_ratio * temperature_height);
    //         Some(PressureAltitude(layer_height + layer.base_altitude.remove_context()))
    //     } else {
    //         let pressure_exp_m1 = relative_pressure_temperature.get::<ratio>();
    //         let temp_ratio = pressure_exp_m1.ln();
    //         let height_gradient: Length = layer.base_temperature * *isa::NEG_R_OVER_g0;
    //         let layer_height: Length = height_gradient * temp_ratio;
    //         Some(PressureAltitude(layer_height + layer.base_altitude.remove_context()))
    //     }
    // }

    pub fn to_pressure(self, altimeter: AltimeterSetting) -> Option<Pressure> {
        let layer = Layer::find_by_altitude(self)?;

        let lapse_rate = layer.lapse_rate.unwrap_or_default();
        let k1 = (-constants::R_over_g0() * -lapse_rate).get::<ratio>();
        let k2 = (lapse_rate * self.0 / layer.base_temperature).get::<ratio>();

        Some(Pressure::new::<pascal>((altimeter.remove_context().get::<pascal>().powf(k1) + layer.base_pressure.get::<pascal>().powf(k1) * k2).powf(k1.recip())))
    }

    pub fn to_pressure_asos(self, altimeter: AltimeterSetting) -> Pressure {
        Pressure::new::<inch_of_mercury>((altimeter.remove_context().get::<inch_of_mercury>().powf(0.1903) - (1.313e-5 * self.0.get::<foot>())).powf(5.255))
    }

    pub fn to_altimeter_setting_asos(self, pressure: Pressure) -> AltimeterSetting {
        AltimeterSetting::new::<inch_of_mercury>((pressure.get::<inch_of_mercury>().powf(0.1903) + (1.313e-5 * self.0.get::<foot>())).powf(5.255))
    }
}

impl std::ops::Add<Length> for GeopotentialAltitude {
    type Output = GeopotentialAltitude;
    fn add(self, rhs: Length) -> Self::Output {
        GeopotentialAltitude(self.0 + rhs)
    }
}

impl std::ops::Add<GeopotentialAltitude> for Length {
    type Output = GeopotentialAltitude;
    fn add(self, rhs: GeopotentialAltitude) -> Self::Output {
        GeopotentialAltitude(self + rhs.0)
    }
}

impl std::ops::Sub for GeopotentialAltitude {
    type Output = Length;
    fn sub(self, rhs: Self) -> Self::Output {
        self.0 - rhs.0
    }
}

impl std::ops::Div for GeopotentialAltitude {
    type Output = Ratio;
    fn div(self, rhs: Self) -> Self::Output {
        self.0 / rhs.0
    }
}

impl std::ops::Div<GeopotentialAltitude> for Length {
    type Output = Ratio;
    fn div(self, rhs: GeopotentialAltitude) -> Self::Output {
        self / rhs.0
    }
}


#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct GeometricAltitude(Length);

impl GeometricAltitude {
    #[inline]
    pub fn new<N>(v: f64) -> Self
    where
        N: length::Unit + Conversion<f64, T = f64>,
    {
        Self(Length::new::<N>(v))
    }

    #[inline]
    pub fn interpret(length: Length) -> Self {
        Self(length)
    }

    #[inline]
    pub fn remove_context(self) -> Length {
        self.0
    }
}

impl std::ops::Add<Length> for GeometricAltitude {
    type Output = GeometricAltitude;
    fn add(self, rhs: Length) -> Self::Output {
        GeometricAltitude(self.0 + rhs)
    }
}

impl std::ops::Add<GeometricAltitude> for Length {
    type Output = GeometricAltitude;
    fn add(self, rhs: GeometricAltitude) -> Self::Output {
        GeometricAltitude(self + rhs.0)
    }
}

impl std::ops::Sub for GeometricAltitude {
    type Output = Length;
    fn sub(self, rhs: Self) -> Self::Output {
        self.0 - rhs.0
    }
}

impl std::ops::Div for GeometricAltitude {
    type Output = Ratio;
    fn div(self, rhs: Self) -> Self::Output {
        self.0 / rhs.0
    }
}

impl std::ops::Div<GeometricAltitude> for Length {
    type Output = Ratio;
    fn div(self, rhs: GeometricAltitude) -> Self::Output {
        self / rhs.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct PressureAltitude(Length);

impl PressureAltitude {
    #[inline]
    pub fn new<N>(v: f64) -> Self
    where
        N: length::Unit + Conversion<f64, T = f64>,
    {
        Self(Length::new::<N>(v))
    }

    #[inline]
    pub fn interpret(length: Length) -> Self {
        Self(length)
    }

    #[inline]
    pub fn remove_context(self) -> Length {
        self.0
    }
}

impl std::ops::Add<Length> for PressureAltitude {
    type Output = PressureAltitude;
    fn add(self, rhs: Length) -> Self::Output {
        PressureAltitude(self.0 + rhs)
    }
}

impl std::ops::Add<PressureAltitude> for Length {
    type Output = PressureAltitude;
    fn add(self, rhs: PressureAltitude) -> Self::Output {
        PressureAltitude(self + rhs.0)
    }
}

impl std::ops::Sub for PressureAltitude {
    type Output = Length;
    fn sub(self, rhs: Self) -> Self::Output {
        self.0 - rhs.0
    }
}

impl std::ops::Div for PressureAltitude {
    type Output = Ratio;
    fn div(self, rhs: Self) -> Self::Output {
        self.0 / rhs.0
    }
}

impl std::ops::Div<PressureAltitude> for Length {
    type Output = Ratio;
    fn div(self, rhs: PressureAltitude) -> Self::Output {
        self / rhs.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct AltimeterSetting(Pressure);

impl AltimeterSetting {
    #[inline]
    pub fn new<N>(v: f64) -> Self
    where
        N: pressure::Unit + Conversion<f64, T = f64>,
    {
        Self(Pressure::new::<N>(v))
    }

    #[inline]
    pub fn interpret(pressure: Pressure) -> Self {
        Self(pressure)
    }

    #[inline]
    pub fn remove_context(self) -> Pressure {
        self.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct DensityAltitude(Length);

impl DensityAltitude {
    #[inline]
    pub fn new<N>(v: f64) -> Self
    where
        N: uom::si::length::Unit + uom::Conversion<f64, T = f64>,
    {
        Self(Length::new::<N>(v))
    }

    #[inline]
    pub fn interpret(length: Length) -> Self {
        Self(length)
    }

    #[inline]
    pub fn remove_context(self) -> Length {
        self.0
    }
}

impl std::ops::Add<Length> for DensityAltitude {
    type Output = DensityAltitude;
    fn add(self, rhs: Length) -> Self::Output {
        DensityAltitude(self.0 + rhs)
    }
}

impl std::ops::Add<DensityAltitude> for Length {
    type Output = DensityAltitude;
    fn add(self, rhs: DensityAltitude) -> Self::Output {
        DensityAltitude(self + rhs.0)
    }
}

impl std::ops::Sub for DensityAltitude {
    type Output = Length;
    fn sub(self, rhs: Self) -> Self::Output {
        self.0 - rhs.0
    }
}

impl std::ops::Div for DensityAltitude {
    type Output = Ratio;
    fn div(self, rhs: Self) -> Self::Output {
        self.0 / rhs.0
    }
}

impl std::ops::Div<DensityAltitude> for Length {
    type Output = Ratio;
    fn div(self, rhs: DensityAltitude) -> Self::Output {
        self / rhs.0
    }
}
