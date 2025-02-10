use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign};

use image::Rgb;

use interval::Interval;
use rand::{rngs::SmallRng, Rng};
use crate::interval;

#[derive(Clone, Copy, Debug, Default)]
pub struct Vec3 {
    pub e: [f64; 3],
}

#[macro_export]
macro_rules! vec3 {
    ($a:expr, $b:expr, $c:expr) => {
        Vec3 { e: [$a, $b, $c] }
    }
}

#[allow(dead_code)]
impl Vec3 {
    pub fn new(a: f64, b: f64, c: f64) -> Vec3 {
        Vec3 {
            e: [a, b, c]
        }
    }

    pub fn length_squared(self) -> f64 {
        let x: f64 = self[0].into();
        let y: f64 = self[1].into();
        let z: f64 = self[2].into();
        x * x + y * y + z * z
    }

    pub fn length(self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn unit(self) -> Self {
        self / self.length()
    }

    pub fn random(rng: &mut SmallRng) -> Self {
        vec3![rng.random_range(0.0..=1.0), rng.random_range(0.0..=1.0), rng.random_range(0.0..=1.0)]
    }

    pub fn random_in(min: f64, max: f64, rng: &mut SmallRng) -> Self {
        vec3![rng.random_range(min..=max), rng.random_range(min..=max), rng.random_range(min..=max)]
    }

    pub fn random_unit(rng: &mut SmallRng) -> Self {
        loop {
            let p = Vec3::random_in(-1.0, 1.0, rng);
            let lensq = p.length_squared();
            if 1e-160 < lensq && lensq <= 1.0 {
                return p / lensq.sqrt();
            }
        }
    }

    pub fn random_on_hemi(normal: Vec3, rng: &mut SmallRng) -> Self {
        let on_unit_sphere = Vec3::random_unit(rng);
        if dot(&on_unit_sphere, &normal) > 0.0 {
            on_unit_sphere
        } else {
            -on_unit_sphere
        }
    }

    pub fn to_rgb(self) -> Rgb<u8> {
        let intensity = interval![0.000, 0.999];
        Rgb([
            (intensity.clamp(self[0]) * 256.0) as u8,
            (intensity.clamp(self[1]) * 256.0) as u8,
            (intensity.clamp(self[2]) * 256.0) as u8,
        ])
    }

}

pub fn dot(u: &Vec3, v: &Vec3) -> f64 {
    u[0] * v[0] +
    u[1] * v[1] +
    u[2] * v[2]
}

pub fn cross<T>(u: &Vec3, v: &Vec3) -> Vec3 {
    vec3![
        u[1] * v[2] - u[2] * v[1],
        u[2] * v[0] - u[0] * v[2],
        u[0] * v[1] - u[1] * v[0]
    ]
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        vec3![
            self[0] + other[0],
            self[1] + other[1],
            self[2] + other[2]
        ]
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = vec3![
            self[0] + other[0],
            self[1] + other[1],
            self[2] + other[2]
        ]
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        vec3![
            self[0] - other[0],
            self[1] - other[1],
            self[2] - other[2]
        ]
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        *self = vec3![
            self[0] - other[0],
            self[1] - other[1],
            self[2] - other[2]
        ]
    }
}

impl <K> Mul<K> for Vec3
where K: Mul + Into<f64> + Copy
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

impl <K> MulAssign<K> for Vec3
where K: Mul + Into<f64> + Copy
{
    fn mul_assign(&mut self, scalar: K) {
        *self = vec3![
            self[0] * scalar.into(),
            self[1] * scalar.into(),
            self[2] * scalar.into()
        ]
    }
}

impl <K> Div<K> for Vec3
where K: Mul + Into<f64> + Copy
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

impl <K> DivAssign<K> for Vec3
where K: Mul + Into<f64> + Copy
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
        vec3![
            -self[0],
            -self[1],
            -self[2]
        ]
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
