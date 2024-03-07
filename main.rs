// TODO: Maybe rewrite the code to use my own vec3 implementation instead of DVec3
pub mod hittable;
pub mod objects;
pub mod camera;
pub mod materials;
pub mod ray;
use crate::camera::Camera;
use hittable::Hittable;
use indicatif::ProgressIterator;

use glam::DVec3;
use itertools::Itertools;
use objects::sphere::Sphere;
use std::{fs, io};

fn main() -> io::Result<()> {
    let mut world:  Vec<Box<dyn Hittable + Sync>> = Vec::new();
    world.push(Box::new(Sphere::new(DVec3::new(0.0, 0.0, -1.0), 0.5)));
    world.push(Box::new(Sphere::new(DVec3::new(4.0, 3.0, -3.0), 0.5)));
    world.push(Box::new(Sphere::new(DVec3::new(9.0, -2.0, -5.0), 0.1)));
    let camera = Camera::new(600, 16.0 / 9.0);
    camera.render_to_disk(&world)?;
    Ok(())
}







