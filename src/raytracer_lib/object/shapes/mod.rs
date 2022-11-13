use crate::raytracer_lib::{raytracer::IntersectionData, utils::ray::Ray};

pub mod sphere;

pub trait RayIntersectable {
    fn intersect(&self, ray: &Ray) -> Option<IntersectionData>;
}
