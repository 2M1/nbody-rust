use crate::{
    body::Body,
    physics::GRAVITY,
    vector::{Vector, Vector2D},
};

#[derive(Debug, Clone, Copy)]
pub enum WeightUnit {
    Kilograms,
    Tonnes, // metric
    SolarMasses,
}

impl Default for WeightUnit {
    fn default() -> Self {
        WeightUnit::Kilograms
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

        let len = self.bodies.len();
        for i in 0..len {
            for j in 0..len {
                if i == j {
                    continue;
                }

                let body = &self.bodies[i];
                let other = &self.bodies[j];

                let distance = other.position - body.position;
                let distance_squared = body.position.distance_squared(&other.position, 0.00001);
                let acc = distance * other.mass * GRAVITY / distance_squared;

                let body = &mut self.bodies[i];
                body.acceleration += acc;
            }
        }
    }
}
