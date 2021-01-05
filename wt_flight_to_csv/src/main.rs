//! Flight Data Recording to CSV processor
//!
//! Usage:
//!
//! ```sh
//! wt_flight_to_csv <input> <output>
//! ```
//!
//! For example:
//!
//! ```sh
//! wt_flight_to_csv 2021-01-05T11-43-44_01.msgpack.gz 2021-01-05T11-43-44_01.csv
//! ```

use wt_cj4::control_params::{ThrottleAxis, ThrottleMode, ThrottlePercent};
use wt_cj4::engines::EngineNumber;

#[derive(serde::Serialize)]
struct FlatSnapshot {
    simulation_time: f64,
    delta_t: f64,
    mach_number: f64,
    ambient_density: f64,
    geometric_altitude: f64,
    pressure_altitude: f64,
    engine1_thrust: f64,
    engine1_fadec_mode: ThrottleMode,
    engine1_physical_throttle: ThrottleAxis,
    engine1_engine_throttle: ThrottlePercent,
    engine1_visual_throttle: ThrottlePercent,
    engine1_pid_config: String,
    engine1_pid_last_error: f64,
    engine1_pid_retained_error: f64,
    engine1_fadec_enabled: bool,
    engine2_thrust: f64,
    engine2_fadec_mode: ThrottleMode,
    engine2_physical_throttle: ThrottleAxis,
    engine2_engine_throttle: ThrottlePercent,
    engine2_visual_throttle: ThrottlePercent,
    engine2_pid_config: String,
    engine2_pid_last_error: f64,
    engine2_pid_retained_error: f64,
    engine2_fadec_enabled: bool,
}

fn main() {
    let mut args = std::env::args();
    args.next();
    let ipath = args.next().unwrap();
    let opath = args.next().unwrap();

    let i = flate2::read::GzDecoder::new(std::fs::File::open(ipath).unwrap());
    let o = std::fs::File::create(opath).unwrap();

    let mut istream = rmp_serde::Deserializer::new(i);
    let mut o = csv::WriterBuilder::new().has_headers(true).from_writer(o);

    loop {
        let x: wt_cj4::Snapshot = serde::de::Deserialize::deserialize(&mut istream).unwrap();
        o.serialize(&FlatSnapshot {
            simulation_time: x.sim_time.get::<uom::si::time::second>(),
            delta_t: x.delta_t.get::<uom::si::time::second>(),
            mach_number: x
                .environment
                .instruments
                .mach_number
                .get::<uom::si::ratio::ratio>(),
            ambient_density: x
                .environment
                .instruments
                .ambient_density
                .get::<uom::si::mass_density::slug_per_cubic_foot>(),
            geometric_altitude: x
                .environment
                .instruments
                .geometric_altitude
                .get::<uom::si::length::foot>(),
            pressure_altitude: x
                .environment
                .instruments
                .pressure_altitude
                .get::<uom::si::length::foot>(),
            engine1_thrust: x.environment.engines[EngineNumber::Engine1]
                .thrust
                .get::<uom::si::force::poundal>(),
            engine1_fadec_mode: x.aircraft.engines[EngineNumber::Engine1].mode,
            engine1_physical_throttle: x.aircraft.engines[EngineNumber::Engine1].physical_throttle,
            engine1_engine_throttle: x.aircraft.engines[EngineNumber::Engine1].engine_throttle,
            engine1_visual_throttle: x.aircraft.engines[EngineNumber::Engine1].visual_throttle,
            engine1_pid_config: format!(
                "{:?}",
                x.aircraft.engines[EngineNumber::Engine1].fadec.pid_config()
            ),
            engine1_pid_last_error: x.aircraft.engines[EngineNumber::Engine1]
                .fadec
                .pid_state()
                .prior_error
                .get::<uom::si::force::poundal>(),
            engine1_pid_retained_error: (x.aircraft.engines[EngineNumber::Engine1]
                .fadec
                .pid_state()
                .retained_error
                / uom::si::f64::Time::new::<uom::si::time::second>(1.))
            .get::<uom::si::force::poundal>(),
            engine1_fadec_enabled: x.aircraft.engines[EngineNumber::Engine1].fadec.is_enabled(),
            engine2_thrust: x.environment.engines[EngineNumber::Engine2]
                .thrust
                .get::<uom::si::force::poundal>(),
            engine2_fadec_mode: x.aircraft.engines[EngineNumber::Engine2].mode,
            engine2_physical_throttle: x.aircraft.engines[EngineNumber::Engine2].physical_throttle,
            engine2_engine_throttle: x.aircraft.engines[EngineNumber::Engine2].engine_throttle,
            engine2_visual_throttle: x.aircraft.engines[EngineNumber::Engine2].visual_throttle,
            engine2_pid_config: format!(
                "{:?}",
                x.aircraft.engines[EngineNumber::Engine2].fadec.pid_config()
            ),
            engine2_pid_last_error: x.aircraft.engines[EngineNumber::Engine2]
                .fadec
                .pid_state()
                .prior_error
                .get::<uom::si::force::poundal>(),
            engine2_pid_retained_error: (x.aircraft.engines[EngineNumber::Engine2]
                .fadec
                .pid_state()
                .retained_error
                / uom::si::f64::Time::new::<uom::si::time::second>(1.))
            .get::<uom::si::force::poundal>(),
            engine2_fadec_enabled: x.aircraft.engines[EngineNumber::Engine2].fadec.is_enabled(),
        })
        .unwrap();
    }
}
