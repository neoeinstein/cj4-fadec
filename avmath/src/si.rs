//! Type aliases for SI units

use uom::si::f64::*;

/// Change of temperature of distance (K / m)
pub type LapseRate = <TemperatureInterval as std::ops::Div<Length>>::Output;

/// Inverse of the lapse rate (m / K)
pub type InvLapseRate = <Length as std::ops::Div<TemperatureInterval>>::Output;

/// Specific weight (kg / m²·s²)
pub type SpecificWeight = <MassDensity as std::ops::Mul<Acceleration>>::Output;

/// Unit density (1 / m³)
pub type NumberDensity = <f64 as std::ops::Div<Volume>>::Output;

/// Dynamic viscosity (kg / m·s)
pub type DynamicViscosity = <Pressure as std::ops::Mul<Time>>::Output;

/// Kinematic viscosity (m² / s)
pub type KinematicViscosity = <Area as std::ops::Div<Time>>::Output;

/// Frequency (1 / m²·s)
pub type FrequencyByArea = <Frequency as std::ops::Div<Area>>::Output;

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
