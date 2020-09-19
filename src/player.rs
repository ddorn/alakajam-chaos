use quicksilver::{
    geom::{Vector},
    graphics::{Color},
    Input,
};
use rand::{Rng, RngCore, SeedableRng, distributions::{Uniform, Normal, Distribution}};
use rand_xorshift::XorShiftRng;

use super::Particle;
use super::hsv2rgb;


pub struct Player {
    pub pos: Vector,
}


impl Player {
    pub fn new() -> Self {
        Player {
            pos: Vector::new(200.0, 200.0),
        }
    }
    pub fn update(&mut self, input: &Input, rng: &mut XorShiftRng) -> Vec<Particle>
    {

        // Move towards the cursor
        let mouse = input.mouse().location();
        let dir = mouse - self.pos;
        let dist = dir.len().max(1.0);  // Avoid div by 0
        self.pos += dir * ((dist * 0.05) / dist); // max N pixels, otherwise prop to dist

        // Generate its particles
        let angle = Uniform::new(0.0, 360.0);
        let speed = Normal::new(10.0, 3.0);
        let hue = Normal::new(27.0, 3.0);

        let mut ps = vec![];
        for _ in 0..4 {
            ps.push(Particle {
                pos: self.pos,
                speed: speed.sample(rng) as f32,
                angle: angle.sample(rng),
                damp: 0.88,
                angular_vel: 25.0,
                color: hsv2rgb(hue.sample(rng) as f32, 1.0, 1.0)
            });
        }

        ps
    }
}
