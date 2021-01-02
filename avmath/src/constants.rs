//! Constant values used as the basis for many calculations

#![allow(non_snake_case)]

use crate::types::{InvLapseRate, LapseRate};
use std::ops::Div;
use uom::si::{
    acceleration::meter_per_second_squared,
    amount_of_substance::kilomole,
    available_energy::joule_per_kilogram,
    f64::*,
    length::meter,
    mass_density::kilogram_per_cubic_meter,
    molar_energy::joule_per_mole,
    molar_mass::kilogram_per_mole,
    pressure::hectopascal,
    ratio::ratio,
    specific_heat_capacity::joule_per_kilogram_kelvin,
    temperature_interval::kelvin as diff_kelvin,
    thermodynamic_temperature::{degree_celsius as celsius, kelvin},
    velocity::meter_per_second,
};

/// Avogadro's number (N<sub>A</sub>)
#[inline(always)]
pub fn avogadros_number() -> <f64 as Div<AmountOfSubstance>>::Output {
    602.257_e24 / AmountOfSubstance::new::<kilomole>(1.)
}

/// Molecular weight of dry air (M<sub>d</sub> or M)
///
/// Identified as M₀ in the ICAO manual
#[inline(always)]
pub fn Md() -> MolarMass {
    MolarMass::new::<kilogram_per_mole>(28.964_420_e-3)
}

/// Molecular weight of water vapor (M<sub>v</sub>)
#[inline(always)]
pub fn Mv() -> MolarMass {
    MolarMass::new::<kilogram_per_mole>(18.01528_e-3)
}

/// Ideal gas constant (R)
///
/// Identified as R<sup>*</sup> in the ICAO manual
#[inline(always)]
pub fn R() -> <MolarEnergy as Div<ThermodynamicTemperature>>::Output {
    // Latest definition says this should be _exactly_ 8.31446261815324
    MolarEnergy::new::<joule_per_mole>(8.31432) / ThermodynamicTemperature::new::<kelvin>(1.)
}

/// Specific heat capacity of dry air (R<sub>d</sub> / M<sub>d</sub>)
#[inline(always)]
pub fn Rd() -> SpecificHeatCapacity {
    SpecificHeatCapacity::new::<joule_per_kilogram_kelvin>(287.052_87)
}

/// Lapse rate of dry air due to the effects of gravity (g₀ / R<sub>d</sub>)
#[inline(always)]
pub fn standard_gravity_msl_over_Rd() -> LapseRate {
    standard_gravity_msl() / Rd()
}

/// Inverse lapse rate of dry air due to the effects of gravity (R<sub>d</sub> / g₀)
#[inline(always)]
pub fn Rd_over_standard_gravity_msl() -> InvLapseRate {
    Rd() / standard_gravity_msl()
}

/// Specific heat capacity of water vapor (R<sub>v</sub> / M<sub>v</sub>)
#[inline(always)]
pub fn Rv() -> SpecificHeatCapacity {
    R() / Mv()
}

/// Standard temperature of the freezing point of water at mean sea level (T<sub>i</sub>)
#[inline(always)]
pub fn water_freezing_point_msl() -> ThermodynamicTemperature {
    ThermodynamicTemperature::new::<kelvin>(273.15)
}

/// Dry air constant for dynamic viscosity calculations (β<sub>s</sub>) in kg / m·s·K<sup>½</sup>
///
/// Used for the calculation of dynamic viscosity. This value is provided
/// without units assigned beacuse the exponent is not possible to model
/// in the dimension scheme utilized. Thus, units support
/// is not provided on this constant.
#[inline(always)]
pub fn Bs() -> f64 {
    1.458_e-6
}

/// The Sutherland temperature (S)
///
/// Used for the calculation of dynamic viscosity.
#[inline(always)]
pub fn S() -> TemperatureInterval {
    TemperatureInterval::new::<diff_kelvin>(110.4)
}

/// Adiabatic index of dry air (κ)
///
/// The ratio of the specific heat of dry air at constant pressure to its
/// specific heat at constant volume.
#[inline(always)]
pub fn Kappa() -> Ratio {
    Ratio::new::<ratio>(1.4)
}

/// Effective collision diameter of an air molecule (σ)
#[inline(always)]
pub fn Sigma() -> Length {
    Length::new::<meter>(0.365_e-9)
}

/// The square of the effective collision diameter of an air molecule (σ²)
#[inline(always)]
pub fn SigmaSquared() -> Area {
    Sigma() * Sigma()
}

/// Standard geopotential metre (m′)
///
/// As defined by the World Meteorological Organization
#[inline(always)]
pub fn standard_geopotential_metre() -> AvailableEnergy {
    AvailableEnergy::new::<joule_per_kilogram>(9.806_65)
}

/// Radius of the earth (r)
#[inline(always)]
pub fn earth_radius() -> Length {
    Length::new::<meter>(6_356_766.)
}

/// Speed of sound in standard atmosphere at mean sea level (a₀)
#[inline(always)]
pub fn speed_of_sound_msl() -> Velocity {
    Velocity::new::<meter_per_second>(340.294)
}

/// Standard gravitational acceleration at mean sea level (g₀)
#[inline(always)]
pub fn standard_gravity_msl() -> Acceleration {
    Acceleration::new::<meter_per_second_squared>(9.806_65)
}

/// Standard pressure of dry air at mean sea level (P₀)
#[inline(always)]
pub fn standard_pressure_msl() -> Pressure {
    Pressure::new::<hectopascal>(1013.25)
}

/// Standard density of dry air at mean sea level (ρ₀)
pub fn standard_density_msl() -> MassDensity {
    MassDensity::new::<kilogram_per_cubic_meter>(1.225)
}

/// Standard temperature of dry air at mean sea level (T₀)
pub fn standard_temperature_msl() -> ThermodynamicTemperature {
    ThermodynamicTemperature::new::<celsius>(15.)
}

/// Relative humidity of dry air
pub fn dry_air_relative_humidity() -> Ratio {
    Ratio::new::<ratio>(0.)
}
