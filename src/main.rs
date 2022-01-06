use ppm::PPM;
use ray::Ray;
use vec3::{v3, Color, Point3};

mod ray;

fn main() {
    // image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 800_u32;
    let image_height = (image_width as f64 / aspect_ratio) as u32;
    let mut image = PPM::new(image_width, image_height);

    // camera
    let viewport_height = 4.0;
    let viewport_width = viewport_height * aspect_ratio;
    let focal_length = -1.;

    let origin = v3!(0., 0., 0.);
    let horizontal = v3!(viewport_width, 0., 0.);
    let vertical = v3!(0., viewport_height, 0.);
    // unintuitive
    let lower_left_corner = origin - horizontal / 2. - vertical / 2. - v3!(0., 0., focal_length);
    // let lower_left_corner = v3!(-2., -1., -focal_length);

    // render
    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let u = i as f64 / (image_width as f64 - 1.);
            let v = j as f64 / (image_height as f64 - 1.);
            let ray = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );
            let color = ray_color(&ray);
            image.set((image_height - j - 1) as usize, i as usize, color);
        }
    }
    image.save("test.ppm").unwrap();
}

fn ray_color(ray: &Ray) -> Color {
    if hit_sphere(&v3!(0., 0., -1.), 0.5, ray) {
        return v3!(1., 0., 0.);
    }
    let unit = ray.direction().unit_vector();
    let t = 0.5 * (unit.y() + 1.);
    return (1.0 - t) * v3!(1., 1., 1.) + t * v3!(0.5, 0.7, 1.0);
}

fn hit_sphere(center: &Point3, radius: f64, ray: &Ray) -> bool {
    let oc = ray.origin() - center;
    let a = ray.direction().dot(ray.direction());
    let b = 2.0 * ray.direction().dot(&oc);
    let c = oc.dot(&oc) - radius * radius;
    b * b - 4. * a * c > 0.
}
