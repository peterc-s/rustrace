//! Contains the [`Ray`] struct which is a simple abstraction over two [`Vec3`]s for
//! the rays starting position and direction of travel.

use crate::vec3::Vec3;

/// The [`Ray`] struct itself.
#[derive(Copy, Clone, Debug)]
pub struct Ray {
    /// The origin of the ray in world-space.
    pub origin: Vec3,

    // TODO: enforce that this is a unit vector - preferably without
    // runtime overhead.
    /// The unit direction vector of the ray.
    pub direction: Vec3,
}

#[macro_export]
macro_rules! ray {
    ($o:expr, $d:expr $(,)?) => {
        Ray {
            origin: $o,
            direction: $d,
        }
    };
}

impl Ray {
    /// Parameterises the [ray](Ray) linearly with `t` to calculate the
    /// [`Vec3`] point the [ray](Ray) is at with that `t` value.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustrace::{ray::Ray, ray, vec3::Vec3, vec3};
    ///
    /// let ray = ray!(
    ///     vec3![0.0, 0.0, 0.0],
    ///     vec3![1.0, 2.0, 0.0],
    /// );
    ///
    /// assert_eq!(ray.at(10.0), vec3![10.0, 20.0, 0.0]);
    /// ```
    pub fn at(self, t: f64) -> Vec3 {
        self.origin + self.direction * t
    }
}
