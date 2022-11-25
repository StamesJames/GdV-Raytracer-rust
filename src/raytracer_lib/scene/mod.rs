use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
    str::SplitWhitespace,
    sync::Arc,
};

use super::{
    camera::perspective_cam::PerspectiveCam,
    light::pointlight::Pointlight,
    object::{
        material::Material,
        shapes::{plane::Plane, sphere::Sphere},
        Object,
    },
    utils::vector::{Color, Vec3},
};

pub struct Scene {
    pub cam: PerspectiveCam,
    pub recursion_depth: u32,
    pub background_color: Color,
    pub ambient_light: Color,
    pub lights: Vec<Pointlight>,
    pub objects: Vec<Object>,
}

impl Scene {
    pub fn read_from_file(file: &str) -> io::Result<Self> {
        let path = Path::new(file);
        let file = File::open(path)?;
        let reader = io::BufReader::new(file);
        let lines = reader.lines();

        let mut cam = None;
        let mut recursion_depth = None;
        let mut background_color = None;
        let mut ambient_light = None;
        let mut lights = Vec::new();
        let mut objects = Vec::new();

        for line in lines {
            match line {
                Ok(line) => {
                    let mut tokens = line.split_whitespace();
                    match tokens.next() {
                        Some("camera") => {
                            cam = Some(Scene::parse_cam(&mut tokens));
                            println!("found Cam");
                        }
                        Some("background") => {
                            background_color = Some(Scene::parse_vec3(&mut tokens));
                            println!("found background color {:?}", background_color);
                        }
                        Some("ambience") => {
                            ambient_light = Some(Scene::parse_vec3(&mut tokens));
                            println!("found ambient light {:?}", ambient_light);
                        }
                        Some("light") => {
                            lights.push(Scene::parse_light(&mut tokens));
                            println!("found Light");
                        }
                        Some("sphere") => {
                            objects.push(Object::Sphere(Scene::parse_sphere(&mut tokens)));
                            println!("found Sphere");
                        }
                        Some("plane") => {
                            objects.push(Object::Plane(Scene::parse_plane(&mut tokens)));
                            println!("found Plane");
                        }
                        Some("depth") => {
                            recursion_depth = Some(tokens.next().unwrap().parse().unwrap());
                            println!("found Depth");
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        return Ok(Scene {
            cam: cam.unwrap(),
            recursion_depth: recursion_depth.unwrap(),
            background_color: background_color.unwrap(),
            ambient_light: ambient_light.unwrap(),
            lights: lights,
            objects: objects,
        });
    }

    fn parse_cam(tokens: &mut SplitWhitespace) -> PerspectiveCam {
        let eye = Scene::parse_vec3(tokens);
        let center = Scene::parse_vec3(tokens);
        let up = Scene::parse_vec3(tokens);
        let fovy = tokens.next().unwrap().parse().unwrap();
        let width = tokens.next().unwrap().parse().unwrap();
        let height = tokens.next().unwrap().parse().unwrap();
        return PerspectiveCam::new(eye, center, up, fovy, width, height);
    }

    fn parse_vec3(tokens: &mut SplitWhitespace) -> Color {
        return Color::new(
            tokens.next().unwrap().parse().unwrap(),
            tokens.next().unwrap().parse().unwrap(),
            tokens.next().unwrap().parse().unwrap(),
        );
    }

    fn parse_light(tokens: &mut SplitWhitespace) -> Pointlight {
        let position = Scene::parse_vec3(tokens);
        let color = Scene::parse_vec3(tokens);
        return Pointlight::new(position, color);
    }

    fn parse_sphere(tokens: &mut SplitWhitespace) -> Sphere {
        let center = Scene::parse_vec3(tokens);
        let radius = tokens.next().unwrap().parse().unwrap();
        let material = Scene::parse_material(tokens);
        return Sphere::new(center, radius, Arc::new(material));
    }

    fn parse_material(tokens: &mut SplitWhitespace) -> Material {
        return Material::new(
            Scene::parse_vec3(tokens),
            Scene::parse_vec3(tokens),
            Scene::parse_vec3(tokens),
            tokens.next().unwrap().parse().unwrap(),
            tokens.next().unwrap().parse().unwrap(),
            false,
            None,
        );
    }

    fn parse_plane(tokens: &mut SplitWhitespace) -> Plane {
        let center = Scene::parse_vec3(tokens);
        let normal = Scene::parse_vec3(tokens);
        let material = Scene::parse_material(tokens);
        return Plane::new(center, normal, Arc::new(material));
    }
}
