use std::sync::Arc;

use anyhow::Result;
use camera::CameraBuilder;
use hit_list::HittableList;
use sphere::Sphere;
use vec3::Vec3;

mod vec3;
mod ray;
mod sphere;
mod hit;
mod hit_list;
mod interval;
mod camera;

fn main() -> Result<()> {
    // Camera setup
    let camera = CameraBuilder::default()
        .set_image_width(1920)
        .set_aspect_ratio(16.0 / 9.0)
        .set_max_depth(50)
        .set_anti_aliasing(camera::AntiAliasing::Random(400))
        .build();

    // World
    let mut world = HittableList::default();
    world.add(
        Arc::new(
            Sphere {
                centre: vec3![0.0, 0.0, -1.0],
                radius: 0.5,
            }
        )
    );

    world.add(
        Arc::new(
            Sphere {
                centre: vec3![-3.0, 1.5, -3.0],
                radius: 1.0,
            }
        )
    );

    world.add(
        Arc::new(
            Sphere {
                centre: vec3![0.0, -100.5, -1.0],
                radius: 100.0,
            }
        )
    );

    camera.render("output.png", &world)?;
    Ok(())
}
