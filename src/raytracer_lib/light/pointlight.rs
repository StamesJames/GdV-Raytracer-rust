use crate::raytracer_lib::utils::vector::{Vec3, Color};
pub struct Pointlight{
    pub position: Vec3,
    pub color: Color,
}

impl Pointlight{
    pub fn new(position: Vec3, color: Color) -> Self{
        Self{
            position,
            color,
        }
    }
}