use cg_raytracer_rust::raytracer_lib::{
    camera::perspective_cam::PerspectiveCam,
    light::pointlight::Pointlight,
    object::{material::Material, shapes::sphere::Sphere},
    scene::Scene,
    utils::vector::Vec3,
};

fn main() {
    let material_1 = Material {
        ambient: Vec3([1., 0., 0.]),
        diffuse: Vec3([1., 0., 0.]),
        specular: Vec3([1., 1., 1.]),
        shininess: 100.,
        mirror: 0.4,
        shadowable: false,
        texture_png: None,
    };
    let material_2 = Material {
        ambient: Vec3([0., 1., 0.]),
        diffuse: Vec3([0., 1., 0.]),
        specular: Vec3([1., 1., 1.]),
        shininess: 200.,
        mirror: 0.2,
        shadowable: false,
        texture_png: None,
    };
    let material_3 = Material {
        ambient: Vec3([0., 0., 1.]),
        diffuse: Vec3([0., 0., 1.]),
        specular: Vec3([1., 1., 1.]),
        shininess: 50.,
        mirror: 0.2,
        shadowable: false,
        texture_png: None,
    };

    let scene = Scene {
        cam: PerspectiveCam::new(
            Vec3([1., 3., 8.]),
            Vec3([1., 1., 0.]),
            Vec3([0., 1., 0.]),
            45.,
            500,
            500,
        ),
        recursion_depth: 5,
        background_color: Vec3([0., 0., 0.]),
        ambient_light: Vec3([0.2, 0.2, 0.2]),
        lights: vec![
            Pointlight {
                position: Vec3([0., 50., 0.]),
                color: Vec3([0.35, 0.35, 0.35]),
            },
            Pointlight {
                position: Vec3([50., 50., 50.]),
                color: Vec3([0.35, 0.35, 0.35]),
            },
            Pointlight {
                position: Vec3([-50., 50., 50.]),
                color: Vec3([0.35, 0.35, 0.35]),
            },
        ],
        objects: vec![
            Sphere {
                center: Vec3([0., 1., 0.]),
                radius: 1.0,
                material: &material_1,
            },
            Sphere {
                center: Vec3([-1., 0.5, 2.0]),
                radius: 0.5,
                material: &material_2,
            },
            Sphere {
                center: Vec3([3., 2., 1.5]),
                radius: 2.0,
                material: &material_3,
            },
        ],
    };
    println!("Hello, world!");
}
