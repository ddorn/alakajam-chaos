use quicksilver::{
    geom::{Vector, Circle},
    graphics::{Color, Graphics},
};
use rand_distr::*;
use rand_xorshift::XorShiftRng;

use super::{Particle, Shape, Game};

const KNOCK_BACK: f32 = 50.0;
const KNOCK_DAMP: f32 = 0.8;
const INVINCIBLE_FRAMES: f32 = 4.0;
const DIE_SHARDS: usize = 3;


#[derive(Copy, Clone, Debug)]
pub struct Enemy {
    pub pos: Vector,
    pub speed: f32,
    pub angle: f32,
    pub radius: f32,
    pub level: u32,
    life: i32,
    pub knockback: Vector,
    color: Color,
}

impl Enemy {
    pub fn new(pos: Vector, level: u32) -> Self {
        Enemy::new_kb(pos, level, Vector::ZERO)
    }

    pub fn new_kb(pos: Vector, level: u32, knockback: Vector) -> Self {
        let colors = vec![
            Color::PURPLE,
            Color::INDIGO,
            Color::MAGENTA,
            Color::BLUE,
            Color::GREEN,
            Color::ORANGE,
            Color::RED,
        ];
        let color = colors[level as usize % colors.len()];

        Enemy {
            pos: pos,
            speed: 0.0,
            angle: 0.0,
            level: level,
            life: (level * level) as i32,
            radius: level as f32 * 5.0 + 30.0 ,
            knockback: knockback,
            color: color,
        }
    }
    
    pub fn alive(&self) -> bool {
        self.life > 0
    }
    pub fn invincible(&self) -> bool {
        self.knockback.len2() >= KNOCK_BACK * KNOCK_DAMP.powf(INVINCIBLE_FRAMES) * 0.99  // rounding
    }

    pub fn update(&mut self, game: &mut Game) -> Vec<Self> {

        // Move and update speed + angle
        let player_dir = game.player.pos - self.pos;
        let player_angle = player_dir.angle();

        let angular_diff = ((player_angle - self.angle) % 360.0 + 540.0) % 360.0 - 180.0;
        self.angle = (self.angle + 0.09 * angular_diff) % 360.0;

        self.speed = (self.speed + 0.4).min(4.0 + (self.level as f32));
         
        let vel = Vector::from_angle(self.angle) * self.speed;
        self.pos += vel;

        self.knockback *= KNOCK_DAMP;
        self.pos += self.knockback;

        // Check collisions
        let mut hit_angle = None;
        if !self.invincible() {
            for s in &mut game.shots {
                if s.pierce > 0 && (s.pos - self.pos).len2() < (s.radius + self.radius).powi(2) {
                    s.pierce -= 1;
                    let dmg = s.damage.min(self.life);
                    self.life -= dmg;

                    let a = s.vel.angle();
                    hit_angle = Some(a);
                    
                    self.knockback = Vector::from_angle(a) * KNOCK_BACK;

                    game.shake += 1;
                    game.bg.chaos(&mut game.rng);


                    let angle = Normal::new(a as f64, 40.0).unwrap();
                    let speed = Normal::new(60.0, 12.0).unwrap();
                    for _ in 0..=dmg {
                        // let angle = 360.0 * (i as f32) / (DIE_SHARDS as f32);
                        game.particles.push(Particle {
                            pos: self.pos,
                            speed: speed.sample(&mut game.rng) as f32,
                            damp: 0.8,
                            angle: angle.sample(&mut game.rng) as f32,
                            shape: Shape::Shard(0.2, 3.0, true),
                            color: Color::WHITE.with_alpha(0.8),
                            ..Particle::default()
                        })
                    }
                }
            }
        }

        if !self.alive() && self.level > 1 {
            let d = hit_angle.unwrap();
            let dir1 = Vector::from_angle(d + 30.0) * KNOCK_BACK;
            let dir2 = Vector::from_angle(d - 30.0) * KNOCK_BACK;
            vec![
                Enemy::new_kb(self.pos, self.level - 1, dir1),
                Enemy::new_kb(self.pos, self.level - 1, dir2),
            ]
        } else {
            vec![]
        }
    }

    pub fn draw(&self, gfx: &mut Graphics, prop: f32) {
        gfx.fill_circle(
            &Circle::new(self.pos, self.radius),
            self.color.with_alpha(0.1)
        );
    }

    pub fn particles(&self, rng: &mut XorShiftRng, density: f32) -> Vec<Particle> {

        let angle = Uniform::new(0.0, 360.0);

        let l = (self.life as f32).sqrt() + self.level as f32;

        let mut qte = density.floor() as i32;
        if Bernoulli::new((density - density.floor()) as f64).unwrap().sample(rng) {
            qte += 1;
        }

        let speed = 5.0 + l;
        let acc = -speed.powi(2) / (speed + 2.0 * self.radius);
        let bias = Vector::from_angle(self.angle) * self.speed;

        let v = (0..qte).map(|_| Particle {
            pos: self.pos,
            speed: speed,
            angle: angle.sample(rng),
            accel: acc,
            bias: bias,
            // angular_vel: l * 0.1,
            shape: Shape::Circle(3.0),
            color: self.color,
            ..Particle::default()
        }).collect();

        v
    }
}