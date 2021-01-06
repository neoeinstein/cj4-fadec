//! Testing utilities for systems in this crate

/// Testing support macro for the PID controller
pub mod pid {
    #[doc(inline)]
    pub use crate::pid_step_tests as step_tests;
}

/// Compares two values by equalizing their magnitudes and determining whether
/// the values are equal over the requested number of significant figures
///
/// The final digit is allowed to be off by one unit above or below to account for
/// potential rounding errors.
///
/// ## Examples
///
/// ```
/// # use wt_systems::testing::are_equal_in_significant_figures;
/// assert!(are_equal_in_significant_figures(123.45, 123.0, 3));
/// assert!(are_equal_in_significant_figures(123.45, 123.35, 4));
/// assert!(are_equal_in_significant_figures(123.45, 123.46, 5));
/// assert!(are_equal_in_significant_figures(123.45, 123.45, 5));
/// assert!(are_equal_in_significant_figures(123.45, 123.44, 5));
/// assert!(!are_equal_in_significant_figures(123.45, 1.2345, 1));
/// assert!(!are_equal_in_significant_figures(123.45, 12.345, 1));
/// assert!(!are_equal_in_significant_figures(123.45, 123.34, 4));
/// assert!(!are_equal_in_significant_figures(123.45, 123.434, 5));
/// assert!(!are_equal_in_significant_figures(123.45, 123.470, 5));
/// ```
pub fn are_equal_in_significant_figures(expected: f64, actual: f64, figures: usize) -> bool {
    let magnitude_expected = expected.abs().log10();
    let expected_power = if magnitude_expected < 0. {
        magnitude_expected
    } else {
        magnitude_expected + 1.
    }
    .trunc();

    let magnitude_actual = actual.abs().log10();
    let actual_power = if magnitude_actual < 0. {
        magnitude_actual
    } else {
        magnitude_actual + 1.
    }
    .trunc();

    if expected_power - actual_power > std::f64::EPSILON {
        return false;
    }

    let power = figures as f64 - expected_power;

    let normalized_expected = (expected * 10_f64.powf(power)).round();
    let normalized_actual = (actual * 10_f64.powf(power)).round();

    (normalized_expected - normalized_actual).abs() <= 1. // Allows for a rounding error in the
}

/// Asserts that two values are equal to a certain number of significant figures
///
/// ## Examples
///
/// Successes:
///
/// ```
/// # use wt_systems::testing::assert_equal_in_significant_figures;
/// assert_equal_in_significant_figures(123.45, 123.0, 3);
/// assert_equal_in_significant_figures(123.45, 123.35, 4);
/// assert_equal_in_significant_figures(123.45, 123.46, 5);
/// assert_equal_in_significant_figures(123.45, 123.45, 5);
/// assert_equal_in_significant_figures(123.45, 123.44, 5);
/// ```
///
/// Failures:
///
/// ```should_panic
/// # use wt_systems::testing::assert_equal_in_significant_figures;
/// assert_equal_in_significant_figures(123.45, 1.2345, 1);
/// ```
///
/// ```should_panic
/// # use wt_systems::testing::assert_equal_in_significant_figures;
/// assert_equal_in_significant_figures(123.45, 12.345, 1);
/// ```
///
/// ```should_panic
/// # use wt_systems::testing::assert_equal_in_significant_figures;
/// assert_equal_in_significant_figures(123.45, 123.34, 4);
/// ```
///
/// ```should_panic
/// # use wt_systems::testing::assert_equal_in_significant_figures;
/// assert_equal_in_significant_figures(123.45, 123.434, 5);
/// ```
///
/// ```should_panic
/// # use wt_systems::testing::assert_equal_in_significant_figures;
/// assert_equal_in_significant_figures(123.45, 123.470, 5);
/// ```
#[track_caller]
pub fn assert_equal_in_significant_figures(expected: f64, actual: f64, figures: usize) {
    if !are_equal_in_significant_figures(expected, actual, figures) {
        println!("Expected: {:.*e}", figures - 1, expected);
        println!("Actual:   {:.*e}", figures - 1, actual);
        panic!(
            "Expected and actual values differ within first {} significant figures",
            figures
        );
    }
}

/// Checks whether two values are equal within a certain tolerance
pub fn are_equal_within_epsilon(expected: f64, actual: f64, epsilon: f64) -> bool {
    (expected - actual).abs() < epsilon
}

/// Asserts that two values are equal with a certain tolerance
#[track_caller]
pub fn assert_equal_within_epsilon(expected: f64, actual: f64, epsilon: f64) {
    if !are_equal_within_epsilon(expected, actual, epsilon) {
        println!("Expected:   {}", expected);
        println!("Actual:     {}", actual);
        println!("Difference: {:e}", (expected - actual).abs());
        panic!("Expected and actual values differ by more than {}", epsilon);
    }
}
