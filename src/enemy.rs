use quicksilver::{
    geom::{Vector},
    graphics::{Color},
    Input,
};
use rand::{Rng, RngCore, SeedableRng, distributions::{Uniform, Normal, Distribution}};
use rand_xorshift::XorShiftRng;

use super::{Particle, Shape, hsv2rgb, Shot, Player};


#[derive(Copy, Clone, Debug)]
pub struct Enemy {
    pub pos: Vector,
    pub speed: f32,
    pub angle: f32,
    pub alive: bool,
    pub radius: f32,
}

impl Enemy {
    pub fn new(pos: Vector, radius: f32) -> Self {
        Enemy {
            pos: pos,
            speed: 0.0,
            angle: 0.0,
            alive: true,
            radius: radius,
        }
    }

    pub fn update(&mut self, player: &Player, rng: &mut XorShiftRng) -> Vec<Particle> {

        // Move
        let player_dir = player.pos - self.pos;
        let player_angle = player_dir.angle();



        let angular_diff = ((player_angle - self.angle) % 360.0 + 180.0) % 360.0 - 180.0;
        self.angle = (self.angle + 0.1 * angular_diff) % 360.0;

        let vel = Vector::from_angle(self.angle) * self.speed;
        self.speed = 4.0;
        // self.speed = (self.speed + 0.5).max(10.0);

        self.pos += vel;
        // Update velocity to target the player
        // TODO...

        // Generate particles
        let angle = Uniform::new(0.0, 360.0);

        (0..2).map(|_| Particle {
            pos: self.pos,
            speed: 7.0,
            angle: angle.sample(rng),
            damp: 1.0,
            accel: -0.8,
            angular_vel: 0.0,
            shape: Shape::Circle(3.0),
            color: Color::PURPLE,
        }).collect()
    }
}