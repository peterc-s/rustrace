use anyhow::Result;
use mimalloc::MiMalloc;
use rand::{rngs::SmallRng, Rng, SeedableRng};
use rustrace::{
    aabb::SplitAxis,
    bvh::BVHTree,
    camera::{AntiAliasing, CameraBuilder},
    hit_list::HittableList,
    material::{Dielectric, Lambertian, Material, Metal},
    mesh::Mesh,
    sphere::Sphere,
    vec3,
    vec3::Vec3,
};

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

fn main() -> Result<()> {
    // Camera setup
    let camera = CameraBuilder::default()
        .set_image_width(300)
        .set_aspect_ratio(1.)
        .set_max_depth(50)
        .set_anti_aliasing(AntiAliasing::Random(300))
        .set_vfov(30)
        .set_look_from(vec3![13.0, 2.0, 3.0])
        .set_look_at(vec3![0.0, 0.0, 0.0])
        .set_v_up(vec3![0.0, 1.0, 0.0])
        .set_defocus_angle(0.6)
        .build();

    // Scene
    let mut hit_list = HittableList::default();

    let material_teapot = Box::new(Metal::new(vec3![1., 1., 1.], 0.0));
    // let material_teapot = Box::new(Lambertian::new(vec3![1., 1., 1.]));
    // let material_teapot = Box::new(Dielectric::new(1.5));

    // TODO: get path in a better way than this
    let teapot = Mesh::from_obj("objs/teapot.obj", material_teapot)?;
    hit_list.add(Box::new(teapot));

    let material_ground = Box::new(Lambertian::new(vec3![0.8, 0.8, 0.0]));

    hit_list.add(Box::new(Sphere {
        centre: vec3![0.0, -1002.0, 0.0],
        radius: 1000.0,
        mat: material_ground,
    }));
    //
    // let material1 = Box::new(Dielectric::new(1.5));
    // hit_list.add(Box::new(Sphere {
    //     centre: vec3![0.0, 1.0, 0.0],
    //     radius: 1.0,
    //     mat: material1.clone(),
    // }));
    //
    // let material2 = Box::new(Lambertian::new(vec3![0.4, 0.2, 0.1]));
    // hit_list.add(Box::new(Sphere {
    //     centre: vec3![-4.0, 1.0, 0.0],
    //     radius: 1.0,
    //     mat: material2,
    // }));
    //
    // hit_list.add(Box::new(Triangle::new(
    //     [vec3![7., 2., 0.], vec3![6., 1., 0.], vec3![7., 2., 1.]],
    //     None,
    //     material1,
    // )));
    //
    // let material3 = Box::new(Metal::new(vec3![0.7, 0.6, 0.5], 0.0));
    // hit_list.add(Box::new(Sphere {
    //     centre: vec3![4.0, 1.0, 0.0],
    //     radius: 1.0,
    //     mat: material3,
    // }));

    let mut rng = SmallRng::from_os_rng();
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.random_range(0.0..=1.0);
            let centre = vec3![
                a as f64 + 0.9 * rng.random_range(0.0..=1.0),
                0.2 - 2.,
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

    let world = BVHTree::from_hit_list(hit_list);
    // assert!(world.verify());

    camera.render("output.png", &world)?;
    Ok(())
}
