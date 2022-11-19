use std::{f64, ops};

#[derive(Clone)]
pub struct Vector<T, const N: usize>(pub [T; N]);
#[derive(Clone, Debug)]
pub struct Vec3(pub [f64; 3]);

pub type Color = Vec3;

impl<const N: usize> Vector<f64, N> {
    pub fn zero() -> Self {
        Self([0.; N])
    }

    pub fn new_uniform(s: f64) -> Self {
        Self([s; N])
    }

    pub fn new(data: [f64; N]) -> Self {
        Self(data)
    }
}
impl Vec3 {
    pub fn comp_mult(lhs: &Self, rhs: &Self) -> Self {
        return Vec3([lhs[0] * rhs[0], lhs[1] * rhs[1], lhs[2] * rhs[2]]);
    }

    pub fn norm(&self) -> f64 {
        return f64::sqrt(self * self);
    }
    pub fn zero() -> Self {
        Self([0.; 3])
    }

    pub fn new_uniform(s: f64) -> Self {
        Self([s; 3])
    }

    pub fn new(data: [f64; 3]) -> Self {
        Self(data)
    }

    pub fn cross(v0: &Vec3, v1: &Vec3) -> Vec3 {
        Vec3([
            v0[1] * v1[2] - v0[2] * v1[1],
            v0[2] * v1[0] - v0[0] * v1[2],
            v0[0] * v1[1] - v0[1] * v1[0],
        ])
    }

    pub fn normalized(&self) -> Vec3 {
        return self / self.norm();
    }

    pub fn distance(v0: &Vec3, v1: &Vec3) -> f64 {
        return (v0 - v1).norm();
    }

    pub fn min(v0: &Vec3, v1: &Vec3) -> Vec3 {
        return Vec3([
            f64::min(v0[0], v1[0]),
            f64::min(v0[1], v1[1]),
            f64::min(v0[2], v1[2]),
        ]);
    }
}

impl ops::Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

// Negation Operator
impl ops::Neg for &Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3([-self.0[0], -self.0[1], -self.0[2]])
    }
}
impl ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3([-self.0[0], -self.0[1], -self.0[2]])
    }
}

// Addition Operator
impl ops::Add<&Vec3> for &Vec3 {
    type Output = Vec3;

    fn add(self, rhs: &Vec3) -> Self::Output {
        Vec3([
            self.0[0] + rhs.0[0],
            self.0[1] + rhs.0[1],
            self.0[2] + rhs.0[2],
        ])
    }
}
impl ops::Add<&Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: &Vec3) -> Self::Output {
        Vec3([
            self.0[0] + rhs.0[0],
            self.0[1] + rhs.0[1],
            self.0[2] + rhs.0[2],
        ])
    }
}
impl ops::Add<Vec3> for &Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3([
            self.0[0] + rhs.0[0],
            self.0[1] + rhs.0[1],
            self.0[2] + rhs.0[2],
        ])
    }
}
impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3([
            self.0[0] + rhs.0[0],
            self.0[1] + rhs.0[1],
            self.0[2] + rhs.0[2],
        ])
    }
}

// Subtraction Operator
impl ops::Sub<&Vec3> for &Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: &Vec3) -> Self::Output {
        Vec3([
            self.0[0] - rhs.0[0],
            self.0[1] - rhs.0[1],
            self.0[2] - rhs.0[2],
        ])
    }
}
impl ops::Sub<&Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: &Vec3) -> Self::Output {
        Vec3([
            self.0[0] - rhs.0[0],
            self.0[1] - rhs.0[1],
            self.0[2] - rhs.0[2],
        ])
    }
}
impl ops::Sub<Vec3> for &Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3([
            self.0[0] - rhs.0[0],
            self.0[1] - rhs.0[1],
            self.0[2] - rhs.0[2],
        ])
    }
}
impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3([
            self.0[0] - rhs.0[0],
            self.0[1] - rhs.0[1],
            self.0[2] - rhs.0[2],
        ])
    }
}

// Dot Multiplication Operator
impl ops::Mul<&Vec3> for &Vec3 {
    type Output = f64;

    fn mul(self, rhs: &Vec3) -> Self::Output {
        return self.0[0] * rhs.0[0] + self.0[1] * rhs.0[1] + self.0[2] * rhs.0[2];
    }
}
impl ops::Mul<Vec3> for &Vec3 {
    type Output = f64;

    fn mul(self, rhs: Vec3) -> Self::Output {
        return self.0[0] * rhs.0[0] + self.0[1] * rhs.0[1] + self.0[2] * rhs.0[2];
    }
}
impl ops::Mul<Vec3> for Vec3 {
    type Output = f64;

    fn mul(self, rhs: Vec3) -> Self::Output {
        return self.0[0] * rhs.0[0] + self.0[1] * rhs.0[1] + self.0[2] * rhs.0[2];
    }
}
impl ops::Mul<&Vec3> for Vec3 {
    type Output = f64;

    fn mul(self, rhs: &Vec3) -> Self::Output {
        return self.0[0] * rhs.0[0] + self.0[1] * rhs.0[1] + self.0[2] * rhs.0[2];
    }
}

// Skalar Multiplication Operator
impl ops::Mul<&Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: &Vec3) -> Self::Output {
        Vec3([rhs.0[0] * self, rhs.0[1] * self, rhs.0[2] * self])
    }
}
impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3([rhs.0[0] * self, rhs.0[1] * self, rhs.0[2] * self])
    }
}

impl ops::Mul<f64> for &Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3([self.0[0] * rhs, self.0[1] * rhs, self.0[2] * rhs])
    }
}
impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3([self.0[0] * rhs, self.0[1] * rhs, self.0[2] * rhs])
    }
}

// Skalare Division Operator
impl ops::Div<f64> for &Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        Vec3([self.0[0] / rhs, self.0[1] / rhs, self.0[2] / rhs])
    }
}
impl ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        Vec3([self.0[0] / rhs, self.0[1] / rhs, self.0[2] / rhs])
    }
}
