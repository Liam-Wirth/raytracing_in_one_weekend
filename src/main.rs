// TODO: Maybe rewrite the code to use my own vec3 implementation instead of DVec3
use indicatif::ProgressIterator;

use itertools::Itertools;
use std::{fs, io};
use glam::DVec3;
const ASPECT_RATIO: f64 = 16.0/9.0;

const IMAGE_WIDTH: u32 = 1280;
const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64/ASPECT_RATIO) as u32;
const MAX_VALUE: u8 = 255;

const VIEWPORT_HEIGHT: f64 = 2.0;
const VIEWPORT_WIDTH: f64 = VIEWPORT_HEIGHT * (IMAGE_WIDTH as f64/IMAGE_HEIGHT as f64);

const CAMERA_CENTER: DVec3 = DVec3::new(0.0, 0.0, 0.0);

// TODO: Make these values that can be changed based on user input from some sort of UI
const FOCAL_LENGTH: f64 = 1.0;

const VIEWPORT_U: DVec3 = DVec3::new(VIEWPORT_WIDTH, 0.0, 0.0);
const VIEWPORT_V: DVec3 = DVec3::new(0.0, -VIEWPORT_HEIGHT, 0.0);

fn main() -> io::Result<()> {
    let pixel_delta_u: DVec3 = VIEWPORT_U / IMAGE_WIDTH as f64;
    let pixel_delta_v: DVec3 = VIEWPORT_V / IMAGE_HEIGHT as f64;

    let viewport_upper_left = calculate_viewport_upper_left();

    let pixel100_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);


    let pixels = (0..IMAGE_HEIGHT)
        .cartesian_product(0..IMAGE_WIDTH)
        .progress_count(IMAGE_HEIGHT as u64 * IMAGE_WIDTH as u64) //Progress Bar Baby
        .map(|(y, x)| {
            let pixel_center = pixel100_loc + (x as f64 * pixel_delta_u) + (y as f64 * pixel_delta_v);
            let ray_direction = pixel_center - CAMERA_CENTER;
            let ray = Ray {
                origin: CAMERA_CENTER,
                direction: ray_direction,
            };
           let pixel_color = ray.color() * 255.0; 
           format!("{} {} {}", pixel_color.x , pixel_color.y , pixel_color.z )
        })
        .join("\n");
    println!("{}", pixels);
    fs::write(
        "output.ppm",
        format!(
            "P3 {IMAGE_WIDTH} {IMAGE_HEIGHT} \n 
        {MAX_VALUE} \n
        {pixels}",
        ),
    )
}

fn calculate_viewport_upper_left() -> DVec3 {
    CAMERA_CENTER - DVec3::new(0.,0., FOCAL_LENGTH) - VIEWPORT_U/2. - VIEWPORT_V/2.
}


struct Ray {
    origin: DVec3,
    direction: DVec3,
}

impl Ray {
    fn at(&self, t: f64) -> DVec3 {
        self.origin + t * self.direction
    }
    fn color(&self) -> DVec3 {
    let t = hit_sphere(&DVec3::new(0.0, 0.0, -10.0), 4., self); // Dereference the &self parameter
        if t > 0. {
            let N = (self.at(t) - DVec3::new(0.0, 0.0, -10.0)).normalize();
            return 0.5 * (N + 1.0)
        };
        let unit_direction: DVec3 =
            self.direction.normalize();
        let a = 0.5 * (unit_direction.y + 1.0);
        return(1.0 - a) * DVec3::new(1.0, 1.0, 1.0) + a * DVec3::new(0.5, 0.7, 1.0);
    }

}
fn hit_sphere( center: &DVec3, radius: f64, ray: &Ray) -> f64 {
    let oc: DVec3 = ray.origin - *center;
    let a = ray.direction.dot(ray.direction);
    let b = 2.0 * oc.dot(ray.direction);
    let c = oc.dot(oc) - radius * radius;
    let discriminant = b*b -4.0 *a*c; //PYTHAG JUMPSCARE!!!! HOLY SHIT I USED IT IN REAL LIFE BABY!!!!!!!
    if discriminant < 0. {
         -1.0
    } else {
       (-b - discriminant.sqrt() /(2.0 *a))
    }
}