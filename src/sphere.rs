use std::sync::Arc;

use vec3::Point3;

use crate::{hittable::{Hittable, HitRecord}, ray::Ray, material::Material};

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
    pub mat_ptr: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(cen: Point3, r: f64, m: Arc<dyn Material>) -> Self {
        Self {
            center: cen,
            radius: r,
            mat_ptr: m,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin() - &self.center;
        let a = r.direction().length_squared();
        let half_b = oc.dot(r.direction());
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0. {
            return None;
        }
        let sqrtd = discriminant.sqrt();

        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }
        let t = root;
        let p = r.at(t);
        let normal = (p - self.center) / self.radius;
        let hit_rec = HitRecord::new(p, t, normal, *r, &*self.mat_ptr);
        Some(hit_rec)
    }
}
