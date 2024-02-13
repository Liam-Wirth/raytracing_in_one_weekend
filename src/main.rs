use indicatif::ProgressIterator;
use itertools::Itertools;
use std::{fs, io};
use glam::DVec3;
const ASPECT_RATIO: f64 = 16.0/9.0;

const IMAGE_WIDTH: u32 = 400;
const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64/ASPECT_RATIO) as u32;
const MAX_VALUE: u8 = 255;

const VIEWPORT_HEIGHT: f64 = 2.0;
const VIEWPORT_WIDTH: f64 = VIEWPORT_HEIGHT * (IMAGE_WIDTH as f64/IMAGE_HEIGHT as f64);

fn main() -> io::Result<()> {
    //let pixel_delta_u: DVec3 = VIEWPORT_U / IMAGE_WIDTH as f64;
    //let pixel_delta_v: DVec3 = VIEWPORT_V / IMAGE_HEIGHT as f64;
    let pixels = (0..IMAGE_HEIGHT)
        .cartesian_product(0..IMAGE_WIDTH)
        .progress_count(IMAGE_HEIGHT as u64 * IMAGE_WIDTH as u64) //Progress Bar Baby
        .map(|(y, x)| {
            let r = x as f64 / (IMAGE_WIDTH - 1) as f64;
            let g = y as f64 / (IMAGE_HEIGHT - 1) as f64;
            let b = 0.0;
            format!("{} {} {}", r * 255.0, g * 255.0, b * 255.0)
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


struct Ray {
    origin: DVec3,
    direction: DVec3,
}

impl Ray {
    fn at(&self, t: f64) -> DVec3 {
        self.origin + t * self.direction
    }
    fn color(&self) -> DVec3 {
        let unit_direction: DVec3 =
            self.direction.normalize();
        let a = 0.5 * (unit_direction.y + 1.0);
        return(1.0 - a) * DVec3::new(1.0, 1.0, 1.0) + a * DVec3::new(0.5, 0.7, 1.0);
    }
}
