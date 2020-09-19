use quicksilver::geom::Vector;
use std::f32::consts::PI;
use super::Particle;
use super::Color;


#[derive(Copy, Clone, Debug)]
pub struct Shot {
    pub pos: Vector,
    pub vel: Vector,
}

impl Shot {
    pub fn new(pos: Vector, vel: Vector) -> Self {
        Shot {
            pos,
            vel
        }
    }

    pub fn update(&mut self) -> Vec<Particle> {
        self.pos += self.vel;

        // Generate particles

        (0..2).map(|_| Particle {
            pos: self.pos,
            speed: 10.0,
            angle: self.vel.angle() + PI,
            damp: 0.8,
            angular_vel: 0.0,
            color: Color::RED,
        }).collect()
    }

    pub fn alive(&self) -> bool {
        self.pos.x > -100.0
        && self.pos.y > -100.0
        && self.pos.x < 5000.0
        && self.pos.y < 5000.0

    }
}