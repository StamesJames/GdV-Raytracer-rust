use std::sync::Arc;

use super::{
    object::{material::Material, shapes::RayIntersectable},
    scene::Scene,
    utils::{
        ray::Ray,
        vector::{Color, Vec3},
    }, image::Image,
};

pub struct Raytracer {
    pub scene: Scene,
    pub max_depth: i64,
}

impl Raytracer {
    pub fn new(scene: Scene, max_depth: i64) -> Self { Self { scene, max_depth } }

    pub fn compute_image(&self) -> Image {
        let mut pixels = std::iter::repeat_with(|| self.scene.background_color.clone())
            .take((self.scene.cam.width * self.scene.cam.height) as usize)
            .collect::<Vec<_>>();
        for x in 0..self.scene.cam.width {
            for y in 0..self.scene.cam.height {
                let ray = self.scene.cam.primary_ray(x as f64, y as f64);
                let color = Vec3::min(&self.trace(&ray, 0), &Vec3([1., 1., 1.]));
                let i = y * self.scene.cam.width + x;
                pixels[i as usize] = color;
            }
        }

        return Image::new(
            self.scene.cam.width,
            self.scene.cam.height,
            pixels,
        );
    }

    fn trace(&self, ray: &Ray, depth: i64) -> Vec3 {
        if depth > self.max_depth {
            return Vec3([0., 0., 0.]);
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
        let ambient = Vec3::comp_mult(&self.scene.ambient_light, &material.ambient);
        let mut diff_plus_spec = Vec3::zero();

        for light in &self.scene.lights {
            let l = (point - &light.position).normalized();
            let r = 2. * normal * (normal * &l) - &l;
            let v = view.normalized();
            diff_plus_spec = &diff_plus_spec + Vec3::comp_mult(&light.color, 
                &(
                    &material.diffuse * (normal * &l) + 
                    &material.specular * f64::powf(r * v, material.shininess)
                )
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
