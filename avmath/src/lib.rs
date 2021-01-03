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
#![forbid(unsafe_code)]

// #[macro_use]
// extern crate uom;

pub mod calculations;
pub mod constants;
pub mod isa;
pub mod si;
