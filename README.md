# rustrace
Raytracer in Rust rougly following [Raytracing in One Weekend](https://raytracing.github.io/) plus some of my own additions.

![](final.png)

Rendered at 1000 samples per pixel, 1920x1080, in 13 minutes on an laptop (13th Gen i5-1350P, 16GB DDR5 @ 4800 MT/s, powersave governor - unplugged!).

![](final_small.png)

Rendered at 500 samples per pixel, 1280x720, in just under 3 minutes (same laptop).

## Features
- Geometry:
    - Spheres
    - Triangles
    - Triangular meshes
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
- [ ] Other geometry.
    - [x] Triangles
    - [ ] Triangular meshes from OBJs
        - [x] Basic OBJ parsing
        - [x] Normal interpolation
        - [ ] Clean up implementation
        - [ ] Look at optimisations
            - [x] Surface area heuristic splitting
        - [ ] Setting position, scale, etc. (transforms) for meshes
- [ ] Texturing.
- [ ] Lighting.
- [ ] Transforms.
- [ ] Volumetrics.
- [ ] Command line image output configuration.
