//! Layers of the ICAO International Standard Atmosphere

use crate::{GeopotentialAltitude, LapseRate};
use uom::si::{
    f64::*, length::kilometer, mass_density::kilogram_per_cubic_meter, pressure::hectopascal,
    temperature_interval::kelvin as diff_kelvin, thermodynamic_temperature::kelvin,
};
use once_cell::sync::Lazy;
use std::ops::Range;

/// A layer of the atmosphere
#[derive(Clone, Debug)]
pub struct Layer {
    /// The altitude range
    pub altitude: Range<GeopotentialAltitude>,
    /// The pressure range
    pub pressure: Range<Pressure>,
    /// The density range
    pub density: Range<MassDensity>,
    /// The standard temperature at the base of the layer
    pub base_temperature: ThermodynamicTemperature,
    /// The lapse rate for standard temperatures within this layer, or `None`
    /// the temperature is constant
    pub lapse_rate: Option<LapseRate>,
}

impl Layer {
    // pub fn get_by_index(index: usize) -> Option<&'static Layer> {
    //     layers().get(index)
    // }

    /// Returns the atmospheric layer associated with a given altitude
    pub fn find_by_altitude(altitude: GeopotentialAltitude) -> Option<&'static Layer> {
        use once_cell::sync::OnceCell;
        static LAYER_INDEX: OnceCell<Vec<GeopotentialAltitude>> = OnceCell::new();

        if altitude < LAYERS[0].altitude.start {
            return None;
        }

        let idx = LAYER_INDEX
            .get_or_init(|| LAYERS.iter().map(|l| l.altitude.end).collect())
            .iter()
            .copied()
            .position(|top| altitude < top)?;

        Some(&LAYERS[idx])
    }

    /// Returns the atmospheric layer associated with a given pressure
    pub fn find_by_pressure(pressure: Pressure) -> Option<&'static Layer> {
        use once_cell::sync::OnceCell;
        static LAYER_INDEX: OnceCell<Vec<Pressure>> = OnceCell::new();

        if pressure > LAYERS[0].pressure.start {
            return None;
        }

        let idx = LAYER_INDEX
            .get_or_init(|| LAYERS.iter().map(|l| l.pressure.end).collect())
            .iter()
            .copied()
            .position(|top| pressure > top)?;

        Some(&LAYERS[idx])
    }

    /// Returns the atmospheric layer associated with a given air density
    pub fn find_by_density(density: MassDensity) -> Option<&'static Layer> {
        use once_cell::sync::OnceCell;
        static LAYER_INDEX: OnceCell<Vec<MassDensity>> = OnceCell::new();

        if density > LAYERS[0].density.start {
            return None;
        }

        let idx = LAYER_INDEX
            .get_or_init(|| LAYERS.iter().map(|l| l.density.end).collect())
            .iter()
            .copied()
            .position(|top| density > top)?;

        Some(&LAYERS[idx])
    }
}

fn construct_layers() -> [Layer; 8] {
    [
        Layer {
            altitude: GeopotentialAltitude::new::<kilometer>(-5.)..GeopotentialAltitude::new::<kilometer>(0.),
            pressure: Pressure::new::<hectopascal>(1.77687e3)..Pressure::new::<hectopascal>(1.01325e3),
            density: MassDensity::new::<kilogram_per_cubic_meter>(1.93047)..MassDensity::new::<kilogram_per_cubic_meter>(1.22500),
            base_temperature: ThermodynamicTemperature::new::<kelvin>(320.65),
            lapse_rate: Some(
                TemperatureInterval::new::<diff_kelvin>(-6.5) / Length::new::<kilometer>(1.),
            ),
        },
        Layer {
            altitude: GeopotentialAltitude::new::<kilometer>(0.)..GeopotentialAltitude::new::<kilometer>(11.),
            pressure: Pressure::new::<hectopascal>(1.01325e3)..Pressure::new::<hectopascal>(2.26320e2),
            density: MassDensity::new::<kilogram_per_cubic_meter>(1.22500)..MassDensity::new::<kilogram_per_cubic_meter>(3.63918e-1),
            base_temperature: ThermodynamicTemperature::new::<kelvin>(288.150),
            lapse_rate: Some(
                TemperatureInterval::new::<diff_kelvin>(-6.5) / Length::new::<kilometer>(1.),
            ),
        },
        Layer {
            altitude: GeopotentialAltitude::new::<kilometer>(11.)..GeopotentialAltitude::new::<kilometer>(20.),
            pressure: Pressure::new::<hectopascal>(2.26320e2)..Pressure::new::<hectopascal>(5.47487e1),
            density: MassDensity::new::<kilogram_per_cubic_meter>(3.63918e-1)..MassDensity::new::<kilogram_per_cubic_meter>(8.80345e-2),
            base_temperature: ThermodynamicTemperature::new::<kelvin>(216.650),
            lapse_rate: None,
        },
        Layer {
            altitude: GeopotentialAltitude::new::<kilometer>(20.)..GeopotentialAltitude::new::<kilometer>(32.),
            pressure: Pressure::new::<hectopascal>(5.47487e1)..Pressure::new::<hectopascal>(8.68014e0),
            density: MassDensity::new::<kilogram_per_cubic_meter>(8.80345e-2)..MassDensity::new::<kilogram_per_cubic_meter>(1.32249e-2),
            base_temperature: ThermodynamicTemperature::new::<kelvin>(216.650),
            lapse_rate: Some(
                TemperatureInterval::new::<diff_kelvin>(1.) / Length::new::<kilometer>(1.),
            ),
        },
        Layer {
            altitude: GeopotentialAltitude::new::<kilometer>(32.)..GeopotentialAltitude::new::<kilometer>(47.),
            pressure: Pressure::new::<hectopascal>(8.68014e0)..Pressure::new::<hectopascal>(1.10906e0),
            density: MassDensity::new::<kilogram_per_cubic_meter>(1.32249e-2)..MassDensity::new::<kilogram_per_cubic_meter>(1.42752e0),
            base_temperature: ThermodynamicTemperature::new::<kelvin>(228.650),
            lapse_rate: Some(
                TemperatureInterval::new::<diff_kelvin>(2.8) / Length::new::<kilometer>(1.),
            ),
        },
        Layer {
            altitude: GeopotentialAltitude::new::<kilometer>(47.)..GeopotentialAltitude::new::<kilometer>(51.),
            pressure: Pressure::new::<hectopascal>(6.69384e-1)..Pressure::new::<hectopascal>(1.10906e0),
            density: MassDensity::new::<kilogram_per_cubic_meter>(1.42752e0)..MassDensity::new::<kilogram_per_cubic_meter>(8.61600e-4),
            base_temperature: ThermodynamicTemperature::new::<kelvin>(270.650),
            lapse_rate: None,
        },
        Layer {
            altitude: GeopotentialAltitude::new::<kilometer>(51.)..GeopotentialAltitude::new::<kilometer>(71.),
            pressure: Pressure::new::<hectopascal>(6.69384e-1)..Pressure::new::<hectopascal>(3.95639e-2),
            density: MassDensity::new::<kilogram_per_cubic_meter>(8.61600e-4)..MassDensity::new::<kilogram_per_cubic_meter>(6.42105e-5),
            base_temperature: ThermodynamicTemperature::new::<kelvin>(270.650),
            lapse_rate: Some(
                TemperatureInterval::new::<diff_kelvin>(-2.8) / Length::new::<kilometer>(1.),
            ),
        },
        Layer {
            altitude: GeopotentialAltitude::new::<kilometer>(71.)..GeopotentialAltitude::new::<kilometer>(80.),
            pressure: Pressure::new::<hectopascal>(3.95639e-2)..Pressure::new::<hectopascal>(8.86272e-3),
            density: MassDensity::new::<kilogram_per_cubic_meter>(6.42105e-5)..MassDensity::new::<kilogram_per_cubic_meter>(1.57004e-5),
            base_temperature: ThermodynamicTemperature::new::<kelvin>(214.650),
            lapse_rate: Some(
                TemperatureInterval::new::<diff_kelvin>(-2.0) / Length::new::<kilometer>(1.),
            ),
        },
    ]
}

static LAYERS: Lazy<[Layer; 8]> = Lazy::new(construct_layers);
