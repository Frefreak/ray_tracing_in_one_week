use utils::random_double;
use vec3::{Color, random_unit_vector, reflect, random_in_unit_sphere, v3, refract};

use crate::{hittable::HitRecord, ray::Ray};

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool;
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(c: &Color) -> Self {
        Self {
            albedo: *c,
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let mut scatter_direction = rec.normal + random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }
        *scattered = Ray::new(rec.p, scatter_direction);
        *attenuation = self.albedo;
        return true;
    }
}

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(c: &Color, f: f64) -> Self {
        Self {
            albedo: *c,
            fuzz: if f < 1. {f} else {1.},
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let reflected = reflect(&r_in.direction().unit_vector(), &rec.normal);
        *scattered = Ray::new(rec.p, reflected + self.fuzz * random_in_unit_sphere());
        *attenuation = self.albedo;
        scattered.direction().dot(&rec.normal) > 0.
    }
}

pub struct Dielectric {
    ir: f64,
}

impl Dielectric {
    pub fn new(ir: f64) -> Self {
        Self {
            ir,
        }
    }

    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        // Use schlick's approximation for reflectance.
        let r0 = (1. - ref_idx) / (1. + ref_idx);
        let r0 = r0 * r0;
        r0 + (1. - r0) * (1. - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        *attenuation = v3!(1., 1., 1.);
        let refraction_ratio = if rec.front_face {1. / self.ir} else {self.ir};
        let unit_direction = r_in.direction().unit_vector();
        let cos_theta = if -unit_direction.dot(&rec.normal) > 1. {
            -unit_direction.dot(&rec.normal)
        } else {
            1.
        };
        let sin_theta = (1. - cos_theta * cos_theta).sqrt();
        let cannot_refract = refraction_ratio * sin_theta > 1.;
        let direction = if cannot_refract || Dielectric::reflectance(cos_theta, refraction_ratio) > random_double() {
            reflect(&unit_direction, &rec.normal)
        } else {
            refract(&unit_direction, &rec.normal, refraction_ratio)
        };
        *scattered = Ray::new(rec.p, direction);
        return true;
    }
}
