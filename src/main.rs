use std::sync::Arc;

use anyhow::Result;
use camera::CameraBuilder;
use hit_list::HittableList;
use material::{Dielectric, Lambertian, Metal};
use sphere::Sphere;
use vec3::Vec3;

mod vec3;
mod ray;
mod sphere;
mod hit;
mod hit_list;
mod interval;
mod camera;
mod material;

fn main() -> Result<()> {
    // Camera setup
    let camera = CameraBuilder::default()
        .set_image_width(600)
        .set_aspect_ratio(16.0 / 9.0)
        .set_max_depth(20)
        .set_anti_aliasing(camera::AntiAliasing::Random(200))
        .set_look_from(vec3![-2.0, 2.0, 1.0])
        .set_look_at(vec3![0.0, 0.0, -1.0])
        .set_v_up(vec3![0.0, 1.0, 0.0])
        .set_vfov(20)
        .build();

    // Materials
    let material_ground = Arc::new(
        Lambertian::new(
            vec3![0.8, 0.8, 0.0]
        )    
    );

    let material_centre = Arc::new(
        Metal::new(
            vec3![0.1, 0.2, 0.5],
            1.0,
        )
    );

    let material_left = Arc::new(
        Dielectric::new(
            1.50,
        )    
    );

    let material_bubble = Arc::new(
        Dielectric::new(
            1.00 / 1.50,
        )
    );

    let material_right = Arc::new(
        Metal::new(
            vec3![0.8, 0.6, 0.2],
            0.3,
        )
    );

    // World
    let mut world = HittableList::default();
    world.add(
        Arc::new(
            Sphere {
                centre: vec3![0.0, -100.5, -1.0],
                radius: 100.0,
                mat: material_ground,
            }
        )
    );

    world.add(
        Arc::new(
            Sphere {
                centre: vec3![0.0, 0.0, -1.2],
                radius: 0.5,
                mat: material_centre,
            }
        )
    );

    world.add(
        Arc::new(
            Sphere {
                centre: vec3![-1.0, 0.0, -1.0],
                radius: 0.5,
                mat: material_left,
            }
        )
    );

    world.add(
        Arc::new(
            Sphere {
                centre: vec3![-1.0, 0.0, -1.0],
                radius: 0.4,
                mat: material_bubble,
            }
        )
    );

    world.add(
        Arc::new(
            Sphere {
                centre: vec3![1.0, 0.0, -1.0],
                radius: 0.5,
                mat: material_right,
            }
        )
    );

    camera.render("output.png", &world)?;
    Ok(())
}
