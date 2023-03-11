use crate::vector::{Vector, Vector2D};
use std::{
    fs::File,
    io::{BufRead, BufReader},
    num::ParseFloatError,
    path::Path,
    str::FromStr,
};

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Body<T: Vector> {
    pub position: T,
    pub velocity: T,
    pub acceleration: T,
    pub mass: f64,
}

impl Body<Vector2D> {
    pub fn new(
        position: Vector2D,
        velocity: Vector2D,
        acceleration: Vector2D,
        mass: f64,
    ) -> Body<Vector2D> {
        Body {
            position: position,
            velocity: velocity,
            acceleration: acceleration,
            mass: mass,
        }
    }

    pub fn from_file(path: &Path) -> Result<Vec<Body<Vector2D>>, String> {
        let mut bodies = Vec::new();

        let file = File::open(path).map_err(|e| e.to_string())?;
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line.map_err(|e| e.to_string())?;
            let body = line.parse::<Body<Vector2D>>().map_err(|e| e.to_string())?;
            bodies.push(body);
        }

        return Ok(bodies);
    }
}

impl FromStr for Body<Vector2D> {
    type Err = ParseFloatError;

    fn from_str(s: &str) -> Result<Self, <f64 as FromStr>::Err> {
        let row: Vec<&str> = s.trim().split(',').collect();

        let position = Vector2D {
            x: row[0].parse::<f64>()?,
            y: row[1].parse::<f64>()?,
        };

        let velocity = Vector2D {
            x: row[2].parse::<f64>()?,
            y: row[3].parse::<f64>()?,
        };

        let acceleration = Vector2D { x: 0.0, y: 0.0 };

        let mass = row[4].parse::<f64>()?;

        return Ok(Body::new(position, velocity, acceleration, mass));
    }
}
