use std::default;

use crate::{
    hittable::{self, Hittable},
    Ray,
};
use glam::DVec3;
use indicatif::ProgressIterator;
use itertools::Itertools;
use std::{fs, io};
//pub const ASPECT_RATIO: f64 = 16.0 / 9.0;

//pub const IMAGE_WIDTH: u32 = 480;
//pub const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
//x const MAX_VALUE: u8 = 255;

//pub const VIEWPORT_HEIGHT: f64 = 2.0;
//pub const VIEWPORT_WIDTH: f64 = VIEWPORT_HEIGHT * (IMAGE_WIDTH as f64 / IMAGE_HEIGHT as f64);

//pub const CAMERA_CENTER: DVec3 = DVec3::new(0.0, 0.0, 0.0);

// TODO: Make these values that can be changed based on user input from some sort of UI
//pub const FOCAL_LENGTH: f64 = 1.0;

//pub const VIEWPORT_U: DVec3 = DVec3::new(VIEWPORT_WIDTH, 0.0, 0.0);
//pub const VIEWPORT_V: DVec3 = DVec3::new(0.0, -VIEWPORT_HEIGHT, 0.0);

pub(crate) struct Camera {
    image_width: u32,
    image_height: u32,
    max_value: u8,
    aspect_ratio: f64,
    center: DVec3,
    pixel_delta_u: DVec3,
    pixel_delta_v: DVec3,
    pixel100_loc: DVec3,
    viewport_upper_left: DVec3,
}

impl Camera {
    pub fn new(image_width: u32, aspect_ratio: f64) -> Self {
        let max_value = 255;
        let image_height = (image_width as f64 / aspect_ratio) as u32;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
        let focal_length = 1.0;
        let center = DVec3::ZERO;

        //calculating stuff now:
        let viewport_u = DVec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = DVec3::new(0.0, -viewport_height, 0.0);

        let pixel_delta_u = viewport_u / (image_width as f64);
        let pixel_delta_v = viewport_v / (image_height as f64);

        let viewport_upper_left =
            center - DVec3::new(0., 0., focal_length) - viewport_u / 2. - viewport_v / 2.;
        let pixel100_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        Self {
            image_width,
            image_height,
            max_value,
            aspect_ratio,
            center,
            pixel_delta_u,
            pixel_delta_v,
            pixel100_loc,
            viewport_upper_left,
        }
    }
    pub fn calculate_viewport_upper_left(&self) -> DVec3 {
        self.center - 0.5 * (self.pixel_delta_u + self.pixel_delta_v)
    }
    pub fn ray_color<T>(&self, ray: &Ray, world: &T) -> DVec3
    where
        T: Hittable,
    {
        if let Some(rec) =
            world.hit(&ray, (0.)..f64::INFINITY)
        {
            return 0.5
                * (rec.normal + DVec3::new(1., 1., 1.));
        }

        let unit_direction: DVec3 =
            ray.direction.normalize();
        let a = 0.5 * (unit_direction.y + 1.0);
        return (1.0 - a) * DVec3::new(1.0, 1.0, 1.0)
            + a * DVec3::new(0.5, 0.7, 1.0);
    }
    pub fn render_to_disk<T>(&self, world: &T) -> io::Result<()>
    where
        T: Hittable,
    {
        let pixels = (0..self.image_height)
            .cartesian_product(0..self.image_width)
            .progress_count(self.image_height as u64 * self.image_width as u64)
            .map(|(y, x)| {
                let pixel_center = self.pixel100_loc
                    + (x as f64 * self.pixel_delta_u)
                    + (y as f64 * self.pixel_delta_v);
                let ray_direction = pixel_center - self.center;
                let ray = Ray {
                    origin: self.center,
                    direction: ray_direction,
                };
                let pixel_color = Self::ray_color(&self, &ray, world) * 255.0;
                format!("{} {} {}", pixel_color.x, pixel_color.y, pixel_color.z)
            })
            .join("\n");

        fs::write(
            "output.ppm",
            format!(
                "P3 {} {} \n 
        {} \n
        {}",
                self.image_width, self.image_height, self.max_value, pixels
            ),
        )
    }
}
