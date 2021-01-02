use crate::types::{InvLapseRate, LapseRate};
use std::ops::Div;
use uom::si::{
    acceleration::meter_per_second_squared, amount_of_substance::kilomole,
    available_energy::joule_per_kilogram, f64::*, length::meter,
    mass_density::kilogram_per_cubic_meter, molar_energy::joule_per_mole,
    molar_mass::kilogram_per_mole, pressure::hectopascal,
    specific_heat_capacity::joule_per_kilogram_kelvin, temperature_interval::kelvin as diff_kelvin,
    thermodynamic_temperature::kelvin, velocity::meter_per_second,
};

/// Gravitational acceleration at 0 MSL
#[inline(always)]
pub fn g0() -> Acceleration {
    Acceleration::new::<meter_per_second_squared>(9.806_65)
}

/// Avogadro's number
#[inline(always)]
pub fn NA() -> <f64 as Div<AmountOfSubstance>>::Output {
    602.257_e24 / AmountOfSubstance::new::<kilomole>(1.)
}

/// Molecular weight of dry air
#[inline(always)]
pub fn M0() -> MolarMass {
    MolarMass::new::<kilogram_per_mole>(28.964_420_e-3)
}

/// Molecular weight of water vapor
#[inline(always)]
pub fn Mv() -> MolarMass {
    MolarMass::new::<kilogram_per_mole>(18.01528_e-3)
}

/// Standard pressure of dry air at standard temperature at 0 MSL
#[inline(always)]
pub fn P0() -> Pressure {
    Pressure::new::<hectopascal>(1013.25)
}

/// Ideal gas constant
#[inline(always)]
pub fn RStar() -> <MolarEnergy as Div<ThermodynamicTemperature>>::Output {
    // Latest definition says this should be _exactly_ 8.31446261815324
    MolarEnergy::new::<joule_per_mole>(8.31432) / ThermodynamicTemperature::new::<kelvin>(1.)
}

/// Specific heat capacity of dry air (R / M)
#[inline(always)]
pub fn R() -> SpecificHeatCapacity {
    SpecificHeatCapacity::new::<joule_per_kilogram_kelvin>(287.052_87)
}

#[inline(always)]
pub fn g0_over_R() -> LapseRate {
    g0() / R()
}

#[inline(always)]
pub fn R_over_g0() -> InvLapseRate {
    R() / g0()
}

/// Specific heat capacity of water vapor (R / M)
#[inline(always)]
pub fn Rv() -> SpecificHeatCapacity {
    RStar() / Mv()
}

/// Temperature equivalent to 0deg C
#[inline(always)]
pub fn Ti() -> ThermodynamicTemperature {
    ThermodynamicTemperature::new::<kelvin>(273.15)
}

/// Standard temperature of dry air under standard pressure at 0 MSL
#[inline(always)]
pub fn T0() -> ThermodynamicTemperature {
    ThermodynamicTemperature::new::<kelvin>(288.15)
}

pub const Bs: f64 = 1.458_e-6; // kg * m^-1 * s^-1 * K^-1/2

#[inline(always)]
pub fn S() -> TemperatureInterval {
    TemperatureInterval::new::<diff_kelvin>(110.4)
}

pub const Kappa: f64 = 1.4;

#[inline(always)]
pub fn Rho0() -> MassDensity {
    MassDensity::new::<kilogram_per_cubic_meter>(1.225)
}

#[inline(always)]
pub fn Sigma() -> Length {
    Length::new::<meter>(0.365_e-9)
}

#[inline(always)]
pub fn SigmaSquared() -> Area {
    Sigma() * Sigma()
}

#[inline(always)]
pub fn MPrime() -> AvailableEnergy {
    AvailableEnergy::new::<joule_per_kilogram>(9.806_65)
}

#[inline(always)]
pub fn earth_radius() -> Length {
    Length::new::<meter>(6_356_766.)
}

/// Speed of sound in standard atmosphere
#[inline(always)]
pub fn a0() -> Velocity {
    Velocity::new::<meter_per_second>(340.294)
}
