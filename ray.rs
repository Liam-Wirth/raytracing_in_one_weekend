
pub struct Ray {
    origin: DVec3,
    direction: DVec3,
}

impl Ray {
    fn at(&self, t: f64) -> DVec3 {
        self.origin + t * self.direction
    }
    fn color<T>(&self, world: &T)  -> DVec3 
    where T:  Hittable,
    {
        if let Some(rec) = world.hit(self, 0.0..f64::INFINITY) {
            return 0.5 * (rec.normal + DVec3::new(1.0, 1.0, 1.0));
        }
        
        let unit_direction: DVec3 = self.direction.normalize();
        let a = 0.5 * (unit_direction.y + 1.0);
        return (1.0 - a) * DVec3::new(1.0, 1.0, 1.0) + a * DVec3::new(0.5, 0.7, 1.0);
    }
}
