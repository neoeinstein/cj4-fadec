//! # simconnect-sys
//! 
//! Utilities for interoperating with the SimConnect API

#![warn(
    missing_docs,
    unused_import_braces,
    unused_imports,
    unused_qualifications
)]
#![deny(
    missing_debug_implementations,
    unused_must_use
)]

pub mod ffi;
mod simconnect;

pub use simconnect::*;