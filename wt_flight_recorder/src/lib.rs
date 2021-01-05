//! Flight Data Recorder
//!
//! This module allows for easily recording important flight information for later analysis and
//! debugging.
//!
//! Data is output to a file in the `\work` directory with the initialization time as a prefix
//! and `.msgpack.gz` as the extension. Files produced are serialized as [MsgPack][] and then
//! processed by a gzip encoder. Over the course of a session, a log may be broken across
//! multiple files in order to keep the size of each individual log file reasonable. Files
//! belonging to the same session will have the same prefix, but have an incrementing number
//! associated with them.
//!
//! These files can then be later processed by the `wt_flight_to_csv` processor to convert the
//! data into a CSV format, suitable for further analysis.
//!
//!   [MsgPack]: https://msgpack.org
//!
//! ```no_run
//! use wt_flight_recorder::FlightDataRecorder;
//!
//! #[derive(serde::Serialize, serde::Deserialize)]
//! struct MyData {
//!     time: f64,
//!     thrust: f64,
//!     output: f64,
//! }
//!
//! let mut recorder: FlightDataRecorder<MyData> = FlightDataRecorder::new().unwrap();
//!
//! let publish_result = recorder.publish(&MyData {
//!     time: 0.0,
//!     thrust: 1000.0,
//!     output: 0.9,
//! });
//!
//! if let Err(e) = publish_result {
//!     eprintln!("Unable to log event: {}", e);
//! }
//! ```

mod flight_data_recorder;

pub use flight_data_recorder::FlightDataRecorder;

/// Monkey-patched replacement for the broken MSFS `__wasilibc_find_relpath`
/// implementation
///
/// Big thanks goes to _devsnek_ for working with me to figure out how to get
/// around the issue in the broken MSFS WASI implementation.
#[cfg(target_os = "wasi")]
#[no_mangle]
unsafe extern "C" fn __wasilibc_find_relpath(
    path: *const std::os::raw::c_char,
    relative_path: *mut *const std::os::raw::c_char,
) -> std::os::raw::c_int {
    static mut PREOPENS: Vec<(wasi::Fd, String)> = vec![];
    static mut PREOPENS_AVAILABLE: bool = false;
    static mut EMPTY: *const std::os::raw::c_char =
        b".\0" as *const u8 as *const std::os::raw::c_char;

    if !PREOPENS_AVAILABLE {
        PREOPENS_AVAILABLE = true;

        const START_FD: wasi::Fd = 3; // skip stdio 0,1,2
        for fd in START_FD.. {
            let mut prestat = std::mem::MaybeUninit::uninit();
            let r = wasi::wasi_snapshot_preview1::fd_prestat_get(fd, prestat.as_mut_ptr());
            if r == wasi::ERRNO_BADF {
                break;
            }
            assert!(r == wasi::ERRNO_SUCCESS);
            let prestat = prestat.assume_init();

            if prestat.tag == wasi::PREOPENTYPE_DIR {
                let mut prefix = Vec::new();
                prefix.resize(prestat.u.dir.pr_name_len, 0);
                let r = wasi::wasi_snapshot_preview1::fd_prestat_dir_name(
                    fd,
                    prefix.as_mut_ptr(),
                    prestat.u.dir.pr_name_len,
                );
                assert!(r == wasi::ERRNO_SUCCESS);
                PREOPENS.push((
                    fd,
                    std::ffi::CString::from_vec_unchecked(prefix)
                        .into_string()
                        .unwrap(),
                ));
            }
        }
    }

    let rust_path = std::ffi::CStr::from_ptr(path).to_str().unwrap();
    for (fd, prefix) in &PREOPENS {
        if rust_path.starts_with(prefix) {
            if rust_path.len() == prefix.len() {
                *relative_path = EMPTY;
            } else {
                *relative_path = path.add(prefix.len());
                loop {
                    if **relative_path == '\\' as i8 {
                        *relative_path = (*relative_path).add(1);
                    } else if **relative_path == '.' as i8 && *(*relative_path.add(1)) == '\\' as i8
                    {
                        *relative_path = (*relative_path).add(2);
                    } else {
                        break;
                    }
                }
                if **relative_path == 0 {
                    *relative_path = EMPTY;
                }
            }
            return *fd as std::os::raw::c_int;
        }
    }

    return -1;
}
