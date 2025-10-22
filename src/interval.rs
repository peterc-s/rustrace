//! Contains the [`Interval`] struct.

use core::f64;

/// The [`Interval`] struct itself that can be used as either
/// a closed or open interval. See
/// [wikipedia](https://en.wikipedia.org/wiki/Interval_(mathematics))
/// for more.
#[derive(Debug, Copy, Clone, Default)]
pub struct Interval {
    /// The infinimum / minimum of the interval.
    pub min: f64,
    /// The supremum / maximum of the interval.
    pub max: f64,
}

#[macro_export]
macro_rules! interval {
    ($min:expr, $max: expr) => {
        Interval {
            min: $min,
            max: $max,
        }
    };
}

#[allow(dead_code)]
impl Interval {
    /// Create a new [`Interval`] with the given `min` and `max`.
    pub fn new(min: f64, max: f64) -> Interval {
        interval![min, max]
    }

    /// Create an [`Interval`] with a minimum of [`f64::INFINITY`] and
    /// a maximum of [`f64::NEG_INFINITY`], essentially containing nothing.
    pub fn empty() -> Interval {
        interval![f64::INFINITY, f64::NEG_INFINITY]
    }

    /// Create an [`Interval`] with a minimum of [`f64::NEG_INFINITY`] and
    /// a maximum of [`f64::INFINITY`], essentially containing everything.
    pub fn universe() -> Interval {
        interval![f64::NEG_INFINITY, f64::INFINITY]
    }

    /// Get the size of `self` (shorthand `self.max - self.min`).
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustrace::{interval, interval::Interval};
    ///
    /// let size_1_interval = interval![0.0, 1.0];
    /// assert_eq!(size_1_interval.size(), 1.0);
    /// ```
    pub fn size(self) -> f64 {
        self.max - self.min
    }

    /// Test if an [`Interval`] contains a point (open-interval check).
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustrace::{interval, interval::Interval};
    ///
    /// let i = interval![0.0, 1.0];
    ///
    /// // Contains endpoints (and values inbetween)
    /// assert!(i.contains(0.0));
    /// assert!(i.contains(1.0));
    ///
    /// // Doesn't contain anything outside of the endpoints
    /// assert!(!i.contains(0.0 - f64::EPSILON));
    /// assert!(!i.contains(1.0 + f64::EPSILON));
    /// ```
    pub fn contains(self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    /// Test if an [`Interval`] contains another (open-interval check).
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustrace::{interval, interval::Interval};
    ///
    /// // Same interval contains itself
    /// let i0 = interval![0.0, 1.0];
    /// let i1 = interval![0.0, 1.0];
    /// assert!(i0.contains_interval(&i1));
    ///
    /// // But this interval isn't contained within `i0`
    /// let i2 = interval![0.0 - f64::EPSILON, 1.0];
    /// assert!(!i0.contains_interval(&i2));
    ///
    /// // However, `i0` is contained within `i2`
    /// assert!(i2.contains_interval(&i0));
    /// ```
    pub fn contains_interval(self, other: &Self) -> bool {
        self.contains(other.min) && self.contains(other.max)
    }

    /// Test if an [`Interval`] overlaps another at any point (open-interval check).
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustrace::{interval, interval::Interval};
    ///
    /// let i0 = interval![0.0, 1.0];
    /// let i1 = interval![-1.0, 0.0];
    ///
    /// // These overlap at the endpoints
    /// assert!(i0.overlaps(&i1));
    ///
    /// let i2 = interval![-1.0, 0.0 - f64::EPSILON];
    ///
    /// // These don't overlap at all
    /// assert!(!i0.overlaps(&i2));
    ///
    /// // But these do
    /// assert!(i1.overlaps(&i2));
    /// ```
    pub fn overlaps(self, other: &Self) -> bool {
        self.min <= other.max && other.min <= self.max
    }

    /// Test if an [`Interval`] surrounds a point (closed-interval check).
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustrace::{interval, interval::Interval};
    ///
    /// let i0 = interval![0.0, 1.0];
    ///
    /// // End points aren't surrounded
    /// assert!(!i0.surrounds(0.0));
    ///
    /// // But anything inside is
    /// assert!(i0.surrounds(0.0 + f64::EPSILON));
    /// ```
    pub fn surrounds(self, x: f64) -> bool {
        self.min < x && x < self.max
    }

    /// Clamp `x` to be contained by the [`Interval`].
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustrace::{interval, interval::Interval};
    ///
    /// let i = interval![0.0, 1.0];
    /// let life = 42.;
    ///
    /// assert_eq!(i.clamp(life), 1.0);
    /// assert_eq!(i.clamp(-life), 0.0);
    /// assert_eq!(i.clamp(0.5), 0.5);
    /// ```
    pub fn clamp(self, x: f64) -> f64 {
        if x < self.min {
            return self.min;
        };
        if x > self.max {
            return self.max;
        };
        x
    }

    /// Unions two [`Interval`]s, mutating `self`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustrace::{interval, interval::Interval};
    ///
    /// let mut i0 = interval![0.0, 1.0];
    /// let i1 = interval![-1.0, 0.0];
    ///
    /// i0.union(&i1);
    ///
    /// assert_eq!(i0.min, -1.0);
    /// assert_eq!(i0.max, 1.0);
    /// ```
    pub fn union(&mut self, other: &Self) {
        self.min = f64::min(self.min, other.min);
        self.max = f64::max(self.max, other.max);
    }

    /// Get the midpoint of the [`Interval`].
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustrace::{interval, interval::Interval};
    ///
    /// let i0 = interval![-1.0, 1.0];
    ///
    /// assert_eq!(i0.mid(), 0.0);
    /// ```
    pub fn mid(self) -> f64 {
        f64::midpoint(self.min, self.max)
    }
}

impl PartialEq for Interval {
    fn eq(&self, other: &Self) -> bool {
        self.min == other.min && self.max == other.max
    }
}
