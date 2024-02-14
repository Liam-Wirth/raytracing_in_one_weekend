use glam::DVec3;
use std::ops::Range;
use crate::{hittable::{self, HitRecord, Hittable}, Ray};

pub struct Sphere {
    pub center: DVec3,
    pub radius: f64,
}
impl Sphere {
    //TODO miiight need other methods for this struct, but I don't really think so tbh.
    pub fn new(center: DVec3, radius: f64) -> Self {
        Sphere { center, radius }
    }
}   

impl Hittable for Sphere {
    fn hit (&self, ray: &Ray, interval: Range<f64>) -> Option<HitRecord> {
        let oc: DVec3 = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b =  oc.dot(ray.direction);
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = half_b * half_b - ( a * c); //PYTHAG JUMPSCARE!!!! HOLY SH
        if discriminant < 0 {return None};
        let dsqrt = discriminant.sqrt();

        //find the nearest root that lies within the acceptable range
        let mut root = (-half_b - dsqrt) / a;

        if root <= interval.start || root >= interval.end {
            root = ((-half_b + dsqrt) /a);
         if root <= interval.start || root >= interval.end {
             return None;
         }
        }
        let t = root;
        let point = ray.at(t);
        let outward_normal = (point - self.center) / self.radius;
        let (u, v) = hittable::get_sphere_uv(outward_normal);
          Some(HitRecord {
            t: root,
            point: ray.at(root),
            normal: (ray.at(root) - self.center) / self.radius,
            u: 0.0,
            v: 0.0,
         })
        }
         

}