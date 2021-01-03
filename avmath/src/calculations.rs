//! Calculations related to the ICAO Standard Atmosphere
//!
//! As described in the ICAO document 7488: _Manual of the ICAO Standard
//! Atmosphere_. Calculations requiring information about atmospheric layers
//! are valid for geopotential altitudes from 5 km below to
//! 80 km above mean sea level (geometric altitudes: 4.996 km
//! below to 81.020 km above)

use crate::{
    constants,
    isa::{DensityAltitude, GeopotentialAltitude, Layer},
    si::{
        DynamicViscosity, FrequencyByArea, InvLapseRate, KinematicViscosity, LapseRate,
        NumberDensity, SpecificWeight,
    },
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
    base_temperature + lapse_rate * (altitude - layer_base).remove_context()
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
    let inner =
        dbg!(1.0_f64 + (height_to_zero_temp * layer_height.remove_context()).get::<ratio>());
    let pressure_exp = ((-constants::standard_gravity_msl_over_Rd()) / lapse_rate).get::<ratio>();
    base_pressure * inner.powf(pressure_exp)
}

fn standard_pressure_no_lapse(
    altitude: GeopotentialAltitude,
    layer_base: GeopotentialAltitude,
    layer_temperature: ThermodynamicTemperature,
    base_pressure: Pressure,
) -> Pressure {
    let layer_height = dbg!(altitude - layer_base);
    let effective_lapse_rate: InvLapseRate =
        dbg!(layer_height.remove_context() / layer_temperature);
    let pressure_exp =
        dbg!(-constants::standard_gravity_msl_over_Rd() * effective_lapse_rate).get::<ratio>();
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

#[allow(dead_code)]
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

#[allow(dead_code)]
fn density_altitude(
    ambient_pressure: Pressure,
    temperature: ThermodynamicTemperature,
    dew_point: ThermodynamicTemperature,
) -> DensityAltitude {
    let vapor_pressure = dbg!(saturation_vapor_pressure_fast(dew_point));
    let relative_humidity = dbg!(relative_humidity(ambient_pressure, vapor_pressure));
    let virtual_temperature = dbg!(virtual_temperature(relative_humidity, temperature));

    let air_density = dbg!(moist_air_density(
        ambient_pressure,
        vapor_pressure,
        virtual_temperature
    ));

    //DensityAltitude::interpret

    //let density_pressure = dbg!(temperature * air_density * (*R));

    //let density_pressure = ambient_pressure * virtual_temperature;

    let layer = dbg!(Layer::find_by_density(air_density).unwrap());

    let relative_pressure = dbg!(ambient_pressure / layer.pressure.start);
    let relative_temperature = dbg!(layer.base_temperature / virtual_temperature);

    let relative_pressure_temperature = dbg!(relative_pressure * relative_temperature);

    let altitude_above_layer_base: GeopotentialAltitude = if let Some(lapse_rate) = layer.lapse_rate
    {
        // let inner = 1.0_f64 + f64::from((lapse_rate * (altitude - layer_base)) / base_temperature);
        // let power = -f64::from(constants::standard_gravity_msl()/(*R * lapse_rate));
        // let standard_pressure = base_pressure * inner.powf(power);

        let temperature_height: Length = dbg!(layer.base_temperature / lapse_rate);

        let pressure_exp_m1 =
            dbg!(lapse_rate * constants::Rd_over_standard_gravity_msl()).get::<ratio>();
        let temp_ratio = dbg!(
            1.0_f64
                - relative_pressure_temperature
                    .get::<ratio>()
                    .powf(pressure_exp_m1)
        );

        let layer_height: Length = dbg!(temp_ratio * temperature_height);
        GeopotentialAltitude::interpret(layer_height)

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
        GeopotentialAltitude::interpret(layer_height)
    };

    DensityAltitude::interpret((altitude_above_layer_base + layer.altitude.start).remove_context())
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
    use crate::{
        constants,
        isa::{AltimeterSetting, GeometricAltitude, GeopotentialAltitude},
    };
    use uom::si::acceleration::meter_per_second_squared;
    use uom::si::f64::*;
    use uom::si::length::{foot, meter};
    use uom::si::mass_density::kilogram_per_cubic_meter;
    use uom::si::pressure::{hectopascal, inch_of_mercury};
    use uom::si::thermodynamic_temperature::{degree_celsius, kelvin};

    /// Compares two values by equalizing their magnitudes and determining whether
    /// the values are equal over the requested number of significant figures
    ///
    /// The final digit is allowed to be off by one unit above or below to account for
    /// potential rounding errors.
    pub fn are_equal_in_significant_figures(expected: f64, actual: f64, figures: u8) -> bool {
        let magnitude_expected = dbg!(dbg!(expected).abs().log10());
        let bonus_expected = if magnitude_expected < 0. { 0. } else { 1. };
        let normalized_expected = (expected
            * 10_f64.powf(figures as f64 - magnitude_expected.trunc() - bonus_expected))
        .round();

        let magnitude_actual = dbg!(dbg!(actual).abs().log10());
        let bonus_actual = if magnitude_actual < 0. { 0. } else { 1. };
        let normalized_actual = (actual
            * 10_f64.powf(figures as f64 - magnitude_actual.trunc() - bonus_actual))
        .round();

        (dbg!(normalized_expected) - dbg!(normalized_actual)).abs() <= 1. // Allows for a rounding error in the
    }

    /// Asserts that two values are equal to a certain number of significant figures
    #[track_caller]
    pub fn assert_equal_in_significant_figures(expected: f64, actual: f64, figures: u8) {
        if !are_equal_in_significant_figures(expected, actual, figures) {
            println!("Expected: {}", expected);
            println!("Actual: {}", actual);
            panic!(
                "Expected and actual values differ within first {} significant figures",
                figures
            );
        }
    }

    pub fn are_equal_within_epsilon(expected: f64, actual: f64, epsilon: f64) -> bool {
        (expected - actual).abs() < epsilon
    }

    /// Asserts that two values are equal with a certain error range
    #[track_caller]
    pub fn assert_equal_within_epsilon(expected: f64, actual: f64, epsilon: f64) {
        if !are_equal_within_epsilon(expected, actual, epsilon) {
            println!("Expected: {}", expected);
            println!("Actual: {}", actual);
            panic!("Expected and actual values differ by more than {}", epsilon);
        }
    }

    #[test]
    #[should_panic]
    pub fn significant_figures_test_panic() {
        assert_equal_in_significant_figures(9.56859_e34, 9.56857_e34, 6)
    }

    #[test]
    pub fn significant_figures_test_no_panic_beyond_significant() {
        assert_equal_in_significant_figures(9.56859_e34, 9.568587_e34, 6)
    }

    #[test]
    #[should_panic]
    pub fn significant_figures_test_panic_low_magnitude() {
        assert_equal_in_significant_figures(9.56859_e-1, 9.56857_e-1, 6)
    }

    #[test]
    pub fn significant_figures_test_no_panic_beyond_significant_low_magnitude() {
        assert_equal_in_significant_figures(9.56859_e-1, 9.568589_e-1, 6)
    }

    const STD_TABLE: &[(f64, f64, f64, f64, f64, f64)] = &[
        (-5_000., -4_996., 320.650, 1.77687_e+3, 1.93047_e_0, 9.8221),
        (-4_500., -4_497., 317.400, 1.68423_e+3, 1.84856_e_0, 9.8205),
        (-4_000., -3_997., 314.150, 1.59555_e+3, 1.76934_e_0, 9.8190),
        (-3_500., -3_498., 310.900, 1.51068_e+3, 1.69274_e_0, 9.8175),
        (-3_000., -2_999., 307.650, 1.42950_e+3, 1.61870_e_0, 9.8159),
        (-2_500., -2_499., 304.400, 1.35190_e+3, 1.54717_e_0, 9.8144),
        (-2_000., -1_999., 301.150, 1.27774_e+3, 1.47808_e_0, 9.8128),
        (-1_500., -1_500., 297.900, 1.20691_e+3, 1.41137_e_0, 9.8113),
        (-1_000., -1_000., 294.650, 1.13929_e+3, 1.34700_e_0, 9.8097),
        (-500., -500., 291.400, 1.07478_e+3, 1.28489_e_0, 9.8082),
        (0., 0., 288.150, 1.01325_e+3, 1.22500_e_0, 9.8067),
        (500., 500., 284.900, 9.54608_e+2, 1.16727_e_0, 9.8051),
        (1_000., 1_000., 281.650, 8.98746_e+2, 1.11164_e_0, 9.8036),
        (1_500., 1_500., 278.400, 8.45560_e+2, 1.05807_e_0, 9.8020),
        (2_000., 2_001., 275.150, 7.94952_e+2, 1.00649_e_0, 9.8005),
        (2_500., 2_501., 271.900, 7.46825_e+2, 9.56859_e-1, 9.7989),
        (3_000., 3_001., 268.650, 7.01085_e+2, 9.09122_e-1, 9.7974),
        (3_500., 3_502., 265.400, 6.57641_e+2, 8.63229_e-1, 9.7959),
        (4_000., 4_003., 262.150, 6.16402_e+2, 8.19129_e-1, 9.7943),
        (7_000., 7_008., 242.650, 4.10607_e+2, 5.89501_e-1, 9.7851),
        (10_000., 10_016., 223.150, 2.64362_e+2, 4.12706_e-1, 9.7758),
        (13_000., 13_027., 216.650, 1.65104_e+2, 2.65482_e-1, 9.7666),
        (16_000., 16_040., 216.650, 1.02874_e+2, 1.65419_e-1, 9.7573),
        (19_000., 19_057., 216.650, 6.40998_e+1, 1.03071_e-1, 9.7481),
        (22_000., 22_076., 218.650, 3.99977_e+1, 6.37271_e-2, 9.7389),
        (23_000., 23_084., 219.650, 3.42242_e+1, 5.42800_e-2, 9.7358),
        (25_000., 25_099., 221.650, 2.51101_e+1, 3.94656_e-2, 9.7297),
        (28_000., 28_124., 224.650, 1.58629_e+1, 2.45987_e-2, 9.7204),
        (31_000., 31_152., 227.650, 1.00823_e+1, 1.54287_e-2, 9.7112),
        (36_000., 36_205., 239.850, 4.84314_e+1, 7.03437_e-3, 9.6959),
        (42_000., 42_279., 256.650, 2.12029_e_0, 2.87800_e-3, 9.6775),
        (47_000., 47_350., 270.650, 1.10906_e_0, 1.42752_e-3, 9.6622),
        (47_500., 47_858., 270.650, 1.04122_e_0, 1.34021_e-3, 9.6606),
        (47_900., 48_264., 270.650, 9.89959_e-1, 1.27422_e-3, 9.6594), // 9.89956
        (50_900., 51_311., 270.650, 6.77890_e-1, 8.72548_e-3, 9.6502), // 6.77887, 8.72544
        (57_000., 57_516., 253.850, 3.06272_e-1, 4.20308_e-3, 9.6316),
        (69_000., 69_757., 220.250, 5.41713_e-2, 8.56823_e-5, 9.5949),
        //( 80_000.,  81_020., 196.650, 8.86272_e-3, 1.57004_e-5, 9.5614),
    ];

    #[allow(non_snake_case)]
    #[derive(Clone, Copy, Debug)]
    struct StandardTable {
        H: GeopotentialAltitude,
        h: GeometricAltitude,
        T: ThermodynamicTemperature,
        p: Pressure,
        rho: MassDensity,
        g: Acceleration,
    }

    #[allow(non_snake_case)]
    fn standard_table() -> impl IntoIterator<Item = StandardTable> {
        STD_TABLE
            .iter()
            .copied()
            .map(|(H, h, T, p, rho, g)| StandardTable {
                H: GeopotentialAltitude::new::<meter>(H),
                h: GeometricAltitude::new::<meter>(h),
                T: ThermodynamicTemperature::new::<kelvin>(T),
                p: Pressure::new::<hectopascal>(p),
                rho: MassDensity::new::<kilogram_per_cubic_meter>(rho),
                g: Acceleration::new::<meter_per_second_squared>(g),
            })
    }

    #[test]
    fn check_geometric_to_geopotential() {
        for entry in standard_table() {
            assert_equal_within_epsilon(
                entry.H.get::<meter>(),
                GeopotentialAltitude::from(entry.h).get::<meter>(),
                1.,
            );
        }
    }

    #[test]
    fn check_geopotential_to_geometric() {
        for entry in standard_table() {
            assert_equal_within_epsilon(
                entry.h.get::<meter>(),
                GeometricAltitude::from(entry.H).get::<meter>(),
                1.,
            );
        }
    }

    #[test]
    fn check_standard_temperature() {
        for entry in standard_table() {
            println!("Entry: {:?}", entry.H);
            assert_equal_in_significant_figures(
                entry.T.get::<kelvin>(),
                super::standard_temperature(entry.H)
                    .unwrap()
                    .get::<kelvin>(),
                6,
            );
        }
    }

    #[test]
    fn check_standard_pressure() {
        for entry in standard_table() {
            println!("Entry: {:?}", entry.H);
            assert_equal_in_significant_figures(
                entry.p.get::<hectopascal>(),
                super::standard_pressure(entry.H)
                    .unwrap()
                    .get::<hectopascal>(),
                6,
            );
        }
    }

    #[test]
    fn check_standard_density() {
        for entry in standard_table() {
            println!("Entry: {:?}", entry.H);
            assert_equal_in_significant_figures(
                entry.rho.get::<kilogram_per_cubic_meter>(),
                super::standard_density_dry_air(entry.p, entry.T).get::<kilogram_per_cubic_meter>(),
                6,
            );
        }
    }

    #[test]
    #[ignore]
    fn check_standard_gravity() {
        for entry in standard_table() {
            println!("Entry: {:?}", entry.H);
            //assert_equal_in_significant_figures(entry.g.get::<meter_per_second_squared>(), super::, 5)
        }
    }

    #[test]
    #[ignore = "Still being worked on"]
    fn pressure_altitude() {
        assert_equal_in_significant_figures(
            29.92,
            GeopotentialAltitude::new::<foot>(0.)
                .to_pressure(AltimeterSetting::new::<inch_of_mercury>(29.92))
                .unwrap()
                .get::<inch_of_mercury>(),
            4,
        );
    }

    #[test]
    #[ignore = "Still being worked on"]
    fn pressure_altitude_high() {
        assert_equal_in_significant_figures(
            265.,
            GeopotentialAltitude::new::<meter>(9984.3)
                .to_pressure(AltimeterSetting::new::<hectopascal>(1013.25))
                .unwrap()
                .get::<hectopascal>(),
            4,
        );
    }

    #[test]
    #[ignore = "Still being worked on"]
    fn pressure_altitude_low_pressure() {
        assert_equal_in_significant_figures(
            261.88,
            GeopotentialAltitude::new::<meter>(9984.3)
                .to_pressure(AltimeterSetting::new::<hectopascal>(1004.))
                .unwrap()
                .get::<hectopascal>(),
            4,
        );
    }

    #[test]
    #[ignore = "Still being worked on"]
    fn pressure_altitude_higher_altitude() {
        assert_equal_in_significant_figures(
            2.74,
            GeopotentialAltitude::new::<meter>(29859.1)
                .to_pressure(AltimeterSetting::new::<hectopascal>(1004.))
                .unwrap()
                .get::<hectopascal>(),
            4,
        );
    }

    #[test]
    fn pressure_altitude_asos() {
        assert_equal_in_significant_figures(
            29.92,
            GeopotentialAltitude::new::<foot>(0.)
                .to_pressure_asos(AltimeterSetting::new::<inch_of_mercury>(29.92))
                .get::<inch_of_mercury>(),
            4,
        );
    }

    #[test]
    fn pressure_altitude_asos_high() {
        assert_equal_in_significant_figures(
            265.,
            GeopotentialAltitude::new::<meter>(9984.3)
                .to_pressure_asos(AltimeterSetting::new::<hectopascal>(1013.25))
                .get::<hectopascal>(),
            4,
        );
    }

    #[test]
    fn pressure_altitude_asos_low_pressure() {
        assert_equal_in_significant_figures(
            261.88,
            GeopotentialAltitude::new::<meter>(9984.3)
                .to_pressure_asos(AltimeterSetting::new::<hectopascal>(1004.))
                .get::<hectopascal>(),
            4,
        );
    }

    #[test]
    fn pressure_altitude_asos_higher_altitude() {
        assert_equal_in_significant_figures(
            2.74,
            GeopotentialAltitude::new::<meter>(29859.1)
                .to_pressure_asos(AltimeterSetting::new::<hectopascal>(1004.))
                .get::<hectopascal>(),
            4,
        );
    }

    #[test]
    #[ignore = "Still being worked on"]
    fn density_altitude_dry() {
        dbg!(super::standard_density_dry_air(
            super::standard_pressure(GeopotentialAltitude::new::<meter>(1234.)).unwrap(),
            super::standard_temperature(GeopotentialAltitude::new::<meter>(1234.)).unwrap(),
        ));
        assert_equal_within_epsilon(
            1234.,
            super::density_altitude(
                Pressure::new::<hectopascal>(898.5),
                constants::standard_temperature_msl(),
                ThermodynamicTemperature::new::<kelvin>(1.),
            )
            .get::<meter>(),
            1.,
        );
    }

    #[test]
    #[ignore = "Still being worked on"]
    fn density_altitude_odd() {
        dbg!(super::standard_density_dry_air(
            super::standard_pressure(GeopotentialAltitude::new::<foot>(12098.)).unwrap(),
            super::standard_temperature(GeopotentialAltitude::new::<foot>(12098.)).unwrap(),
        ));
        assert_equal_within_epsilon(
            12098.,
            super::density_altitude(
                Pressure::new::<hectopascal>(724.2),
                ThermodynamicTemperature::new::<degree_celsius>(30.),
                ThermodynamicTemperature::new::<degree_celsius>(23.),
            )
            .get::<foot>(),
            1.,
        );
    }
}
