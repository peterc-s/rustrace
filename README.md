# rustrace
Raytracer in Rust rougly following [Raytracing in One Weekend](https://raytracing.github.io/).

![](final.png)

## Features
- Spheres only right now.
- Materials:
    - Lambertian (diffuse),
    - Dielectric,
    - Metal.
- Anti-Aliasing:
    - Grid,
    - Random.
- Defocus Blur. 
- Parallelised using Rayon.
- BVH tree to speed up intersection detection.

## To-Do
- [ ] Move AABB out of `bvh.rs`
- [ ] Other geometry.
- [ ] Texturing.
- [ ] Lighting.
- [ ] Transforms.
- [ ] Volumetrics.
- [ ] Command line image output configuration.
