use flate2::write::GzEncoder;
use std::{fmt, fs, marker::PhantomData};

const MAX_EVENTS_PER_FILE: u32 = 20 * 60 * 30;

/// A flight data recorder for aircraft data
///
/// Data is output to a file in the `\work` directory with the initialization time as a prefix
/// and `.msgpack.gz` as the extension. Files produced are serialized as [MsgPack][] and then
/// processed by a gzip encoder. Over the course of a session, a log may be broken across
/// multiple files in order to keep the size of each individual log file reasonable. Files
/// belonging to the same session will have the same prefix, but have an incrementing number
/// associated with them. The full format for the filename is `%Y-%m-%dT%H-%M-%SZ_##.msgpack.gz`.
///
/// These files can then be later processed by the `wt_flight_to_csv` processor to convert the
/// data into a CSV format, suitable for further analysis.
///
///   [MsgPack]: https://msgpack.org
///
/// ```no_run
/// use wt_flight_recorder::FlightDataRecorder;
///
/// #[derive(serde::Serialize, serde::Deserialize)]
/// struct MyData {
///     time: f64,
///     thrust: f64,
///     output: f64,
/// }
///
/// let mut recorder: FlightDataRecorder<MyData> = FlightDataRecorder::new().unwrap();
///
/// let publish_result = recorder.publish(&MyData {
///     time: 0.0,
///     thrust: 1000.0,
///     output: 0.9,
/// });
///
/// if let Err(e) = publish_result {
///     eprintln!("Unable to log event: {}", e);
/// }
/// ```

pub struct FlightDataRecorder<T> {
    events: u32,
    file_num: u32,
    prefix: String,
    writer: GzEncoder<fs::File>,
    _phantom: PhantomData<T>,
}

impl<T> fmt::Debug for FlightDataRecorder<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("FlightDataRecorder")
            .field("events", &self.events)
            .field("file", &self.file_num)
            .field("prefix", &self.prefix)
            .field("writer", &"<boxed>")
            .finish()
    }
}

impl<T> FlightDataRecorder<T> {
    /// Constructs a new flight data recorder instance
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let prefix = format!("{}", chrono::Utc::now().format("%Y-%m-%dT%H-%M-%SZ"));
        println!("Logging using the {} prefix", prefix);
        let mut file_num = 0;
        let writer = open_file(&prefix, &mut file_num)?;
        Ok(FlightDataRecorder {
            events: 0,
            file_num,
            prefix,
            writer,
            _phantom: PhantomData,
        })
    }

    fn manage_files(&mut self) {
        if self.events >= MAX_EVENTS_PER_FILE {
            println!("Recorded {} events; rotating...", self.events);
            match open_file(&self.prefix, &mut self.file_num) {
                Ok(w) => self.writer = w,
                Err(err) => println!(
                    "Error opening next file for logging; will try again later: {}",
                    err
                ),
            }
            self.events = 0;
        }
    }
}

impl<T> FlightDataRecorder<T>
where
    T: serde::Serialize,
{
    /// Publishes an event to the flight data recorder
    pub fn publish(&mut self, message: &T) -> Result<(), rmp_serde::encode::Error> {
        self.events += 1;
        rmp_serde::encode::write_named(&mut self.writer, message)?;

        self.manage_files();

        Ok(())
    }
}

fn open_file(
    prefix: &str,
    file_num: &mut u32,
) -> Result<GzEncoder<fs::File>, Box<dyn std::error::Error>> {
    let next = *file_num + 1;
    let filename = format!(r#"\work\{}_{:02}.msgpack.gz"#, prefix, next);
    println!("Opening {} for logging", &filename[..filename.len() - 2]);
    let file = std::fs::File::create(&filename)?;
    println!("Opened {} for logging", &filename[..filename.len() - 2]);
    *file_num = next;
    Ok(flate2::write::GzEncoder::new(
        file,
        flate2::Compression::best(),
    ))
}
