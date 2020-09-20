use quicksilver::geom::Vector;
use std::f32::consts::PI;
use rand::distributions::{Distribution, Normal, Uniform};
use super::{Particle, Color, XorShiftRng, Shape, in_screen};


#[derive(Copy, Clone, Debug)]
pub struct Shot {
    pub pos: Vector,
    pub vel: Vector,
    pub radius: f32,
    pub alive: bool,
}

impl Shot {
    pub fn new(pos: Vector, vel: Vector) -> Self {
        Shot {
            pos,
            vel,
            radius: 15.0,
            alive: true,
        }
    }

    pub fn particles(&self, rng: &mut XorShiftRng) -> Vec<Particle> {
        // let angle = Normal::new(self.vel.angle() as f64 + 180.0, 10.0);
        let speed = Normal::new(15.0, 1.0);
        
         (0..1).map(|_| Particle {
            pos: self.pos,
            speed: speed.sample(rng) as f32,
            angle: self.vel.angle(),
            accel: -4.0,
            damp: 1.0,
            angular_vel: 0.0,
            // angular_vel: angular_vel.sample(rng) as f32,
            shape: Shape::Shard(1.0, 3.0, false),
            color: Color::RED,
        }).collect()
        
    }
    pub fn update(&mut self) {
        self.pos += self.vel;

        self.alive = self.alive && in_screen(&self.pos);
   }
}