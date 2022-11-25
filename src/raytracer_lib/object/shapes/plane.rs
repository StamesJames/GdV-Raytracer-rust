use std::sync::Arc;

use crate::raytracer_lib::{object::material::Material, utils::vector::Vec3, raytracer::IntersectionData};

use super::RayIntersectable;

pub struct Plane {
    pub center: Vec3,
    pub normal: Vec3,
    pub material: Arc<Material>,
}

impl Plane {
    pub fn new(center: Vec3, normal: Vec3, material: Arc<Material>) -> Self {
        let normal = normal.normalize();
        Self {
            center,
            normal,
            material,
        }
    }
}

impl RayIntersectable for Plane {
    fn intersect(
        &self,
        ray: &crate::raytracer_lib::utils::ray::Ray,
    ) -> Option<crate::raytracer_lib::raytracer::IntersectionData> {
        let n = &self.normal;
        let o = &ray.origin;
        let dir = &ray.direction;
        let c = &self.center;
        let nd = n.dot(&dir);
        let dist = n.dot(&c);
        if nd != 0. {
            let t = (dist - n.dot(o)) / nd;
            if t > 1e-5 {
                let point = ray(t);
                let distance = Vec3::metric_distance(&point, &o);
                return Some(IntersectionData{
                    point: point,
                    normal: n.clone(),
                    distance: distance,
                    material: self.material.clone(),
                });
            }
        }
        None
    }
}
