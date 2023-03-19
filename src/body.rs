use crate::vector::{Vector, Vector2D};
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
    str::FromStr,
};

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Body<T: Vector> {
    ///  The position in meters from 0^n (n - number of dimensions)
    pub position: T,
    /// the current velocity of the body
    pub velocity: T,
    /// current acceleration of the body in all n directions (component wise)
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
            position,
            velocity,
            acceleration,
            mass,
        }
    }

    pub fn from_file(path: &Path) -> Result<Vec<Body<Vector2D>>, String> {
        let mut bodies = Vec::new();

        let file = File::open(path).map_err(|e| {
            format!(
                "error: {}, for {}!\n",
                e.to_string(),
                path.to_string_lossy()
            )
        })?;
        let reader = BufReader::new(file);

        let mut line_no = 0;
        for line in reader.lines() {
            line_no += 1;
            let line = line.map_err(|e| e.to_string())?;
            let parsed = line.parse::<Body<Vector2D>>();

            if let Ok(body) = parsed {
                bodies.push(body);
                continue;
            }
            let err = parsed.expect_err("wtf! Failed cast to err after Ok also failed!");

            println!(
                "[WARN]: failed to parse line {} due to: {}! Skipping!",
                line_no, err
            );
        }

        return Ok(bodies);
    }
}

impl FromStr for Body<Vector2D> {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let row: Vec<&str> = s.trim().split(',').collect();

        if row.len() != 5 {
            return Err("invalid number of columns!".to_string());
        }

        let position = Vector2D {
            x: row[0].parse::<f64>().map_err(|er| er.to_string())?,
            y: row[1].parse::<f64>().map_err(|er| er.to_string())?,
        };

        let velocity = Vector2D {
            x: row[2].parse::<f64>().map_err(|er| er.to_string())?,
            y: row[3].parse::<f64>().map_err(|er| er.to_string())?,
        };

        let acceleration = Vector2D { x: 0.0, y: 0.0 };

        let mass = row[4].parse::<f64>().map_err(|er| er.to_string())?;

        return Ok(Body::new(position, velocity, acceleration, mass));
    }
}
