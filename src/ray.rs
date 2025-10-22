use crate::vec3::Vec3;

#[derive(Copy, Clone, Debug)]
pub struct Ray {
    pub origin: Vec3,
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
    pub fn at(self, t: f64) -> Vec3 {
        self.origin + self.direction * t
    }
}
