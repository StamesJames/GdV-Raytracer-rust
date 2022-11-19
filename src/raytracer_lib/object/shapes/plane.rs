use std::sync::Arc;

use crate::raytracer_lib::{utils::vector::Vec3, object::material::Material};

use super::RayIntersectable;

pub struct Plane {
    pub center: Vec3,
    pub normal: Vec3,
    pub material: Arc<Material>,
}

impl Plane {
    pub fn new(center: Vec3, normal: Vec3, material: Arc<Material>) -> Self { Self { center, normal, material } }
}

impl RayIntersectable for Plane {
    fn intersect(&self, _ray: &crate::raytracer_lib::utils::ray::Ray) -> Option<crate::raytracer_lib::raytracer::IntersectionData> {
        None
    }
}