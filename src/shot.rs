use quicksilver::geom::Vector;
use rand_distr::*;
use super::{Particle, Color, XorShiftRng, Shape, in_screen};


#[derive(Copy, Clone, Debug)]
pub struct Shot {
    pub pos: Vector,
    pub vel: Vector,
    pub radius: f32,
    pub alive: bool,
    pub pierce: i32,
}

impl Shot {
    pub fn new(pos: Vector, vel: Vector, pierce: i32) -> Self {
        Shot {
            pos,
            vel,
            radius: 15.0,
            alive: true,
            pierce: pierce
        }
    }

    pub fn particles(&self, rng: &mut XorShiftRng) -> Vec<Particle> {
        // let angle = Normal::new(self.vel.angle() as f64 + 180.0, 10.0);
        let speed = Normal::new(15.0, 1.0).unwrap();
        
         (0..1).map(|_| Particle {
            pos: self.pos,
            speed: speed.sample(rng) as f32,
            angle: 180.0 + self.vel.angle(),
            accel: -5.0,
            // angular_vel: angular_vel.sample(rng) as f32,
            shape: Shape::Shard(-0.7 - 0.3 * self.pierce as f32, 3.0, false),
            color: Color::RED,
            ..Particle::default()
        }).collect()
        
    }
    pub fn update(&mut self) {
        self.pos += self.vel;

        self.alive = self.alive && in_screen(&self.pos) && self.pierce > 0;
   }
}