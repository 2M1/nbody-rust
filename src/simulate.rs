use crate::{
    body::Body,
    physics::GRAVITY,
    vector::{Vector, Vector2D},
};

pub fn simulate_tick(bodies: &mut Vec<Body<Vector2D>>, dt: f64) {
    for body in bodies.iter_mut() {
        body.velocity += body.acceleration * dt;
        body.position += body.velocity * dt;
    }

    let len = bodies.len();
    for i in 0..len {
        for j in 0..len {
            if i == j {
                continue;
            }

            let body = &bodies[i];
            let other = &bodies[j];

            let distance = other.position - body.position;
            let distance_squared = body.position.distance_squared(&other.position, 0.00001);
            let acc = distance * other.mass * GRAVITY / distance_squared;

            let body = &mut bodies[i];
            body.acceleration += acc;
        }
    }
}
