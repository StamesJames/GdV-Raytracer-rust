use crate::raytracer_lib::{image::Image, utils::vector::Vec3};

pub struct Material {
    pub ambient: Vec3,
    pub diffuse: Vec3,
    pub specular: Vec3,
    pub shininess: f64,
    pub mirror: f64,
    pub shadowable: bool,
    pub texture_png: Option<Image>,
}

impl Material {
    pub fn new(
        ambient: Vec3,
        diffuse: Vec3,
        specular: Vec3,
        shininess: f64,
        mirror: f64,
        shadowable: bool,
        texture_png: Option<Image>,
    ) -> Self {
        Self {
            ambient,
            diffuse,
            specular,
            shininess,
            mirror,
            shadowable,
            texture_png,
        }
    }
}
