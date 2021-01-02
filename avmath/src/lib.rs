//! # <strong>Av</strong>iation <strong>Math</strong>ematics (AvMath)
//! 
//! A library of functions and utilities surrounding calculations
//! relevant to aviation and simulation.

#![warn(
    missing_docs,
    unused_import_braces,
    unused_imports,
    unused_qualifications
)]
#![deny(missing_debug_implementations, unused_must_use)]

pub mod atmosphere;
pub mod calculations;
pub mod constants;
mod types;

pub use types::*;
