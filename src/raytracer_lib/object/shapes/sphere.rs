use std::sync::Arc;

use super::RayIntersectable;
use crate::raytracer_lib::{
    object::material::Material,
    raytracer::IntersectionData,
    utils::{ray::Ray, vector::Vec3},
};

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub material: Arc<Material>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, material: Arc<Material>) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl RayIntersectable for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<IntersectionData> {
        let dir = &ray.direction;
        // println!("dir is {:?}", dir);
        let oc = &ray.origin - &self.center;
        // println!("oc is {:?}", oc);
        
        let a = dir * dir;
        // println!("a is {}", a);
        let b = 2.0 * (dir * &oc);
        // println!("b is {}", b);
        let c = (&oc * &oc) - (self.radius * self.radius);
        let mut d = b * b - 4.0 * a * c;
        // println!("d is {}", d);
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
                    material: self.material.clone(),
                });
            } else {
                return None;
            }
        } else {
            return None;
        }
    }
}
