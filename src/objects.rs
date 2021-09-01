use crate::vecs::{Point, Color, Ray, Vec3};

pub trait LightBlocker {
    fn dist_to(&self, ray:&Ray) -> f64; //negative or 0 if no hit
    fn hit(&self, ray: &Ray) -> Ray;
}

pub trait Material {
    fn reflection(&self, incoming_ray:&Ray, hit_ray:&Ray) -> Ray;
    fn color(&self, reflection_color:&Color) -> Color;
}


pub struct Sphere {
    pub center: Point,
    pub radius: f64,
}

pub type Mirror = Vec3<f64>;

impl Material for Mirror {
    fn reflection(&self, incoming_ray:&Ray, hit_ray:&Ray) -> Ray {
        let normal = hit_ray.direction;
        let par_part = Vec3::project(&incoming_ray.direction, &normal);
        let new_dir = incoming_ray.direction - par_part * 2.0;
        Ray{direction: new_dir, origin: hit_ray.origin}
    }

    fn color(&self, reflection_color:&Color) -> Color {
        Color{
            x: reflection_color.x * self.x,
            y: reflection_color.y * self.y,
            z: reflection_color.z * self.z
        }
    }
}

impl Sphere {
    pub fn new(x:f64,y:f64,z:f64,r:f64) -> Self {
        Self {center: Point{x,y,z}, radius:r}
    }
}

impl LightBlocker for Sphere {
    fn dist_to(&self, ray: &Ray) -> f64 {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let b = 2.0 * Vec3::dot(&oc, &ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminent = b*b - 4.0*a*c;
        if discriminent < 0.0 {
            -1.0
        } else {
            (-b - discriminent.sqrt()) / (2.0*a)
        }
    }
    fn hit(&self, ray: &Ray) -> Ray {
        let point_of_contact = ray.origin + ray.direction * self.dist_to(ray);
        let normal = point_of_contact - self.center;
        Ray{origin: point_of_contact, direction: normal}
    }
}
