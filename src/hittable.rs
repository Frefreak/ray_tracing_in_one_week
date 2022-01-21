use std::sync::Arc;

use vec3::{Point3, Vec3};

use crate::{ray::Ray, material::Material};

#[derive(Clone)]
pub struct HitRecord<'a> {
    pub p: Point3,
    pub normal: Vec3,
    pub material: &'a dyn Material,
    pub t: f64,
    pub front_face: bool,
}

impl<'a> HitRecord<'a> {
    pub fn new(p: Vec3, t: f64, normal: Vec3, r: Ray, material: &'a dyn Material) -> Self {
        let front_face = r.direction().dot(&normal) < 0.0;
        let normal = if front_face {
            normal
        } else {
            -normal
        };
        Self {
            p,
            normal,
            material,
            t,
            front_face,
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}


pub struct HittableList {
    objects: Vec<Arc<dyn Hittable>>,
}
unsafe impl Send for HittableList {}
unsafe impl Sync for HittableList {}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: vec![],
        }
    }
    pub fn clear(&mut self) {
        self.objects.clear();
    }
    pub fn add(&mut self, object: Arc<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut temp_rec = None;
        let mut closest_so_far = t_max;
        for obj in &self.objects {
            if let Some(hit_rec) = obj.hit(r, t_min, closest_so_far) {
                closest_so_far = hit_rec.t.clone();
                temp_rec = Some(hit_rec);
            }
        }
        temp_rec
    }
}
