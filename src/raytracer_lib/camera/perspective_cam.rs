use std::f64::consts::PI;

use crate::raytracer_lib::utils::{ray::Ray, vector::Vec3};

pub struct PerspectiveCam {
    pub eye: Vec3,
    pub center: Vec3,
    pub up: Vec3,
    pub fovy: f64,
    pub width: u32,
    pub height: u32,
    x_dir: Vec3,
    y_dir: Vec3,
    lower_left: Vec3,
}

impl PerspectiveCam {
    pub fn new(eye: Vec3, center: Vec3, up: Vec3, fovy: f64, width: u32, height: u32) -> Self {
        let view = (&center - &eye).normalized();
        let dist = Vec3::distance(&center, &eye);
        // compute width & height of the image plane
        // based on the opening angle of the camera (fovy) and the distance
        // of the eye to the near plane (dist)
        let w = width as f64;
        let h = height as f64;
        let image_height = 2.0 * dist * f64::tan(0.5 * fovy / 180.0 * PI);
        let image_width = w / h * image_height;

        // compute right and up vectors on the image plane
        let x_dir = &Vec3::cross(&view, &up).normalized() * (image_width / w);
        let y_dir = &Vec3::cross(&x_dir, &view).normalized() * (image_height / h);

        // compute lower left corner on the image plane
        let lower_left = &center - &((0.5 * w) * &x_dir) - 0.5 * h * &y_dir;
        Self {
            eye,
            center,
            up,
            fovy,
            width,
            height,
            x_dir,
            y_dir,
            lower_left,
        }
    }

    pub fn primary_ray(&self, x: f64, y: f64) -> Ray {
        Ray::new(self.eye.clone(), &self.lower_left + (x * &self.x_dir) + (y * &self.y_dir) - &self.eye)
    }
}
