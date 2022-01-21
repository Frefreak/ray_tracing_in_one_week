use std::sync::{Arc, Mutex};

use camera::Camera;
use hittable::{Hittable, HittableList};
use material::{Dielectric, Lambertian, Metal, Material};
use ppm::PPM;
use ray::Ray;
use sphere::Sphere;
use utils::{random_double, random_double_range};
use vec3::{v3, Color, Vec3};
use rayon::prelude::*;

mod camera;
mod hittable;
mod material;
mod ray;
mod sphere;

const MAX_DEPTH: u32 = 50;
const NSAMPLES: usize = 100;

fn main() {
    // image
    let aspect_ratio = 3.0 / 2.0;
    let image_width = 800_u32;
    let image_height = (image_width as f64 / aspect_ratio) as u32;
    let image = PPM::new(image_width, image_height);
    let the_image = Arc::new(Mutex::new(image));

    // World
    let world = random_scene();

    // camera
    let lookfrom = v3!(13., 2., 3.);
    let lookat = v3!(0., 0., 0.);
    let vup = v3!(0., 1., 0.);
    let dist_to_focus = 10.;
    let aperture = 0.1;
    let camera = Camera::new(
        lookfrom, lookat ,vup, 20., aspect_ratio, aperture, dist_to_focus);

    // render
    (0..image_height).collect::<Vec<_>>().par_iter().rev().for_each(|j| {
    // for j in (0..image_height).rev() {
        println!("{}/{}", j, image_height);
        for i in 0..image_width {
            let mut color = v3!(0., 0., 0.);
            // println!("{} {}", j, i);
            for _i in 0..NSAMPLES {
                let u = (i as f64 + random_double()) / (image_width as f64 - 1.);
                let v = (*j as f64 + random_double()) / (image_height as f64 - 1.);
                let r = camera.get_ray(u, v);
                color = color + ray_color(&r, &world, MAX_DEPTH);
            }
            the_image.lock().unwrap().set_with_samples((image_height - j - 1) as usize, i as usize, color, NSAMPLES);
        }
    });
    the_image.lock().unwrap().save("test.ppm").unwrap();
}

fn random_scene() -> HittableList {
    let mut world = HittableList::new();
    let ground_material = Arc::new(Lambertian::new(&v3!(0.5, 0.5, 0.5)));
    world.add(Arc::new(Sphere::new(v3!(0., -1000., 0.), 1000., ground_material)));

    for a in -11..=11 {
        for b in -11..=11 {
            let choose_mat = random_double();
            let center = v3!(a as f64 + 0.9 * random_double(), 0.2, b as f64 + 0.9 * random_double());
            if (center - v3!(4., 0.2, 0.)).length() > 0.9 {
                let sphere_material: Arc<dyn Material> = if choose_mat < 0.8 {
                    let albedo = Vec3::random() * Vec3::random();
                    Arc::new(Lambertian::new(&albedo))
                } else if choose_mat < 0.95 {
                    let albedo = Vec3::random_range(0.5, 1.);
                    let fuzz = random_double_range(0., 0.5);
                    Arc::new(Metal::new(&albedo, fuzz))
                } else {
                    Arc::new(Dielectric::new(1.5))
                };
                world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
            }
        }
    }
    let material1 = Arc::new(Dielectric::new(1.5));
    world.add(Arc::new(Sphere::new(v3!(0., 1., 0.), 1., material1)));
    let material2 = Arc::new(Lambertian::new(&v3!(0.4, 0.2, 0.1)));
    world.add(Arc::new(Sphere::new(v3!(-4., 1., 0.), 1., material2)));
    let material3 = Arc::new(Metal::new(&v3!(0.7, 0.6, 0.5), 0.0));
    world.add(Arc::new(Sphere::new(v3!(4., 1., 0.), 1., material3)));
    world
}

fn ray_color(ray: &Ray, world: &dyn Hittable, depth: u32) -> Color {
    if depth <= 0 {
        return v3!(0., 0., 0.);
    }
    if let Some(rec) = world.hit(ray, 0.001, utils::INFINITY) {
        let mut scattered = Ray::new(v3!(0., 0., 0.), v3!(0., 0., 0.));
        let mut attenuation = v3!(1., 1., 1.);
        if rec
            .material
            .scatter(ray, &rec, &mut attenuation, &mut scattered)
        {
            return attenuation * ray_color(&scattered, world, depth - 1);
        }
        return v3!(0., 0., 0.);
    }
    let unit_direction = ray.direction().unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.);
    (1. - t) * v3!(1., 1., 1.) + t * v3!(0.5, 0.7, 1.0)
}
