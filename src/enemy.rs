use quicksilver::{
    geom::{Vector},
    graphics::{Color},
};
use rand::{distributions::{Uniform, Normal, Distribution}};
use rand_xorshift::XorShiftRng;

use super::{Particle, Shape, hsv2rgb, Shot, Player, Game};


#[derive(Copy, Clone, Debug)]
pub struct Enemy {
    pub pos: Vector,
    pub speed: f32,
    pub angle: f32,
    pub alive: bool,
    pub radius: f32,
    pub life: u32,
}

impl Enemy {
    pub fn new(pos: Vector, life: u32) -> Self {
        Enemy {
            pos: pos,
            speed: 6.0,
            angle: 0.0,
            alive: true,
            life: life,
            radius: life as f32 * 5.0 + 30.0 ,
        }
    }

    pub fn update(&mut self, game: &mut Game) {

        // Move and update speed + angle
        let player_dir = game.player.pos - self.pos;
        let player_angle = player_dir.angle();

        let angular_diff = ((player_angle - self.angle) % 360.0 + 540.0) % 360.0 - 180.0;
        self.angle = (self.angle + 0.1 * angular_diff) % 360.0;

        let vel = Vector::from_angle(self.angle) * self.speed;
        self.pos += vel;

        for s in &mut game.shots {
            if (s.pos - self.pos).len2() < (s.radius + self.radius).powi(2) {
                s.alive = false;
                self.alive = false;
            }
        }

        // Generate particles
        let angle = Uniform::new(0.0, 360.0);

        let ps : Vec<Particle> = (0..2).map(|_| Particle {
            pos: self.pos,
            speed: 7.0,
            angle: angle.sample(&mut game.rng),
            damp: 1.0,
            accel: -0.8,
            angular_vel: 0.0,
            shape: Shape::Circle(3.0),
            color: Color::PURPLE,
        }).collect();

        game.particles.extend(ps);
    }
}