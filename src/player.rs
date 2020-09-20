use quicksilver::{
    geom::{Vector},
};
use rand_distr::*;
use rand_xorshift::XorShiftRng;

use super::{Particle, Shape, hsv2rgb, Shot, Power, Game};

const SHOT_SPEED: f32 = 45.0;
const SHOT_DELAY: u32 = 5;

#[derive(Debug)]
pub struct Player {
    pub pos: Vector,
    pub life: usize,
    pub radius: f32,
    pub invincible: i32,
    pub shots: i32,
    pub pierce: i32,
    pub damage: i32,
    pub shoot_delay: u32,
}


impl Player {
    pub fn new() -> Self {
        Player {
            pos: Vector::new(200.0, 200.0),
            life: 3,
            radius: 30.0,
            invincible: 0,
            shots: 5,
            pierce: 5,
            damage: 5,
            shoot_delay: SHOT_DELAY,
        }
    }

    pub fn update(mouse: Vector, game: &mut Game) {

        // Move towards the cursor
        let dir = mouse - game.player.pos;
        let dist = dir.len();
        if dist > 4.0 {
            game.player.pos += dir * 0.2;
        }

        // Check collisions with enemies
        game.player.invincible -= 1;
        if game.player.invincible < 0 {
            for e in &game.enemies {
                if (e.pos - game.player.pos).len2() < (e.radius + game.player.radius).powi(2) {
                    game.player.life -= 1;
                    game.player.invincible = 30;  // 2/3 of a second
                    game.shake += 12;
                    break; // Only one life per frame
                }
            }
        }

        // game.player.shoot_delay -= 1;
        // if game.player.shoot_delay == 0 {
        //     game.shots.extend(game.player.fire(mouse));
        //     game.player.shoot_delay = SHOT_DELAY;
        // }
    }

    pub fn particles(&self, rng: & mut  XorShiftRng) -> Vec<Particle> {

        // Generate its particles
        let angle = Uniform::new(0.0, 360.0);
        let speed = Normal::new(10.0, 3.0).unwrap();
        let hue = Normal::new(27.0, 3.0).unwrap();

        (0..4).map(|_| Particle {
            pos: self.pos,
            speed: speed.sample(rng) as f32,
            angle: angle.sample(rng),
            accel: -1.5,
            // damp: 0.88,
            angular_vel: 25.0,
            shape: Shape::Circle(4.0),
            color: hsv2rgb(hue.sample(rng) as f32, 1.0, 1.0),
            ..Particle::default()
        }).collect()
    }

    pub fn fire(&self, aim: Vector) -> Vec<Shot> {
        let angle =(aim - self.pos).angle();

        (0..self.shots).map(|i| {
            let a = angle - 15.0 * (i as f32 - self.shots as f32 / 2.0);

            Shot::new(
                self.pos, 
                Vector::from_angle(a) * SHOT_SPEED,
                self.pierce,
                self.damage,
            )
        }).collect()
    }

    pub fn powerup(&mut self, up: Power) -> Vec<Particle> {
        match up {
            Power::LifeUp => self.life += 1,
            Power::PierceUp => self.pierce += 1,
            Power::ShotUp => self.shots += 1,
            Power::DamageUp => self.damage += 1,
        };

        vec![]
    }
}
