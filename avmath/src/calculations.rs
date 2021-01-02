//! Calculations related to the ICAO Standard Atmosphere
//!
//! As described in the ICAO document 7488: _Manual of the ICAO Standard
//! Atmosphere_. Calculations requiring information about atmospheric layers
//! are valid for geopotential altitudes from 5 km below to
//! 80 km above mean sea level (geometric altitudes: 4.996 km
//! below to 81.020 km above)

use crate::{
    atmosphere::Layer, constants, DensityAltitude, DynamicViscosity, FrequencyByArea,
    GeopotentialAltitude, InvLapseRate, KinematicViscosity, LapseRate, NumberDensity,
    SpecificWeight,
};
use uom::si::{
    f64::*,
    pressure::{millibar, pascal},
    ratio::ratio,
    temperature_interval::kelvin as diff_kelvin,
    thermal_conductivity::watt_per_meter_kelvin,
    thermodynamic_temperature::{degree_celsius as celsius, kelvin},
    time::second,
};

/// The ideal pressure of a gass given density, specific heat capacity,
/// and ambient temperature
#[inline]
pub fn ideal_pressure(
    rho: MassDensity,
    r: SpecificHeatCapacity,
    t: ThermodynamicTemperature,
) -> Pressure {
    rho * r * t
}

/// Computes the standard temperature at a particular altitude from attributes
/// of the layer
pub fn standard_temperature_in_layer(
    altitude: GeopotentialAltitude,
    layer_base: GeopotentialAltitude,
    base_temperature: ThermodynamicTemperature,
    lapse_rate: LapseRate,
) -> ThermodynamicTemperature {
    base_temperature + lapse_rate * (altitude - layer_base)
}

/// Computes the standard temperature
pub fn standard_temperature(altitude: GeopotentialAltitude) -> Option<ThermodynamicTemperature> {
    let layer = Layer::find_by_altitude(altitude)?;
    Some(standard_temperature_in_layer(
        altitude,
        layer.altitude.start,
        layer.base_temperature,
        layer.lapse_rate.unwrap_or_default(),
    ))
}

fn standard_pressure_with_lapse(
    altitude: GeopotentialAltitude,
    layer_base: GeopotentialAltitude,
    base_temperature: ThermodynamicTemperature,
    lapse_rate: LapseRate,
    base_pressure: Pressure,
) -> Pressure {
    let layer_height = dbg!(altitude - layer_base);
    let height_to_zero_temp = dbg!(lapse_rate / base_temperature);
    let inner = dbg!(1.0_f64 + (height_to_zero_temp * layer_height).get::<ratio>());
    let pressure_exp = ((-constants::standard_gravity_msl_over_Rd()) / lapse_rate).get::<ratio>();
    base_pressure * inner.powf(pressure_exp)
}

fn standard_pressure_no_lapse(
    altitude: GeopotentialAltitude,
    layer_base: GeopotentialAltitude,
    layer_temperature: ThermodynamicTemperature,
    base_pressure: Pressure,
) -> Pressure {
    let layer_height = altitude - layer_base;
    let effective_lapse_rate: InvLapseRate = layer_height / layer_temperature;
    let pressure_exp =
        ((-constants::standard_gravity_msl_over_Rd()) * effective_lapse_rate).get::<ratio>();
    base_pressure * pressure_exp.exp()
}

/// Computes the standard pressure for a given altitude
pub fn standard_pressure(altitude: GeopotentialAltitude) -> Option<Pressure> {
    let layer = Layer::find_by_altitude(altitude)?;
    if let Some(lapse_rate) = layer.lapse_rate {
        Some(standard_pressure_with_lapse(
            altitude,
            layer.altitude.start,
            layer.base_temperature,
            lapse_rate,
            layer.pressure.start,
        ))
    } else {
        Some(standard_pressure_no_lapse(
            altitude,
            layer.altitude.start,
            layer.base_temperature,
            layer.pressure.start,
        ))
    }
}

/// Density of dry air at a given temperature and pressure
pub fn standard_density_dry_air(
    pressure: Pressure,
    temperature: ThermodynamicTemperature,
) -> MassDensity {
    pressure / (constants::Rd() * temperature)
}

/// Gravitational acceleration corrected for altitude
pub fn gravitational_acceleration(altitude: GeopotentialAltitude) -> Acceleration {
    let inner = constants::earth_radius() / (constants::earth_radius() + altitude.remove_context());
    let square = inner * inner;
    constants::standard_gravity_msl() * square
}

/// Specific weight given density and gravitational acceleration
pub fn specific_weight(
    density: MassDensity,
    gravitational_acceleration: Acceleration,
) -> SpecificWeight {
    density * gravitational_acceleration
}

/// Density of elements within a volume given pressure and temperature
pub fn number_density(pressure: Pressure, temperature: ThermodynamicTemperature) -> NumberDensity {
    (constants::avogadros_number() * pressure) / (constants::R() * temperature)
}

/// Mean particle speed in dry air
pub fn mean_particle_speed(temperature: ThermodynamicTemperature) -> Velocity {
    (8. / std::f64::consts::PI * constants::Rd() * temperature).sqrt()
}

/// Mean free path given temperature and pressure
pub fn mean_free_path_temp_pres(
    temperature: ThermodynamicTemperature,
    pressure: Pressure,
) -> Length {
    constants::R()
        / (std::f64::consts::SQRT_2
            * std::f64::consts::PI
            * constants::avogadros_number()
            * constants::SigmaSquared())
        * temperature
        / pressure
}

/// Mean distance between collisions of dry air given a number density
pub fn mean_free_path_number_density(number_density: NumberDensity) -> Length {
    (std::f64::consts::SQRT_2 * std::f64::consts::PI * constants::SigmaSquared() * number_density)
        .recip()
}

/// Frequency of collisions between particles of dry air within an area
pub fn collision_frequency(
    number_density: NumberDensity,
    temperature: ThermodynamicTemperature,
) -> FrequencyByArea {
    0.944_541_e-18 * number_density * (constants::Rd() * temperature).sqrt()
}

/// Dynamic viscosity between two neighboring layers of dry air moving at
/// different speeds given a temperature
pub fn dynamic_viscosity(temperature: ThermodynamicTemperature) -> DynamicViscosity {
    let bs = Pressure::new::<pascal>(constants::Bs()) * Time::new::<second>(1.);
    let t_3_2 = ThermodynamicTemperature::new::<kelvin>(temperature.get::<kelvin>().powf(1.5));
    (bs * t_3_2) / (constants::S() + temperature)
}

/// Kinematic viscosity given the dynamic viscosity and density
pub fn kinematic_viscosity(
    dynamic_viscosity: DynamicViscosity,
    density: MassDensity,
) -> KinematicViscosity {
    dynamic_viscosity / density
}

/// Thermal conductivity of between two layers of dry air with a
/// given temperature difference
pub fn thermal_conductivity(temperature_difference: TemperatureInterval) -> ThermalConductivity {
    let raw_t = temperature_difference.get::<diff_kelvin>();
    let x = 245.4 * 10.0_f64.powf(-12. / raw_t);
    ThermalConductivity::new::<watt_per_meter_kelvin>((2.648151_e-3 * raw_t) / (raw_t + x))
}

/// The speed of sound in dry air given an ambient temperature
pub fn speed_of_sound(temperature: ThermodynamicTemperature) -> Velocity {
    (constants::Kappa() * constants::Rd() * temperature).sqrt()
}

/// Calculates the saturation pressure of water vapor at a given
/// thermodynamic temperature
///
/// For a faster approximation that should be valid within common
/// atmospheric conditions, see [`saturation_vapor_pressure_fast`].
///
/// Based on the formula from https://wahiduddin.net/calc/density_altitude.htm
pub fn saturation_vapor_pressure_wobus(temperature: ThermodynamicTemperature) -> Pressure {
    const ESO: f64 = 6.1078;
    const C0: f64 = 0.99999683;
    const C1: f64 = -0.90826951e-2;
    const C2: f64 = 0.78736169e-4;
    const C3: f64 = -0.61117958e-6;
    const C4: f64 = 0.43884187e-8;
    const C5: f64 = -0.29883885e-10;
    const C6: f64 = 0.21874425e-12;
    const C7: f64 = -0.17892321e-14;
    const C8: f64 = 0.11112018e-16;
    const C9: f64 = -0.30994571e-19;

    let t = temperature.get::<celsius>();
    let p = t.mul_add(
        t.mul_add(
            t.mul_add(
                t.mul_add(
                    t.mul_add(
                        t.mul_add(t.mul_add(t.mul_add(t.mul_add(C9, C8), C7), C6), C5),
                        C4,
                    ),
                    C3,
                ),
                C2,
            ),
            C1,
        ),
        C0,
    );
    Pressure::new::<millibar>(ESO * p.powi(-8))
}

/// Calculates the saturation pressure of water vapor at a given
/// thermodynamic temperature using an approximation
///
/// This formula is generally within 1mb of the Wobus formula up
/// to about 70 °C. Beyond that limit, the error increases.
pub fn saturation_vapor_pressure_fast(temperature: ThermodynamicTemperature) -> Pressure {
    const C0: f64 = 6.1078;
    const C1: f64 = 7.5;
    const C2: f64 = 237.3;

    let t = temperature.get::<celsius>();
    if t < C2 {
        Pressure::new::<millibar>(0.)
    } else {
        let p = (C1 * t) / (C2 + t);
        Pressure::new::<millibar>(C0 * 10_f64.powf(p))
    }
}

/// Calculates the relative humidity given the ambient pressure
/// and partial pressure of water vapor
pub fn relative_humidity(ambient_pressure: Pressure, vapor_pressure: Pressure) -> Ratio {
    vapor_pressure / ambient_pressure
}

fn moist_air_density(
    ambient_pressure: Pressure,
    vapor_pressure: Pressure,
    temperature: ThermodynamicTemperature,
) -> MassDensity {
    let dry_air_pressure = ambient_pressure - vapor_pressure;
    (dry_air_pressure / (constants::Rd() * temperature))
        + (vapor_pressure / (constants::Rv() * temperature))

    //ambient_pressure / (*R * temperature) * (Ratio::new::<ratio>(1.) - ((0.378 * vapor_pressure) / ambient_pressure))
}

fn density_altitude(
    ambient_pressure: Pressure,
    temperature: ThermodynamicTemperature,
    dew_point: ThermodynamicTemperature,
) -> DensityAltitude {
    let vapor_pressure = saturation_vapor_pressure_fast(dew_point);
    let relative_humidity = relative_humidity(ambient_pressure, vapor_pressure);
    let virtual_temperature = dbg!(virtual_temperature(relative_humidity, temperature));

    // let air_density = dbg!(moist_air_density(ambient_pressure, vapor_pressure, temperature));

    //DensityAltitude::interpret

    //let density_pressure = dbg!(temperature * air_density * (*R));

    //let density_pressure = ambient_pressure * virtual_temperature;

    let layer = dbg!(Layer::find_by_pressure(ambient_pressure).unwrap());

    let relative_pressure = dbg!(ambient_pressure / layer.pressure.start);
    let relative_temperature = dbg!(layer.base_temperature / virtual_temperature);

    let relative_pressure_temperature = relative_pressure * relative_temperature;

    if let Some(lapse_rate) = layer.lapse_rate {
        // let inner = 1.0_f64 + f64::from((lapse_rate * (altitude - layer_base)) / base_temperature);
        // let power = -f64::from(constants::standard_gravity_msl()/(*R * lapse_rate));
        // let standard_pressure = base_pressure * inner.powf(power);

        let temperature_height: Length = dbg!(layer.base_temperature / lapse_rate);

        let pressure_exp_m1 =
            dbg!(lapse_rate * -constants::Rd_over_standard_gravity_msl()).get::<ratio>();
        let temp_ratio = dbg!(
            1.0_f64
                - relative_pressure_temperature
                    .get::<ratio>()
                    .powf(pressure_exp_m1)
        );

        let layer_height: Length = dbg!(temp_ratio * temperature_height);
        DensityAltitude::interpret(layer_height + layer.altitude.start.remove_context())

    // let x1 = layer.base_temperature / lapse_rate
    // let ex = lapse_rate * (*RStar) / ((constants::standard_gravity_msl()))
    } else {
        // let layer_height = altitude - layer_base;
        // let inner = f64::from(- (constants::standard_gravity_msl() * layer_height) / (*R * layer_temperature));
        // base_pressure * inner.exp()

        let pressure_exp_m1 = relative_pressure_temperature.get::<ratio>();
        let temp_ratio = pressure_exp_m1.ln();
        let height_gradient: Length =
            layer.base_temperature * -constants::Rd_over_standard_gravity_msl();
        let layer_height: Length = height_gradient * temp_ratio;
        DensityAltitude::interpret(layer_height + layer.altitude.start.remove_context())
    }
}

/// Computes the virtual temperature given the relative humidity
pub fn virtual_temperature(
    relative_humidity: Ratio,
    temperature: ThermodynamicTemperature,
) -> ThermodynamicTemperature {
    temperature
        / (1.
            - (1. - (constants::Mv() / constants::Md()).get::<ratio>())
                * relative_humidity.get::<ratio>())

    //relative_humidity.get::<ratio>().mul_add(0.61, 1.) * temperature
}

#[cfg(test)]
mod tests {
    use crate::{AltimeterSetting, DensityAltitude, GeopotentialAltitude};
    use uom::si::f64::*;
    use uom::si::length::{foot, meter};
    use uom::si::pressure::{hectopascal, inch_of_mercury};
    use uom::si::ratio::percent;
    use uom::si::temperature_interval::kelvin as diff_kelvin;
    use uom::si::thermodynamic_temperature::{degree_celsius, kelvin};

    fn temperature_difference(
        left: ThermodynamicTemperature,
        right: ThermodynamicTemperature,
    ) -> TemperatureInterval {
        TemperatureInterval::new::<diff_kelvin>(left.get::<kelvin>() - right.get::<kelvin>())
    }

    const MAX_ERROR_PERCENT: f64 = 0.0005;

    macro_rules! check_temps_within_error {
        (expected: $expected:expr, actual: $actual:expr,) => {
            let expected = $expected;
            let actual = $actual;
            let format = Ratio::format_args(percent, uom::fmt::DisplayStyle::Abbreviation);
            let expected_interval =
                TemperatureInterval::new::<diff_kelvin>(expected.get::<kelvin>());
            let error = (temperature_difference(expected, actual)) / expected_interval;

            let allowable = Ratio::new::<percent>(MAX_ERROR_PERCENT);
            println!("Expected: {:?}", expected);
            println!("Actual:   {:?}", actual);
            println!(
                "Error: {} (Allowed: {})",
                format.with(error),
                format.with(allowable)
            );

            assert!(error < allowable);
        };
    }

    macro_rules! check_within_error {
        (expected: $expected:expr, actual: $actual:expr,) => {
            let expected = $expected;
            let actual = $actual;
            let format = Ratio::format_args(percent, uom::fmt::DisplayStyle::Abbreviation);
            let error = if expected == actual {
                Ratio::new::<percent>(0.)
            // } else if expected.is_zero() {
            //     (actual - expected) / actual
            } else {
                (expected - actual) / expected
            };

            let allowable = Ratio::new::<percent>(MAX_ERROR_PERCENT);

            println!("Expected: {:?}", expected);
            println!("Actual:   {:?}", actual);
            println!(
                "Error: {} (Allowed: {})",
                format.with(error),
                format.with(allowable)
            );

            assert!(error < allowable);
        };
    }

    #[test]
    fn table1() {
        check_temps_within_error!(
            expected: ThermodynamicTemperature::new::<kelvin>(320.650),
            actual: super::standard_temperature(GeopotentialAltitude::new::<meter>(-5_000.)).unwrap(),
        );
    }

    #[test]
    fn table2() {
        check_temps_within_error!(
            expected: ThermodynamicTemperature::new::<kelvin>(315.775),
            actual: super::standard_temperature(GeopotentialAltitude::new::<meter>(-4_250.)).unwrap(),
        );
    }

    #[test]
    fn table3() {
        check_temps_within_error!(
            expected: ThermodynamicTemperature::new::<kelvin>(292.701),
            actual: super::standard_temperature(GeopotentialAltitude::new::<meter>(-700.)).unwrap(),
        );
    }

    #[test]
    fn pressure1() {
        check_within_error!(
            expected: Pressure::new::<hectopascal>(1.77687_e3),
            actual: super::standard_pressure(GeopotentialAltitude::new::<meter>(-5_000.)).unwrap(),
        );
    }

    #[test]
    fn pressure3() {
        check_within_error!(
            expected: Pressure::new::<hectopascal>(1.10022_e3),
            actual: super::standard_pressure(GeopotentialAltitude::new::<meter>(-700.)).unwrap(),
        );
    }

    #[test]
    fn pressure_altitude() {
        check_within_error!(
            expected: Pressure::new::<inch_of_mercury>(29.92),
            actual: GeopotentialAltitude::new::<foot>(0.).to_pressure(AltimeterSetting::new::<inch_of_mercury>(29.92)).unwrap(),
        );
    }

    #[test]
    fn pressure_altitude_high() {
        check_within_error!(
            expected: Pressure::new::<hectopascal>(265.),
            actual: GeopotentialAltitude::new::<meter>(9984.3).to_pressure(AltimeterSetting::new::<hectopascal>(1013.25)).unwrap(),
        );
    }

    #[test]
    fn pressure_altitude_low_pressure() {
        check_within_error!(
            expected: Pressure::new::<hectopascal>(261.88),
            actual: GeopotentialAltitude::new::<meter>(9984.3).to_pressure(AltimeterSetting::new::<hectopascal>(1004.)).unwrap(),
        );
    }

    #[test]
    fn pressure_altitude_higher_altitude() {
        check_within_error!(
            expected: Pressure::new::<hectopascal>(2.74),
            actual: GeopotentialAltitude::new::<meter>(29859.1).to_pressure(AltimeterSetting::new::<hectopascal>(1004.)).unwrap(),
        );
    }

    #[test]
    fn density_altitude_dry() {
        dbg!(super::standard_pressure(
            GeopotentialAltitude::new::<meter>(1234.)
        ));
        check_within_error!(
            expected: DensityAltitude::new::<meter>(1234.),
            actual: super::density_altitude(Pressure::new::<hectopascal>(898.5), ThermodynamicTemperature::new::<degree_celsius>(15.), ThermodynamicTemperature::new::<kelvin>(1.)),
        );
    }

    #[test]
    fn density_altitude_odd() {
        dbg!(super::standard_pressure(GeopotentialAltitude::new::<foot>(
            13098.
        )));
        check_within_error!(
            expected: DensityAltitude::new::<foot>(12098.),
            actual: super::density_altitude(Pressure::new::<hectopascal>(724.2), ThermodynamicTemperature::new::<degree_celsius>(30.), ThermodynamicTemperature::new::<degree_celsius>(23.)),
        );
    }
}
