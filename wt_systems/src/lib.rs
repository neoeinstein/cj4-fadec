//! # Working Title General Purpose Systems
//! 
//! A collection of systems that can be used by multiple aircraft

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

mod pid;

pub use pid::{PidConfiguration, PidController};
pub mod testing;
