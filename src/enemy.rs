use quicksilver::{
    geom::{Vector},
    graphics::{Color},
};
use rand::{distributions::{Uniform, Normal, Distribution}};
use rand_xorshift::XorShiftRng;

use super::{Particle, Shape, hsv2rgb, Shot, Player, Game};

const KNOCK_BACK: f32 = 40.0;
const KNOCK_DAMP: f32 = 0.7;


#[derive(Copy, Clone, Debug)]
pub struct Enemy {
    pub pos: Vector,
    pub speed: f32,
    pub angle: f32,
    pub alive: bool,
    pub radius: f32,
    pub life: u32,
    pub knockback: Vector,
}

impl Enemy {
    pub fn new(pos: Vector, life: u32) -> Self {
        Enemy {
            pos: pos,
            speed: 0.0,
            angle: 0.0,
            alive: true,
            life: life,
            radius: life as f32 * 5.0 + 30.0 ,
            knockback: Vector::ZERO,
        }
    }

    pub fn new_kb(pos: Vector, life: u32, knockback: Vector) -> Self {
        Enemy {
            pos: pos,
            speed: 0.0,
            angle: knockback.angle(),
            alive: true,
            life: life,
            radius: life as f32 * 5.0 + 30.0 ,
            knockback: knockback,
        }
    }
    

    pub fn update(&mut self, game: &mut Game) -> Vec<Self> {

        // Move and update speed + angle
        let player_dir = game.player.pos - self.pos;
        let player_angle = player_dir.angle();

        let angular_diff = ((player_angle - self.angle) % 360.0 + 540.0) % 360.0 - 180.0;
        self.angle = (self.angle + 0.05 * angular_diff) % 360.0;

        self.speed = (self.speed + 0.4).min(6.0 - (self.life as f32) * 0.5);
        let vel = Vector::from_angle(self.angle) * self.speed;
        self.pos += vel;

        self.knockback *= KNOCK_DAMP;
        self.pos += self.knockback;

        // Check collisions
        let mut kill_dir = None;
        for s in &mut game.shots {
            if s.alive && (s.pos - self.pos).len2() < (s.radius + self.radius).powi(2) {
                s.alive = false;
                self.alive = false;
                kill_dir = Some(s.vel);
            }
        }

        // Generate particles
        game.particles.extend(self.particles(&mut game.rng));


        if !self.alive && self.life > 1 {
            let d = kill_dir.unwrap().angle();
            let dir1 = Vector::from_angle(d + 30.0) * KNOCK_BACK;
            let dir2 = Vector::from_angle(d - 30.0) * KNOCK_BACK;
            vec![
                Enemy::new_kb(self.pos, self.life - 1, dir1),
                Enemy::new_kb(self.pos, self.life - 1, dir2),
            ]
        } else {
            vec![]
        }
    }

    fn particles(&self, rng: &mut XorShiftRng) -> Vec<Particle> {


        let colors = vec![
            Color::PURPLE,
            Color::INDIGO,
            Color::MAGENTA,
            Color::BLUE,
            Color::GREEN,
            Color::ORANGE,
            Color::RED,
        ];

        let angle = Uniform::new(0.0, 360.0);

        let l = self.life as f32;

        (0..1).map(|_| Particle {
            pos: self.pos,
            speed: 5.0 + l * 2.0,
            angle: angle.sample(rng),
            damp: 1.0,
            accel: -0.8 - l*0.2,
            angular_vel: l * 0.1,
            shape: Shape::Circle(3.0),
            color: colors[self.life as usize % colors.len()],
        }).collect()
    }
}