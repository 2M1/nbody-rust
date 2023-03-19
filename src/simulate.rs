use std::{fmt::Display, str::FromStr};

use crate::{
    body::Body,
    physics::GRAVITY,
    vector::{Vector, Vector2D},
};

#[derive(Debug, Clone, Copy, Default)]
pub enum WeightUnit {
    #[default]
    Kilograms,
    Tonnes, // metric
    SolarMasses,
}

impl Display for WeightUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Self::Kilograms => "Kilograms",
            Self::SolarMasses => "SolarMasses",
            Self::Tonnes => "Tonnes",
        };

        f.write_str(name)?;

        return Ok(());
    }
}

impl FromStr for WeightUnit {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "kilograms" => Ok(Self::Kilograms),
            "solar masses" => Ok(Self::SolarMasses),
            "tonnes" => Ok(Self::Tonnes),
            _ => Err(format!("Unknown weight unit: {}", s)),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct UnitParameters {
    pub weight: WeightUnit,
    pub meters_per_pixel: f64,
    pub gravity: f64,
}

impl Default for UnitParameters {
    fn default() -> Self {
        UnitParameters {
            weight: WeightUnit::Kilograms,
            meters_per_pixel: 1.0,
            gravity: GRAVITY,
        }
    }
}

pub struct SimulationState {
    pub bodies: Vec<Body<Vector2D>>,
    pub dt: f64, // time step
    pub unit_parameters: UnitParameters,
    pub remaining_passes: Option<i32>, // the remaining passes (ticks) or None for infinite simulation
}

impl SimulationState {
    pub fn new(bodies: Vec<Body<Vector2D>>, dt: f64) -> Self {
        SimulationState {
            bodies,
            dt,
            unit_parameters: UnitParameters::default(),
            remaining_passes: None,
        }
    }

    pub fn new_with_units(
        bodies: Vec<Body<Vector2D>>,
        dt: f64,
        unit_parameters: UnitParameters,
    ) -> Self {
        SimulationState {
            bodies,
            dt,
            unit_parameters,
            remaining_passes: None,
        }
    }

    pub fn simulate_tick(&mut self) {
        for body in self.bodies.iter_mut() {
            body.velocity += body.acceleration * self.dt;
            body.position += body.velocity * self.dt;
        }

        const SOFTENING: f64 = 0.01;
        let len = self.bodies.len();
        for i in 0..len {
            for j in 0..len {
                if i == j {
                    continue;
                }

                let body = &self.bodies[i];
                let other = &self.bodies[j];

                let distance = other.position - body.position; // distance in Meters
                let distance_squared = body.position.distance_squared(&other.position, SOFTENING); // distance squared in pixels^2
                let acc = distance * other.mass * GRAVITY / distance_squared;

                let body = &mut self.bodies[i];
                body.acceleration += acc;
            }
        }
    }
}
