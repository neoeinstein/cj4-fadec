mod flight_data_recorder;

pub use flight_data_recorder::FlightDataRecorder;

pub fn create_recorder() -> Result<FlightDataRecorder, Box<dyn std::error::Error>> {
    const TEST_PATHS: &[&str] = &[
        r#"\work"#,
        r#"/work"#,
        r#"work"#,
        r#".\"#,
        r#"./"#,
        r#"."#,
        r#".\work"#,
        r#"/"#,
        r#"\"#,
        r#""#,
    ];

    fn test_exists(path: &str) {
        let exists = std::path::Path::new(path).exists();
        println!("Does {} exist? {}", path, exists);
    }

    fn try_read_dir(path: &str) {
        let dir = std::fs::read_dir(r#"\work"#);
        let d = match dir {
            Ok(d) => d,
            Err(err) => {
                println!("Error reading {}: {}", path, err);
                return;
            }
        };
        for entry in d {
            println!("Found entry");
            match entry {
                Ok(e) => println!("Entry: {}", e.file_name().to_str().unwrap_or("<non UTF-8>")),
                Err(err) => println!("Error reading entry in {}: {}", path, err),
            }
        }
    }

    for path in TEST_PATHS {
        test_exists(path);
        try_read_dir(path);
    }

    let file = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .truncate(true)
        .create(true)
        .open(r#"\work\fdr.msgpack"#)?;
    let encoder = flate2::write::GzEncoder::new(file, flate2::Compression::best());
    Ok(FlightDataRecorder::new(encoder))
}
