#[derive(Debug)]
pub struct Vector2D {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug)]
pub struct Vector3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub trait Vector {
    fn magnitude(&self) -> f64;
    fn add(&self, other: &Self) -> Self;
}

impl Vector for Vector2D {
    fn magnitude(&self) -> f64 {
        return (self.x.powi(2) + self.y.powi(2)).sqrt();
    }

    fn add(&self, other: &Self) -> Self {
        return Vector2D {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}
