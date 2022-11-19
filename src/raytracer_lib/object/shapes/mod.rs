use crate::raytracer_lib::{raytracer::IntersectionData, utils::ray::Ray};

use self::sphere::Sphere;

pub mod sphere;
pub mod plane;

pub trait RayIntersectable {
    fn intersect(&self, ray: &Ray) -> Option<IntersectionData>;
}

pub enum Object {
    Sphere(Sphere)
}