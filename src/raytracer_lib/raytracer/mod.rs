use rayon::prelude::*;
use std::sync::{Arc, Mutex};
// use nalgebra::Vector3;

use super::{
    image::Image,
    object::{material::Material, shapes::RayIntersectable},
    scene::Scene,
    utils::{
        ray::Ray,
        vector::{Color, Vec3},
    },
};

pub struct Raytracer {
    pub scene: Scene,
    pub max_depth: i64,
    pub image: Image,
}

impl Raytracer {
    pub fn new(scene: Scene, max_depth: i64) -> Self {
        let image = Image::new(
            scene.cam.width,
            scene.cam.height,
            scene.background_color.clone(),
        );
        Self {
            scene,
            max_depth,
            image,
        }
    }

    pub fn compute_image(&mut self) {
        let width = self.scene.cam.width;
        let height = self.scene.cam.height;
        let cam = &self.scene.cam;
        let scene = &self.scene;
        // let self_mutex = Arc::new(Mutex::new(&mut self.image));

        let pixels : Vec<Vec3> = 
        (0..(width*height)).into_par_iter().map(|i| {
                let x = i%width;
                let y = i/width;

                let ray = cam.primary_ray(x, y);
                self.trace(&ray, self.max_depth)
            
        }).collect();
        self.image.set_pixels(pixels);

        // (0..width).into_par_iter().for_each(|x| {
        //     (0..height).into_par_iter().for_each(|y| {
        //         let ray = cam.primary_ray(x, y);
        //         let color = scene.trace(&ray, 0, self.max_depth);

        //         self_mutex.lock().unwrap().set_pixel(x, y, color);
        //     })
        // })
    }

    pub fn trace(&self, ray: &Ray, depth: i64) -> Vec3 {
        if depth > self.max_depth {
            return Vec3::zeros();
        }
        if let Some(intersection_data) = self.intersect_scene(ray) {
            return self.lighting(
                &intersection_data.point,
                &intersection_data.normal,
                &-&ray.direction,
                &intersection_data.material,
            );
        } else {
            return self.scene.background_color.clone();
        }
    }

    pub fn intersect_scene(&self, ray: &Ray) -> Option<IntersectionData> {
        let mut min_dist = f64::MAX;
        let mut curent_intersection_data = None;
        for object in &self.scene.objects {
            if let Some(intersection_data) = object.intersect(ray) {
                if intersection_data.distance < min_dist {
                    min_dist = intersection_data.distance;
                    curent_intersection_data = Some(intersection_data);
                }
            }
        }

        return curent_intersection_data;
    }

    pub fn lighting(&self, point: &Vec3, normal: &Vec3, view: &Vec3, material: &Material) -> Color {
        let ambient = Vec3::component_mul(&self.scene.ambient_light, &material.ambient);
        let mut diff_plus_spec = Vec3::zeros();
        for light in &self.scene.lights {
            let shadow_intersect_option =
                self.intersect_scene(&Ray::new(point.clone(), point.to(&light.position)));
            if let Some(shadow_intersection) = shadow_intersect_option {
                if shadow_intersection.distance < Vec3::metric_distance(point, &light.position) {
                    break;
                }
            }
            let l = point.to(&light.position).normalize();
            let r = (2. * normal * (normal.dot(&l))) - &l;
            let v = view.normalize();
            diff_plus_spec = &diff_plus_spec
                + Vec3::component_mul(
                    &light.color,
                    &(&material.diffuse * (normal.dot(&l))
                        + &material.specular * f64::powf(r.dot(&v), material.shininess)),
                )
        }

        return ambient + diff_plus_spec;
    }
}

pub struct IntersectionData {
    pub point: Vec3,
    pub normal: Vec3,
    pub distance: f64,
    pub material: Arc<Material>,
}
