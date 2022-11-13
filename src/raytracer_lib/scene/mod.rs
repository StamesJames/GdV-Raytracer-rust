use super::{
    camera::perspective_cam::PerspectiveCam, light::pointlight::Pointlight,
    utils::vector::Color, object::shapes::sphere::Sphere
};

pub struct Scene<'a> {
    pub cam: PerspectiveCam,
    pub recursion_depth: u32,
    pub background_color: Color,
    pub ambient_light: Color,
    pub lights: Vec<Pointlight>,
    pub objects: Vec<Sphere<'a>>,
}
