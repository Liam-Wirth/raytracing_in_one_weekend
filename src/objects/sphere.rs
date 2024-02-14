use glam::DVec3;
use std::{f64::consts::PI, ops::Range};
use crate::{hittable::{HitRecord, Hittable}, Ray};

pub struct Sphere {
    pub center: DVec3,
    pub radius: f64,
}
impl Sphere {
    //TODO miiight need other methods for this struct, but I don't really think so tbh.
    pub fn new(center: DVec3, radius: f64) -> Self {
        Sphere { center, radius }
    }
    pub fn get_sphere_uv(&self, p: DVec3) -> (f64, f64) {
        // p: a given point on the sphere of radius one, centered at the origin.
        // u: returned value [0,1] of angle around the Y axis from X=-1.
        // v: returned value [0,1] of angle from Y=-1 to Y=+1.
        //     <1 0 0> yields <0.50 0.50>       <-1  0  0> yields <0.00 0.50>
        //     <0 1 0> yields <0.50 1.00>       < 0 -1  0> yields <0.50 0.00>
        //     <0 0 1> yields <0.25 0.50>       < 0  0 -1> yields <0.75 0.50>

        let theta = (-p.y).acos();
        let phi = (-p.z).atan2(p.x) + PI;

        let u = phi / (2. * PI);
        let v = theta / PI;
        (u, v)
    }
}   

impl Hittable for Sphere {
    fn hit (&self, ray: &Ray, interval: Range<f64>) -> Option<HitRecord> {
        let oc: DVec3 = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b =  oc.dot(ray.direction);
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = half_b * half_b - ( a * c); //PYTHAG JUMPSCARE!!!! HOLY SH
        if discriminant < 0. {return None};
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
        let (u, v) = Sphere::get_sphere_uv(&self, outward_normal);
        Some(HitRecord::new_with_front_face_calc(point, outward_normal, t, u, v, ray))
        }
         

}