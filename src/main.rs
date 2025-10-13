use anyhow::Result;
use bvh::BVHTree;
use camera::CameraBuilder;
use hit_list::HittableList;
use material::{Dielectric, Lambertian, Material, Metal};
use mimalloc::MiMalloc;
use rand::{rngs::SmallRng, Rng, SeedableRng};
use sphere::Sphere;
use vec3::Vec3;

use crate::bvh::SplitAxis;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

mod bvh;
mod camera;
mod hit;
mod hit_list;
mod interval;
mod material;
mod ray;
mod sphere;
mod utils;
mod vec3;

fn main() -> Result<()> {
    // Camera setup
    let camera = CameraBuilder::default()
        .set_image_width(400)
        .set_aspect_ratio(16.0 / 9.0)
        .set_max_depth(50)
        .set_anti_aliasing(camera::AntiAliasing::Random(500))
        .set_vfov(20)
        .set_look_from(vec3![13.0, 2.0, 3.0])
        .set_look_at(vec3![0.0, 0.0, 0.0])
        .set_v_up(vec3![0.0, 1.0, 0.0])
        .set_defocus_angle(0.6)
        .build();

    // Scene
    let mut hit_list = HittableList::default();
    let material_ground = Box::new(Lambertian::new(vec3![0.8, 0.8, 0.0]));

    hit_list.add(Box::new(Sphere {
        centre: vec3![0.0, -1000.0, 0.0],
        radius: 1000.0,
        mat: material_ground,
    }));

    let material1 = Box::new(Dielectric::new(1.5));
    hit_list.add(Box::new(Sphere {
        centre: vec3![0.0, 1.0, 0.0],
        radius: 1.0,
        mat: material1,
    }));

    let material2 = Box::new(Lambertian::new(vec3![0.4, 0.2, 0.1]));
    hit_list.add(Box::new(Sphere {
        centre: vec3![-4.0, 1.0, 0.0],
        radius: 1.0,
        mat: material2,
    }));

    let material3 = Box::new(Metal::new(vec3![0.7, 0.6, 0.5], 0.0));
    hit_list.add(Box::new(Sphere {
        centre: vec3![4.0, 1.0, 0.0],
        radius: 1.0,
        mat: material3,
    }));

    let mut rng = SmallRng::from_os_rng();
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.random_range(0.0..=1.0);
            let centre = vec3![
                a as f64 + 0.9 * rng.random_range(0.0..=1.0),
                0.2,
                b as f64 + 0.9 * rng.random_range(0.0..=1.0)
            ];

            if (centre - vec3![4.0, 0.2, 0.0]).length() > 0.9 {
                let mat: Box<dyn Material> = match choose_mat {
                    c if (c < 0.8) => {
                        let albedo = Vec3::random(&mut rng);
                        Box::new(Lambertian::new(albedo))
                    }
                    c if (c < 0.95) => {
                        let albedo = Vec3::random_in(0.5, 1.0, &mut rng);
                        let fuzz = rng.random_range(0.0..=0.5);
                        Box::new(Metal::new(albedo, fuzz))
                    }
                    _ => Box::new(Dielectric::new(1.5)),
                };

                hit_list.add(Box::new(Sphere {
                    centre,
                    radius: 0.2,
                    mat,
                }));
            };
        }
    }

    let world = BVHTree::from_hit_list(hit_list, SplitAxis::Y);
    // assert!(world.verify());

    camera.render("output.png", &world)?;
    Ok(())
}
