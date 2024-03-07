// TODO: Maybe rewrite the code to use my own vec3 implementation instead of DVec3
pub mod camera;
pub mod hittable;
pub mod materials;
pub mod objects;
use crate::camera::Camera;
use hittable::Hittable;
use indicatif::ProgressIterator;

use glam::DVec3;
use itertools::Itertools;
use materials::random_on_hemisphere;
use objects::sphere::Sphere;
use std::{fs, io};

fn main() -> io::Result<()> {
    let mut world: Vec<Box<dyn Hittable + Sync>> = Vec::new();
    world.push(Box::new(Sphere::new(DVec3::new(0.0, 0.0, -1.0), 0.5)));
    world.push(Box::new(Sphere::new(DVec3::new(4.0, 3.0, -3.0), 0.5)));
    world.push(Box::new(Sphere::new(DVec3::new(9.0, -2.0, -5.0), 0.1)));
    let camera = Camera::new(600, 16.0 / 9.0);
    camera.render_to_disk(&world)?;
    Ok(())
}

pub struct Ray {
    origin: DVec3,
    direction: DVec3,
}

impl Ray {
    fn at(&self, t: f64) -> DVec3 {
        self.origin + t * self.direction
    }
    fn color<T>(&self, depth: i32, world: &T) -> DVec3
    where
        T: Hittable,
    {
        if depth <= 0 {
            return DVec3::new(0., 0., 0.);
        }
        if let Some(rec) = world.hit(&self, (0.001)..f64::INFINITY) {
            let direction: DVec3 = random_on_hemisphere(&rec.normal);
            let ray = Ray {
                origin: rec.point,
                direction,
            };
            return 0.5 * ray.color(depth - 1, world);
        }

        let unit_direction: DVec3 = self.direction.normalize();
        let a = 0.5 * (unit_direction.y + 1.0);
        return (1.0 - a) * DVec3::new(1.0, 1.0, 1.0) + a * DVec3::new(0.5, 0.7, 1.0);
    }
}
