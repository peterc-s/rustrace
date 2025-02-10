use std::rc::Rc;

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
        .set_image_width(400)
        .set_aspect_ratio(16.0 / 9.0)
        .build();

    // World
    let mut world = HittableList::default();
    world.add(
        Rc::new(
            Sphere {
                centre: vec3![0.0, 0.0, -1.0],
                radius: 0.5,
            }
        )
    );
    world.add(
        Rc::new(
            Sphere {
                centre: vec3![0.0, -100.5, -1.0],
                radius: 100.0,
            }
        )
    );

    camera.render("output.png", &world)?;
    Ok(())
}
