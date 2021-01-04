mod flight_data_recorder;

pub use flight_data_recorder::FlightDataRecorder;

fn try_open(path: &str) {
    match std::fs::File::open(path) {
        Ok(_) => println!("Found one!: {}", path),
        Err(err) => println!("Nope: {}: {:?}", path, err),
    }
}

fn try_create(path: &str) {
    match std::fs::File::create(path) {
        Ok(_) => println!("Found one!: {}", path),
        Err(err) => println!("Nope: {}: {:?}", path, err),
    }
}

pub fn create_recorder() -> Result<FlightDataRecorder, Box<dyn std::error::Error>> {
    // try_open("./SimObjects/Airplanes/Asobo_CJ4/engines.cfg");
    // try_open("SimObjects/Airplanes/Asobo_CJ4/engines.cfg");
    // try_open("SimObjects\\Airplanes\\Asobo_CJ4\\engines.cfg");
    // try_open("\\SimObjects\\Airplanes\\Asobo_CJ4\\engines.cfg");
    // try_open(".\\SimObjects\\Airplanes\\Asobo_CJ4\\engines.cfg");
    // try_open("./thirdparty_licenses.txt");
    // try_open("thirdparty_licenses.txt");
    // try_open("\\thirdparty_licenses.txt");
    // try_open(".\\thirdparty_licenses.txt");
    // try_create("/work/test");
    // try_create("./work/test");
    // try_create("\\work\\test");
    // try_create(".\\work\\test");
    // try_create("/work\\test");
    // try_create("./work\\test");
    // try_create("\\work/test");
    // try_create(".\\work/test");
    // try_create("work/test");
    // try_create("work\\test");
    // try_create("\\\\work\\\\test");
    //
    // let p = std::path::Path::new(".");
    // for entry in p.read_dir()? {
    //     let entry = entry?;
    //     println!("{:?}", entry.file_name().to_str());
    // }
    // let p = std::path::Path::new("/work");
    // println!("work? {}", p.is_dir());
    // let p2 = p.join("fdr.msgpack");
    // let file = std::fs::File::create(p2)?;

    let file = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .truncate(true)
        .create(true)
        .open("\\work\\fdr.msgpack")?;
    let encoder = flate2::write::GzEncoder::new(file, flate2::Compression::best());
    Ok(FlightDataRecorder::new(encoder))
}
