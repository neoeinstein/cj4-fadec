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
    airspeed_indicated: f64,
    airspeed_true: f64,
    vertical_speed: f64,
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
    engine1_pid_proportional: f64,
    engine1_pid_integral: f64,
    engine1_pid_derivative: f64,
    engine1_pid_output: f64,
    engine1_fadec_enabled: bool,
    engine2_thrust: f64,
    engine2_fadec_mode: ThrottleMode,
    engine2_physical_throttle: ThrottleAxis,
    engine2_engine_throttle: ThrottlePercent,
    engine2_visual_throttle: ThrottlePercent,
    engine2_pid_config: String,
    engine2_pid_last_error: f64,
    engine2_pid_retained_error: f64,
    engine2_pid_proportional: f64,
    engine2_pid_integral: f64,
    engine2_pid_derivative: f64,
    engine2_pid_output: f64,
    engine2_fadec_enabled: bool,
}

fn find_splits(path: &str) -> Option<(&str, u32)> {
    let file_name = path.strip_suffix(".msgpack.gz")?;
    let mut splits = file_name.rsplit('_');
    let sequence = splits.next()?.parse::<u32>().ok()?;
    let stem = splits.next()?;
    Some((stem, sequence))
}

type Input = rmp_serde::Deserializer<
    rmp_serde::decode::ReadReader<flate2::read::GzDecoder<std::fs::File>>,
    rmp_serde::config::DefaultConfig,
>;

fn open_next(multi: &mut (&str, u32)) -> Option<Input> {
    multi.1 += 1;
    let path = format!("{}_{:02}.msgpack.gz", multi.0, multi.1);
    open(&path).ok()
}

fn open(path: &str) -> std::io::Result<Input> {
    let file = std::fs::File::open(path)?;
    println!("Processing {}", path);
    let reader = flate2::read::GzDecoder::new(file);
    Ok(rmp_serde::Deserializer::new(reader))
}

fn main() {
    let mut args = std::env::args();
    args.next();
    let ipath = args.next().unwrap();
    let opath_maybe = args.next();

    let mut multi = find_splits(&ipath);

    let opath = opath_maybe
        .or_else(|| multi.map(|m| format!("{}.csv", m.0)))
        .unwrap();

    println!("Output: {}", opath);

    let mut input = open(&ipath).unwrap();
    let o = std::fs::File::create(opath).unwrap();

    let mut o = csv::WriterBuilder::new().has_headers(true).from_writer(o);

    let mut records = 0;
    let mut files = 1;

    while process_record(&mut multi, &mut input, &mut o, &mut files, true) == Loop::Continue {
        records += 1;
    }

    println!("Processed {} records across {} files", records, files);
}

#[derive(PartialEq, Eq)]
enum Loop {
    Break,
    Continue,
}

fn process_record(
    multi: &mut Option<(&str, u32)>,
    input: &mut Input,
    output: &mut csv::Writer<std::fs::File>,
    files: &mut i32,
    recurse: bool,
) -> Loop {
    let x: wt_cj4::Snapshot = match serde::de::Deserialize::deserialize(&mut *input) {
        Ok(x) => x,
        Err(rmp_serde::decode::Error::InvalidMarkerRead(err))
            if err.kind() == std::io::ErrorKind::UnexpectedEof =>
        {
            if let Some(m) = multi {
                *input = if let Some(next) = open_next(m) {
                    *files += 1;
                    next
                } else {
                    return Loop::Break;
                };
                if recurse {
                    return process_record(multi, &mut *input, output, files, false);
                } else {
                    return Loop::Break;
                }
            } else {
                return Loop::Break;
            }
        }
        Err(err) => {
            eprintln!("Error deserializing: {}", err);
            return Loop::Break;
        }
    };
    output
        .serialize(&FlatSnapshot {
            simulation_time: x.sim_time.get::<uom::si::time::second>(),
            delta_t: x.delta_t.get::<uom::si::time::second>(),
            airspeed_indicated: x
                .environment
                .instruments
                .airspeed_indicated
                .get::<uom::si::velocity::knot>(),
            airspeed_true: x
                .environment
                .instruments
                .airspeed_true
                .get::<uom::si::velocity::knot>(),
            vertical_speed: x
                .environment
                .instruments
                .vertical_speed
                .get::<uom::si::velocity::foot_per_minute>(),
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
            engine1_pid_proportional: x.aircraft.engines[EngineNumber::Engine1]
                .fadec
                .last_pid_outputs()
                .proportional
                .get::<uom::si::ratio::ratio>(),
            engine1_pid_integral: x.aircraft.engines[EngineNumber::Engine1]
                .fadec
                .last_pid_outputs()
                .integral
                .get::<uom::si::ratio::ratio>(),
            engine1_pid_derivative: x.aircraft.engines[EngineNumber::Engine1]
                .fadec
                .last_pid_outputs()
                .derivative
                .get::<uom::si::ratio::ratio>(),
            engine1_pid_output: x.aircraft.engines[EngineNumber::Engine1]
                .fadec
                .last_pid_outputs()
                .output()
                .get::<uom::si::ratio::ratio>(),
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
            engine2_pid_proportional: x.aircraft.engines[EngineNumber::Engine2]
                .fadec
                .last_pid_outputs()
                .proportional
                .get::<uom::si::ratio::ratio>(),
            engine2_pid_integral: x.aircraft.engines[EngineNumber::Engine2]
                .fadec
                .last_pid_outputs()
                .integral
                .get::<uom::si::ratio::ratio>(),
            engine2_pid_derivative: x.aircraft.engines[EngineNumber::Engine2]
                .fadec
                .last_pid_outputs()
                .derivative
                .get::<uom::si::ratio::ratio>(),
            engine2_pid_output: x.aircraft.engines[EngineNumber::Engine2]
                .fadec
                .last_pid_outputs()
                .output()
                .get::<uom::si::ratio::ratio>(),
            engine2_fadec_enabled: x.aircraft.engines[EngineNumber::Engine2].fadec.is_enabled(),
        })
        .unwrap();
    Loop::Continue
}
