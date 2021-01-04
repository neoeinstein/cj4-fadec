mod flight_data_recorder;

pub use flight_data_recorder::FlightDataRecorder;

pub fn create_recorder() -> Result<FlightDataRecorder, Box<dyn std::error::Error>> {
    fn test_exists(path: &str) {
        let exists = std::path::Path::new(path).exists();
        println!("Does {} exist? {}", path, exists);
        match std::path::Path::new(path).canonicalize() {
            Ok(path) => println!(
                "Does {} exist? {}",
                path.to_str().unwrap_or("<invalid UTF-8>"),
                path.exists()
            ),
            Err(err) => println!("Cannot canonicalize {}; Error: {}", path, err),
        }
    }

    test_exists(r#"\work"#);
    test_exists(r#".\"#);
    test_exists(r#".\work"#);
    test_exists(r#"work"#);
    test_exists(r#"/work"#);
    test_exists(r#"./"#);
    test_exists(r#"."#);
    test_exists(r#"/"#);
    test_exists(r#"\"#);
    test_exists(r#""#);

    for entry in std::fs::read_dir(r#"\work"#)? {
        println!("Found entry");
        let entry = entry?;
        println!(
            "Entry: {}",
            entry.file_name().to_str().unwrap_or("<non UTF-8>")
        );
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
