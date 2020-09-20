use quicksilver::geom::{Vector, Circle};
use quicksilver::graphics::{Color, Graphics};
use rand_distr::{Uniform, Normal, Distribution};
use super::{XorShiftRng, SIZE, Particle, Shape, Player, in_screen};

const POWER_CIRCLES: i32 = 7;

#[derive(Clone, Copy, Debug)]
pub enum Power {
    LifeUp,
    PierceUp,
    ShotUp,
}

#[derive(Clone, Copy, Debug)]
pub struct PowerUp {
    pub pos: Vector,
    pub vel: Vector,
    pub power: Power,
    pub hits: i32,
    pub radius: f32,
    pub t: f32,
}

impl PowerUp {
    pub fn new(power: Power, rng: &mut XorShiftRng) -> Self {
        let unif = Uniform::new(-100.0, SIZE.x + 100.0);
        let mut pos = Vector::ONE;
        while in_screen(&pos) {
            pos.x = unif.sample(rng);
            pos.y = unif.sample(rng);
        }

        let angle = Uniform::new(0.0, 360.0);
        let length = Normal::new(20.0, 8.0).unwrap();


        PowerUp {
            pos: pos,
            vel: Vector::from_angle(angle.sample(rng)) * length.sample(rng) as f32,
            power: power,
            hits: 5,
            radius: 20.0,
            t: 0.0,
        }
    }

    pub fn color(&self) -> Color {
        match self.power {
            Power::LifeUp => Color::from_hex("#26A65B"),
            Power::PierceUp => Color::from_hex("#BF55EC"),
            Power::ShotUp => Color::from_hex("#F22613"),
        }
    }

    pub fn particles(&self, rng: &mut XorShiftRng) -> Vec<Particle> {

        let speed = Normal::new(8.0, 2.0).unwrap();

        (0..4).map(|u| {
            let a = u as f32 * 90.0; // 4 directions

            Particle {
                pos: self.pos + self.vel, // Particles start to move a the next frame
                speed: speed.sample(rng) as f32,
                angle: a,
                bias: self.vel,
                accel: -2.5,
                shape: Shape::Circle(3.0),
                color: self.color(),
                alpha_scale: 8.0,
                ..Particle::default()
            }
        }).collect()
    }

    pub fn draw(&self, gfx: &mut Graphics, prop: f32) {
        for i in 0..POWER_CIRCLES {
            let a = 360.0 * (i as f32) / (POWER_CIRCLES as f32);
            let angle = a + (self.t + prop) * 20.0;
            gfx.fill_circle(
                &Circle::new(
                    self.pos + Vector::from_angle(angle) * self.radius,
                    5.0,
                ), 
                Color::WHITE.with_alpha(0.7),
            );
        }
    }

    pub fn update(&mut self, player: &mut Player) {
        self.t += 1.0;
        self.pos += self.vel;

        if (self.pos.x < 0.0 && self.vel.x < 0.0)
            || (self.pos.x > SIZE.x && self.vel.x > 0.0) {
            self.vel.x *= -1.0;
            self.hits -= 1;
        }

        if (self.pos.y < 0.0 && self.vel.y < 0.0)
            || (self.pos.y > SIZE.y && self.vel.y > 0.0) {
            self.vel.y *= -1.0;
            self.hits -= 1;
        }

        if self.pos.distance(player.pos) < self.radius + player.radius {
            self.hits = 0;
            player.powerup(self.power);
        }

    }
}