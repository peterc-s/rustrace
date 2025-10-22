//! Contains a simple [`Vec3`] struct implementation.

use std::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
};

use image::Rgb;

use crate::interval;
use interval::Interval;
use rand::{rngs::SmallRng, Rng};

/// The [`Vec3`] itself, simply an abstraction over a `3`-size array with
/// most operation traits defined.
#[derive(Clone, Copy, Debug, Default)]
pub struct Vec3 {
    /// The inner array. Elements be accessed by indexing the [`Vec3`] itself.
    pub e: [f64; 3],
}

#[macro_export]
macro_rules! vec3 {
    ($x:expr, $y:expr, $z:expr $(,)?) => {
        Vec3 { e: [$x, $y, $z] }
    };
}

#[allow(dead_code)]
impl Vec3 {
    /// Create a new [`Vec3`] with the specified values.
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { e: [x, y, z] }
    }

    /// Calculate the length squared of the [`Vec3`].
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustrace::{vec3, vec3::Vec3};
    ///
    /// let v = vec3![2.0, 2.0, 2.0];
    ///
    /// assert_eq!(v.length_squared(), 12.0);
    /// ```
    pub fn length_squared(&self) -> f64 {
        let x: f64 = self[0];
        let y: f64 = self[1];
        let z: f64 = self[2];
        x * x + y * y + z * z
    }

    /// Calculate the length of the [`Vec3`] (shorthand for `self.length_squared().sqrt()`).
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustrace::{vec3, vec3::Vec3};
    ///
    /// let v = vec3![3.0, 4.0, 0.0];
    ///
    /// assert_eq!(v.length(), 5.0);
    /// ```
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    /// Create a unit vector in the same direction as `self` (shorthand for `self /
    /// self.length()`).
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustrace::{vec3, vec3::{Vec3, cross}};
    ///
    /// let v = vec3![5.0, 5.0, 0.0];
    ///
    /// // Epsilon-close due to floating point arithmetic.
    /// assert!(
    ///     (1.0 - f64::EPSILON ..=1.0 + f64::EPSILON)
    ///         .contains(
    ///             &v.unit().length(),
    ///         )
    /// );
    /// assert_eq!(cross(&v, &v.unit()).length(), 0.0);
    /// ````
    pub fn unit(&self) -> Self {
        *self / self.length()
    }

    /// Reflect a [`Vec3`] given a surface normal to reflect on.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustrace::{vec3, vec3::Vec3};
    ///
    /// let v = vec3![1.0, 1.0, 0.0];
    /// let r = v.reflect(&vec3![-1.0, 0.0, 0.0]);
    ///
    /// assert_eq!(r, vec3![-1.0, 1.0, 0.0]);
    /// ```
    pub fn reflect(&self, norm: &Vec3) -> Self {
        *self - (*norm * 2.0 * dot(self, norm))
    }

    // TODO: enforce unit vector
    /// Refract a unit [`Vec3`] given a surface normal and an `etai_over_etat` (ratio
    /// of refractive indices).
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustrace::{vec3, vec3::Vec3};
    ///
    /// // Must be a unit vector
    /// let v = vec3![1.0, 1.0, 0.0].unit();
    /// let no_refraction = v.refract(&vec3![-1.0, 0.0, 0.0], 1.0);
    /// let refraction = v.refract(&vec3![-1.0, 0.0, 0.0], (2.0_f64).sqrt());
    ///
    /// // Floating point imprecision.
    /// assert!((no_refraction - v).length().abs() < f64::EPSILON);
    /// assert_eq!(refraction, vec3![0.0, 1.0, 0.0]);
    /// ```
    pub fn refract(&self, norm: &Vec3, etai_over_etat: f64) -> Self {
        let cos_theta = dot(&-*self, norm).min(1.0);
        let r_out_perp = (*self + *norm * cos_theta) * etai_over_etat;
        let r_out_parallel = *norm * -(1.0 - r_out_perp.length_squared()).abs().sqrt();
        r_out_perp + r_out_parallel
    }

    /// Test that a [`Vec3`] is near the `0` vector with an epsilon of `1e-8`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustrace::{vec3, vec3::Vec3};
    ///
    /// let mut v = vec3![1.0 * f64::EPSILON, -10.0 * f64::EPSILON, 1e-9];
    ///
    /// assert!(v.near_zero());
    ///
    /// v[0] = 1e-8;
    ///
    /// assert!(!v.near_zero());
    /// ```
    pub fn near_zero(self) -> bool {
        let eps = 1e-8;
        (self[0].abs() < eps) && (self[1].abs() < eps) && (self[2].abs() < eps)
    }

    /// Generate a random [`Vec3`] with elements between `0.0` and `1.0` inclusive.
    pub fn random(rng: &mut SmallRng) -> Self {
        vec3![
            rng.random_range(0.0..=1.0),
            rng.random_range(0.0..=1.0),
            rng.random_range(0.0..=1.0)
        ]
    }

    /// Generate a random [`Vec3`] with elements between `min` and `max`.
    pub fn random_in(min: f64, max: f64, rng: &mut SmallRng) -> Self {
        vec3![
            rng.random_range(min..=max),
            rng.random_range(min..=max),
            rng.random_range(min..=max)
        ]
    }

    /// Generate a random unit [`Vec3`].
    pub fn random_unit(rng: &mut SmallRng) -> Self {
        loop {
            let p = Vec3::random_in(-1.0, 1.0, rng);
            let lensq = p.length_squared();
            if 1e-160 < lensq && lensq <= 1.0 {
                return p / lensq.sqrt();
            }
        }
    }

    /// Generate a random unit [`Vec3`] on a hemisphere according to the
    /// outward normal.
    pub fn random_on_hemi(normal: Vec3, rng: &mut SmallRng) -> Self {
        let on_unit_sphere = Vec3::random_unit(rng);
        if dot(&on_unit_sphere, &normal) > 0.0 {
            on_unit_sphere
        } else {
            -on_unit_sphere
        }
    }

    /// Generate a random unit [`Vec3`] in a unit disc. Mostly for defocus blur.
    pub fn random_in_unit_disc(rng: &mut SmallRng) -> Self {
        loop {
            let p = vec3![
                rng.random_range(-1.0..=1.0),
                rng.random_range(-1.0..=1.0),
                0.0
            ];
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    /// Convert a [`Vec3`] to an [`image::Rgb`] with linear-to-gamma
    /// conversion.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustrace::{vec3, vec3::Vec3};
    /// use image::Rgb;
    ///
    /// let v = vec3![1.0, 0.0, 0.0];
    /// let c = v.to_rgb();
    ///
    /// assert_eq!(c[0], 255);
    /// assert_eq!(c[1], 0);
    /// assert_eq!(c[2], 0);
    /// ```
    pub fn to_rgb(self) -> Rgb<u8> {
        let intensity = interval![0.000, 0.999];
        Rgb([
            (intensity.clamp(linear_to_gamma(self[0])) * 256.0) as u8,
            (intensity.clamp(linear_to_gamma(self[1])) * 256.0) as u8,
            (intensity.clamp(linear_to_gamma(self[2])) * 256.0) as u8,
        ])
    }
}

// TODO: make this a Vec3 method and replace all usages
/// Calculate the dot product of two [`Vec3`]s.
///
/// # Example
///
/// ```rust
/// use rustrace::{vec3, vec3::{Vec3, dot}};
///
/// let v0 = vec3![1.0, 0.0, 0.0];
/// let v1 = vec3![0.0, 1.0, 0.0];
/// let v2 = vec3![3.0, 0.0, 0.0];
///
/// assert_eq!(dot(&v0, &v1), 0.0);
/// assert_eq!(dot(&v0, &v2), 3.0);
/// ```
pub fn dot(u: &Vec3, v: &Vec3) -> f64 {
    u[0] * v[0] + u[1] * v[1] + u[2] * v[2]
}

// TODO: make this a Vec3 method and replace all usages
/// Calculate the cross product of two [`Vec3`]s.
///
/// # Example
///
/// ```rust
/// use rustrace::{vec3, vec3::{Vec3, cross}};
///
/// let v0 = vec3![5.0, 5.0, 0.0];
/// let v1 = vec3![1.0, 1.0, 0.0];
///
/// assert_eq!(cross(&v0, &v1).length(), 0.0);
/// ```
pub fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
    vec3![
        u[1] * v[2] - u[2] * v[1],
        u[2] * v[0] - u[0] * v[2],
        u[0] * v[1] - u[1] * v[0]
    ]
}

/// Convert linear values to gamma values. Essentially an [f64::sqrt] with
/// a check that the operand is positive.
fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0.0 {
        linear_component.sqrt()
    } else {
        0.0
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        vec3![self[0] + other[0], self[1] + other[1], self[2] + other[2]]
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = vec3![self[0] + other[0], self[1] + other[1], self[2] + other[2]]
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        vec3![self[0] - other[0], self[1] - other[1], self[2] - other[2]]
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        *self = vec3![self[0] - other[0], self[1] - other[1], self[2] - other[2]]
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        vec3![self[0] * rhs[0], self[1] * rhs[1], self[2] * rhs[2]]
    }
}

impl<K> Mul<K> for Vec3
where
    K: Mul + Into<f64> + Copy,
{
    type Output = Self;

    fn mul(self, scalar: K) -> Self::Output {
        vec3![
            self[0] * scalar.into(),
            self[1] * scalar.into(),
            self[2] * scalar.into()
        ]
    }
}

impl<K> MulAssign<K> for Vec3
where
    K: Mul + Into<f64> + Copy,
{
    fn mul_assign(&mut self, scalar: K) {
        *self = vec3![
            self[0] * scalar.into(),
            self[1] * scalar.into(),
            self[2] * scalar.into()
        ]
    }
}

impl<K> Div<K> for Vec3
where
    K: Mul + Into<f64> + Copy,
{
    type Output = Self;

    fn div(self, scalar: K) -> Self::Output {
        vec3![
            self[0] / scalar.into(),
            self[1] / scalar.into(),
            self[2] / scalar.into()
        ]
    }
}

impl<K> DivAssign<K> for Vec3
where
    K: Mul + Into<f64> + Copy,
{
    fn div_assign(&mut self, scalar: K) {
        *self = vec3![
            self[0] / scalar.into(),
            self[1] / scalar.into(),
            self[2] / scalar.into()
        ]
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self {
        vec3![-self[0], -self[1], -self[2]]
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, idx: usize) -> &f64 {
        &self.e[idx]
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, idx: usize) -> &mut f64 {
        &mut self.e[idx]
    }
}

impl PartialEq for Vec3 {
    fn eq(&self, other: &Self) -> bool {
        self.e == other.e
    }
}

impl Eq for Vec3 {}
