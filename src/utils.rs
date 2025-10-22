//! Module for putting widely useful functions, traits, etc. with no specific
//! purpose.

use std::f64::consts::PI;

/// Convert degrees to radians.
///
/// # Example
/// ```rust
/// use std::f64::consts::PI;
/// use rustrace::utils::deg_to_rad;
///
/// assert_eq!(deg_to_rad(360.0), 2.0 * PI);
/// ```
pub fn deg_to_rad(deg: f64) -> f64 {
    (deg * PI) / 180.0
}
