use super::{RayIntersectable};
use crate::raytracer_lib::{
    object::material::Material,
    utils::{ray::Ray, vector::Vec3}, raytracer::IntersectionData,
};

pub struct Sphere<'a> {
    pub center: Vec3,
    pub radius: f64,
    pub material: &'a Material,
}

impl<'a> RayIntersectable for Sphere<'a> {
    fn intersect(&self, ray: &Ray) -> Option<IntersectionData> {
        let dir = &ray.direction;
        let oc = &ray.origin - &self.center;

        let a = dir * dir;
        let b = 2.0 * (dir * &oc);
        let c = (&oc * &oc) - self.radius * self.radius;
        let mut d = b * b - 4.0 * a * c;

        // is there an intersection?
        if d >= 0.0 {
            d = f64::sqrt(d);

            // the two intersection parameters
            let t1 = (-b - d) / (2.0 * a);
            let t2 = (-b + d) / (2.0 * a);

            let mut intersection_distance = f64::MAX;

            if t1 > 1e-5 && t1 < intersection_distance {
                intersection_distance = t1;
            }
            if t2 > 1e-5 && t2 < intersection_distance {
                intersection_distance = t2;
            }

            // was the intersection not just a numerical problem?
            if intersection_distance != f64::MAX {
                // return intersection data
                let intersection_point = ray(intersection_distance);
                let intersection_normal = &(&intersection_point - &self.center) / self.radius;

                return Some(IntersectionData {
                    distance: intersection_distance,
                    normal: intersection_normal,
                    point: intersection_point,
                    material: self.material
                });
            } else {
                return None;
            }
        } else {
            return None;
        }
    }
}
