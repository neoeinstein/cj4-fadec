use crate::{GeopotentialAltitude, LapseRate};
use uom::si::{
    f64::*,
    length::kilometer,
    pressure::hectopascal,
    mass_density::kilogram_per_cubic_meter,
    temperature_interval::kelvin as diff_kelvin,
    thermodynamic_temperature::kelvin,
};

#[derive(Clone, Copy, Debug)]
pub struct Layer {
    pub base_altitude: GeopotentialAltitude,
    pub top_altitude: GeopotentialAltitude,
    pub base_temperature: ThermodynamicTemperature,
    pub base_pressure: Pressure,
    pub top_pressure: Pressure,
    pub base_density: MassDensity,
    pub lapse_rate: Option<LapseRate>,
}

impl Layer {
    pub fn get_by_index(index: usize) -> Option<&'static Layer> {
        layers().get(index)
    }

    pub fn find_by_altitude(altitude: GeopotentialAltitude) -> Option<&'static Layer> {
        use once_cell::sync::OnceCell;
        static LAYER_INDEX: OnceCell<Vec<GeopotentialAltitude>> = OnceCell::new();
    
        let pulled_layers = layers();
        
        if altitude < pulled_layers[0].base_altitude {
            return None;
        }
        
        let idx = LAYER_INDEX.get_or_init(|| layers().iter().map(|l| l.top_altitude).collect())
            .iter()
            .copied()
            .position(|top| altitude < top)?;
    
        Some(&pulled_layers[idx])
    }
    
    pub fn find_by_pressure(pressure: Pressure) -> Option<&'static Layer> {
        use once_cell::sync::OnceCell;
        static LAYER_INDEX: OnceCell<Vec<Pressure>> = OnceCell::new();
    
        let pulled_layers = layers();
        
        if pressure > pulled_layers[0].base_pressure {
            return None;
        }
        
        let idx = LAYER_INDEX.get_or_init(|| layers().iter().map(|l| l.top_pressure).collect())
            .iter()
            .copied()
            .position(|top| pressure > top)?;
    
        Some(&pulled_layers[idx])
    }
}

fn construct_layers() -> [Layer; 8] {
    [
        Layer {
            base_altitude: GeopotentialAltitude::new::<kilometer>(-5.),
            top_altitude: GeopotentialAltitude::new::<kilometer>(0.),
            base_temperature: ThermodynamicTemperature::new::<kelvin>(320.65),
            base_pressure: Pressure::new::<hectopascal>(1.77687e3),
            top_pressure: Pressure::new::<hectopascal>(1.01325e3),
            base_density: MassDensity::new::<kilogram_per_cubic_meter>(1.93047),
            lapse_rate: Some(TemperatureInterval::new::<diff_kelvin>(-6.5) / Length::new::<kilometer>(1.)),
        },
        Layer {
            base_altitude: GeopotentialAltitude::new::<kilometer>(0.),
            top_altitude: GeopotentialAltitude::new::<kilometer>(11.),
            base_temperature: ThermodynamicTemperature::new::<kelvin>(288.150),
            base_pressure: Pressure::new::<hectopascal>(1.01325e3),
            top_pressure: Pressure::new::<hectopascal>(2.26320e2),
            base_density: MassDensity::new::<kilogram_per_cubic_meter>(1.22500),
            lapse_rate: Some(TemperatureInterval::new::<diff_kelvin>(-6.5) / Length::new::<kilometer>(1.)),
        },
        Layer {
            base_altitude: GeopotentialAltitude::new::<kilometer>(11.),
            top_altitude: GeopotentialAltitude::new::<kilometer>(20.),
            base_temperature: ThermodynamicTemperature::new::<kelvin>(216.650),
            base_pressure: Pressure::new::<hectopascal>(2.26320e2),
            top_pressure: Pressure::new::<hectopascal>(5.47487e1),
            base_density: MassDensity::new::<kilogram_per_cubic_meter>(3.63918e-1),
            lapse_rate: None,
        },
        Layer {
            base_altitude: GeopotentialAltitude::new::<kilometer>(20.),
            top_altitude: GeopotentialAltitude::new::<kilometer>(32.),
            base_temperature: ThermodynamicTemperature::new::<kelvin>(216.650),
            base_pressure: Pressure::new::<hectopascal>(5.47487e1),
            top_pressure: Pressure::new::<hectopascal>(8.68014e0),
            base_density: MassDensity::new::<kilogram_per_cubic_meter>(8.80345e-2),
            lapse_rate: Some(TemperatureInterval::new::<diff_kelvin>(1.) / Length::new::<kilometer>(1.))
        },
        Layer {
            base_altitude: GeopotentialAltitude::new::<kilometer>(32.),
            top_altitude: GeopotentialAltitude::new::<kilometer>(47.),
            base_temperature: ThermodynamicTemperature::new::<kelvin>(228.650),
            base_pressure: Pressure::new::<hectopascal>(8.68014e0),
            top_pressure: Pressure::new::<hectopascal>(1.10906e0),
            base_density: MassDensity::new::<kilogram_per_cubic_meter>(1.32249e-2),
            lapse_rate: Some(TemperatureInterval::new::<diff_kelvin>(2.8) / Length::new::<kilometer>(1.))
        },
        Layer {
            base_altitude: GeopotentialAltitude::new::<kilometer>(47.),
            top_altitude: GeopotentialAltitude::new::<kilometer>(51.),
            base_temperature: ThermodynamicTemperature::new::<kelvin>(270.650),
            base_pressure: Pressure::new::<hectopascal>(6.69384e-1),
            top_pressure: Pressure::new::<hectopascal>(1.10906e0),
            base_density: MassDensity::new::<kilogram_per_cubic_meter>(1.42752e0),
            lapse_rate: None,
        },
        Layer {
            base_altitude: GeopotentialAltitude::new::<kilometer>(51.),
            top_altitude: GeopotentialAltitude::new::<kilometer>(71.),
            base_temperature: ThermodynamicTemperature::new::<kelvin>(270.650),
            base_pressure: Pressure::new::<hectopascal>(6.69384e-1),
            top_pressure: Pressure::new::<hectopascal>(3.95639e-2),
            base_density: MassDensity::new::<kilogram_per_cubic_meter>(8.61600e-4),
            lapse_rate: Some(TemperatureInterval::new::<diff_kelvin>(-2.8) / Length::new::<kilometer>(1.))
        },
        Layer {
            base_altitude: GeopotentialAltitude::new::<kilometer>(71.),
            top_altitude: GeopotentialAltitude::new::<kilometer>(80.),
            base_temperature: ThermodynamicTemperature::new::<kelvin>(214.650),
            base_pressure: Pressure::new::<hectopascal>(3.95639e-2),
            top_pressure: Pressure::new::<hectopascal>(8.86272e-3),
            base_density: MassDensity::new::<kilogram_per_cubic_meter>(6.42105e-5),
            lapse_rate: Some(TemperatureInterval::new::<diff_kelvin>(-2.0) / Length::new::<kilometer>(1.))
        },
    ]
}

fn layers() -> &'static [Layer; 8] {
    use once_cell::sync::OnceCell;
    static LAYERS: OnceCell<[Layer; 8]> = OnceCell::new();

    LAYERS.get_or_init(construct_layers)
}

