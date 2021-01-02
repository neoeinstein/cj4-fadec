//! Testing utilities for systems in this crate

/// Testing support macro for the PID controller
pub mod pid {
    pub use crate::pid_step_tests as step_tests;
}

/// Compares two values by equalizing their magnitudes and determining whether
/// the values are equal over the requested number of significant figures
pub fn are_equal_in_significant_figures(expected: f64, actual: f64, figures: u8) -> bool {
    let power =
        i32::from(figures).saturating_sub(expected.log10().max(actual.log10()).trunc() as i32);
    let mul = 10_f64.powi(power);

    (expected * mul).trunc() == (actual * mul).trunc()
}

/// Asserts that two values are equal to a certain number of significant figures
#[track_caller]
pub fn assert_equal_in_significant_figures(expected: f64, actual: f64, figures: u8) {
    if !are_equal_in_significant_figures(expected, actual, figures) {
        assert_eq!(
            expected, actual,
            "Expected and actual differ within first {} significant figures",
            figures
        );
    }
}
