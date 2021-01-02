//! This module provides a safe layer for interacting with the legacy Gauge
//! API provided with Microsoft Flight Simulator.

#![warn(
    missing_docs,
    unused_import_braces,
    unused_imports,
    unused_qualifications
)]
#![deny(missing_debug_implementations, unused_must_use)]

pub mod ffi;

mod types;

pub use types::*;

#[doc(hidden)]
pub mod once_cell {
    pub use once_cell::sync::Lazy;
}
