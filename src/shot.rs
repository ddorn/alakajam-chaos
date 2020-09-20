use quicksilver::geom::Vector;
use rand_distr::*;
use super::{Particle, Color, XorShiftRng, Shape, in_screen, SHOT_SPEED};


#[derive(Copy, Clone, Debug)]
pub struct Shot {
    pub pos: Vector,
    pub vel: Vector,
    pub radius: f32,
    pub alive: bool,
    pub pierce: i32,
    pub damage: i32,
    pub laser: bool,
}

impl Shot {
    pub fn new(pos: Vector, vel: Vector, pierce: i32, damage: i32) -> Self {
        Shot {
            pos,
            vel,
            radius: 15.0,
            alive: true,
            pierce: pierce,
            damage: damage,
            laser: false,
        }
    }

    pub fn laser(pos: Vector, angle: f32, damage: i32) -> Self {
        Shot {
            pos,
            vel: Vector::from_angle(angle) * SHOT_SPEED * 2.0,
            radius: 25.0,
            alive: true,
            pierce: 1000,
            damage: damage,
            laser: true,
        }
    }

    pub fn particles(&self, rng: &mut XorShiftRng) -> Vec<Particle> {
        // let angle = Normal::new(self.vel.angle() as f64 + 180.0, 10.0);
        let speed = Normal::new(15.0, 1.0).unwrap();

        let shape = if self.laser {
            Shape::Shard(-1.5, 5.0, true)
        } else {
            Shape::Shard(-0.7 - 0.3 * self.pierce as f32, 3.0, false)
        };
        
         (0..1).map(|_| Particle {
            pos: self.pos,
            speed: speed.sample(rng) as f32,
            angle: 180.0 + self.vel.angle(),
            accel: -5.0,
            // angular_vel: angular_vel.sample(rng) as f32,
            shape: shape,
            color: Color::WHITE,
            ..Particle::default()
        }).collect()
        
    }
    pub fn update(&mut self) {
        self.pos += self.vel;

        self.alive = self.alive && in_screen(&self.pos) && self.pierce > 0;
   }
}