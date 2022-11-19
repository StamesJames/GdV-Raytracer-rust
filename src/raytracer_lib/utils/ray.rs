use super::vector::Vec3;

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self {
            origin,
            direction: direction.normalized(),
        }
    }
}

impl FnOnce<(f64,)> for Ray {
    type Output = Vec3;
    extern "rust-call" fn call_once(self, args: (f64,)) -> Self::Output {
        return &self.origin + &(args.0 * &self.direction);
    }
}

impl FnMut<(f64,)> for Ray {
    extern "rust-call" fn call_mut(&mut self, args: (f64,)) -> Self::Output {
        return &self.origin + &(args.0 * &self.direction);
    }
}

impl Fn<(f64,)> for Ray {
    extern "rust-call" fn call(&self, args: (f64,)) -> Self::Output {
        return &self.origin + &(args.0 * &self.direction);
    }
}
