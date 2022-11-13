use super::{
    image::Image,
    object::material::Material,
    scene::Scene,
    utils::{
        ray::Ray,
        vector::{Color, Vec3},
    },
};

pub struct Raytracer<'a> {
    pub scene: Scene<'a>,
    pub max_depth: i64,
}

impl<'a> Raytracer<'a> {
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

        return Image {
            width: self.scene.cam.width,
            height: self.scene.cam.height,
            pixels: pixels,
        };
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
        todo!()
    }

    pub fn lighting(&self, point: &Vec3, normal: &Vec3, view: &Vec3, material: &Material) -> Color {
        todo!()
    }
}

pub struct IntersectionData<'a> {
    pub point: Vec3,
    pub normal: Vec3,
    pub distance: f64,
    pub material: &'a Material,
}
