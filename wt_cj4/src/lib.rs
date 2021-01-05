//! # Working Title systems for the Cessna Citation CJ4

#![warn(
    missing_docs,
    unused_import_braces,
    unused_imports,
    unused_qualifications
)]
#![deny(missing_debug_implementations, unused_must_use)]
#![forbid(unsafe_code)]

pub mod control_params;
pub mod engines;
mod fadec;
mod state;

pub use fadec::FadecController;
pub use state::{Aircraft, Engine, EngineReadings, Environment, Instruments, Snapshot};
