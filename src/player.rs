use quicksilver::{
    geom::{Vector},
    graphics::{Color},
    Input,
};
use rand::{distributions::{Uniform, Normal, Distribution}};
use rand_xorshift::XorShiftRng;

use super::{Particle, Shape, hsv2rgb, Shot, Enemy, Game};

#[derive(Debug)]
pub struct Player {
    pub pos: Vector,
    pub life: usize,
    pub radius: f32,
    pub invincible: i32,
}


impl Player {
    pub fn new() -> Self {
        Player {
            pos: Vector::new(200.0, 200.0),
            life: 3,
            radius: 30.0,
            invincible: 0,
        }
    }
    pub fn update(mouse: Vector, game: &mut Game) {

        // Move towards the cursor
        let dir = mouse - game.player.pos;
        let dist = dir.len();
        if dist > 1.0 {
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
    }

    pub fn particles(&self, rng: & mut  XorShiftRng) -> Vec<Particle> {

        // Generate its particles
        let angle = Uniform::new(0.0, 360.0);
        let speed = Normal::new(10.0, 3.0);
        let hue = Normal::new(27.0, 3.0);

        (0..4).map(|_| Particle {
            pos: self.pos,
            speed: speed.sample(rng) as f32,
            angle: angle.sample(rng),
            accel: -0.1,
            damp: 0.88,
            angular_vel: 25.0,
            shape: Shape::Circle(4.0),
            color: hsv2rgb(hue.sample(rng) as f32, 1.0, 1.0)
        }).collect()
    }

    pub fn fire(&self, aim: Vector) -> Shot {
        Shot::new(self.pos, (aim - self.pos).with_len(45.0))
    }
}
