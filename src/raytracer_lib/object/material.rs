use crate::raytracer_lib::{utils::vector::Vec3, image::Image};


pub struct Material {
    pub ambient: Vec3,
    pub diffuse: Vec3,
    pub specular: Vec3,
    pub shininess: f64,
    pub mirror: f64,
    pub shadowable: bool,
    pub texture_png: Option<Image>,
}