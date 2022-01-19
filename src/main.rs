use std::sync::Arc;

use camera::Camera;
use hittable::{HitRecord, Hittable, HittableList};
use material::{Lambertian, Metal, Dielectric};
use ppm::PPM;
use ray::Ray;
use sphere::Sphere;
use utils::random_double;
use vec3::{v3, Color};

mod ray;
mod hittable;
mod sphere;
mod camera;
mod material;

const MAX_DEPTH: u32 = 50;
const NSAMPLES: usize = 100;

fn main() {
    // image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 800_u32;
    let image_height = (image_width as f64 / aspect_ratio) as u32;
    let mut image = PPM::new(image_width, image_height);

    // World
    let mut world = HittableList::new();
    let material_ground = Arc::new(Lambertian::new(&v3!(0.8, 0.8, 0.)));
    let material_center = Arc::new(Lambertian::new(&v3!(0.1, 0.2, 0.5)));
    let material_left = Arc::new(Dielectric::new(1.5));
    let material_right = Arc::new(Metal::new(&v3!(0.8, 0.6, 0.2), 0.0));
    world.add(Arc::new(Sphere::new(v3!(0., -100.5, -1.), 100., material_ground)));
    world.add(Arc::new(Sphere::new(v3!(0., 0., -1.), 0.5, material_center)));
    world.add(Arc::new(Sphere::new(v3!(-1., 0., -1.), 0.5, material_left.clone())));
    world.add(Arc::new(Sphere::new(v3!(-1., 0., -1.), -0.4, material_left)));
    world.add(Arc::new(Sphere::new(v3!(1., 0., -1.), 0.5, material_right)));

    // camera
    let camera = Camera::new();

    // render
    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let mut color = v3!(0., 0., 0.);
            // println!("{} {}", j, i);
            for _i in 0..NSAMPLES {
                let u = (i as f64 + random_double()) / (image_width as f64 - 1.);
                let v = (j as f64 + random_double()) / (image_height as f64 - 1.);
                let r = camera.get_ray(u, v);
                color = color + ray_color(&r, &world, MAX_DEPTH);
            }
            image.set_with_samples((image_height - j - 1) as usize, i as usize, color, NSAMPLES);
        }
    }
    image.save("test.ppm").unwrap();
}

fn ray_color(ray: &Ray, world: &dyn Hittable, depth: u32) -> Color {
    if depth <= 0 {
        return v3!(0., 0., 0.);
    }
    let mut rec = HitRecord::new();
    if world.hit(ray, 0.001, utils::INFINITY, &mut rec) {
        let mut scattered = Ray::new(v3!(0., 0., 0.), v3!(0., 0., 0.));
        let mut attenuation = v3!(0., 0., 0.);
        if rec.mat_ptr.as_ref().unwrap().scatter(ray, &rec, &mut attenuation, &mut scattered) {
            return attenuation * ray_color(&scattered, world, depth - 1);
        }
        return v3!(0., 0., 0.);
    }
    let unit_direction = ray.direction().unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.);
    (1. - t) * v3!(1., 1., 1.) + t * v3!(0.5, 0.7, 1.0)
}
