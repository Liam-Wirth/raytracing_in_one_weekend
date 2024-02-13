use glam::DVec3;
use std::ops::Range;

use crate::Ray;

pub trait Hittable {
    fn hit(&self, ray: &Ray, interval: Range<f64>) -> Option<HitRecord>;
}
pub struct HitRecord {
    pub point: DVec3,
    pub normal: DVec3,
    pub t: f64,
    pub u: f64,
    pub v: f64,
}

pub struct Sphere {
    center: DVec3,
    radius: f64,
}

impl<T> Hittable for Vec<T>
where
    T: Hittable + Sync,
{
    fn hit(&self, ray: &Ray, interval: Range<f64>) -> Option<HitRecord> {
        let (_closest, hit_record) = self.iter().fold((interval.end, None), |acc, item| {
            if let Some(temp_rec) = item.hit(ray, interval.start..acc.0) {
                (temp_rec.t, Some(temp_rec))
            } else {
                acc
            }
        });

        hit_record
    }
}
impl Hittable for Sphere {
    //TODO: Finish
    fn hit(&self, ray: &Ray, interval: Range<f64>) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = oc.dot(ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;

        if discriminant < 0. {
            return None;
        };

        let discsqrt = discriminant.sqrt();
        let mut root = (-half_b - discsqrt) / a;
        if !interval.contains(&root) {
            root = (-half_b + discsqrt) / a;
            if !interval.contains(&root) {
                return None;
            };
        };
        return None
    }
}

