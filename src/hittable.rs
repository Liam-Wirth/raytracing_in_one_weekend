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
    front_face: bool,
}

// NOTE: I don't need a hittable list because rust just has vectors
impl<T> Hittable for Vec<T>

where
    T: Hittable + Sync,
{
    //NOTE: A difference from the book is the fact that the book/guide uses some T_min and T_max value, while in rust I can just declare a Range
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
impl Hittable for Box<dyn Hittable + Sync> {
    fn hit(&self, ray: &Ray, interval: Range<f64>) -> Option<HitRecord> {
        (**self).hit(ray, interval)
    }
}

impl HitRecord {
    pub fn new(point: DVec3, normal: DVec3, t: f64, u: f64, v: f64, front_face: bool) -> Self {
        HitRecord {
            point,
            normal,
            t,
            u,
            v,
            front_face,
        }
    }
    pub fn new_with_front_face_calc(
        point: DVec3,
        outward_normal: DVec3,
        t: f64,
        u: f64,
        v: f64,
        ray: &Ray,
    ) -> Self {
        let (normal, front_face) = HitRecord::set_face_normal(ray, &outward_normal);
        HitRecord {
            point,
            normal,
            t,
            u,
            v,
            front_face,
        }
    }   
    ///Function that calculates whether the normal is front or rear facing, returns the normal and a bool, true if front face and false if otherwise
    pub fn set_face_normal(ray: &Ray, outward_normal: &DVec3) -> (DVec3, bool) {
        let front_face = ray.direction.dot(*outward_normal) < 0.;
        let normal = if front_face {
            *outward_normal
        } else {
            -*outward_normal
        };
        (normal, front_face)
    }
    /*
    We can set things up so that normals always point “outward” from the surface,
    or always point against the incident ray. This decision is determined by whether 
    you want to determine the side of the surface at the time of geometry intersection or
    at the time of coloring. In this book we have more material types than we have geometry
    types, so we'll go for less work and put the determination at geometry time. 
    This is simply a matter of preference, and you'll see both implementations in the literature.  
    */
}
