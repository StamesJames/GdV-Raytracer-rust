use self::shapes::{sphere::Sphere, plane::Plane, RayIntersectable};


pub mod material;
pub mod shapes;

pub enum Object {
    Sphere(Sphere),
    Plane(Plane),
}

impl RayIntersectable for Object {
    fn intersect(&self, ray: &super::utils::ray::Ray) -> Option<super::raytracer::IntersectionData> {
        match self {
            Object::Sphere(s) => s.intersect(ray),
            Object::Plane(p) => p.intersect(ray),
        }
    }
}