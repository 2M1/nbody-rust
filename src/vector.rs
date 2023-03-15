use std::{
    fmt::{Debug, Formatter},
    ops::{Add, AddAssign, Div, Mul, MulAssign, Sub},
};

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct Vector2D {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Vector3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub trait Vector: AddAssign<Self> + Sized + Copy {
    /// Returns the eucledian distance between two vectors without taking the squareroot.
    ///
    /// This is useful for futher uses in calculations where the distance would be squared again.
    fn distance_squared(&self, other: &Self, softening: f64) -> f64;
}

impl Debug for Vector2D {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl AddAssign<Vector2D> for Vector2D {
    fn add_assign(&mut self, rhs: Vector2D) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl AddAssign<f64> for Vector2D {
    fn add_assign(&mut self, rhs: f64) {
        self.x += rhs;
        self.y += rhs;
    }
}

impl Add<Vector2D> for Vector2D {
    type Output = Vector2D;

    fn add(self, rhs: Vector2D) -> Self::Output {
        return Vector2D {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        };
    }
}

impl Sub<Vector2D> for Vector2D {
    type Output = Vector2D;

    fn sub(self, rhs: Vector2D) -> Self::Output {
        return Vector2D {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        };
    }
}

impl Mul<f64> for Vector2D {
    type Output = Vector2D;

    fn mul(self, rhs: f64) -> Self::Output {
        return Vector2D {
            x: self.x * rhs,
            y: self.y * rhs,
        };
    }
}

impl MulAssign<f64> for Vector2D {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl Div<f64> for Vector2D {
    type Output = Vector2D;

    fn div(self, rhs: f64) -> Self::Output {
        return Vector2D {
            x: self.x / rhs,
            y: self.y / rhs,
        };
    }
}

impl Vector for Vector2D {
    fn distance_squared(&self, other: &Self, softening: f64) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let distance_squared = dx * dx + dy * dy + softening * softening;
        return distance_squared;
    }
}
