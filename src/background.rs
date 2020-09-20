use quicksilver::geom::Vector;
#[allow(unused_imports)]
use quicksilver::graphics::{Color, Graphics, Vertex, Mesh, Element};
use rand_distr::*;
use super::{XorShiftRng, SIZE};


const BG_WIGGLE: f32 = 15.0;

pub struct BgPoint {
    pub pos: Vector,
    pub angle: f32,
}

impl BgPoint {
    fn teleport(&mut self, rng: &mut XorShiftRng) {
        let unif = Uniform::new(10.0, 20.0);
        let angle = Uniform::new(0.0, 360.0);

        self.pos += Vector::from_angle(angle.sample(rng)) * unif.sample(rng);
        self.angle += unif.sample(rng);
    }

    fn update(&mut self, score: f32) {
        self.angle += (score + 2.0).log2();
    }

    fn draw_pos(&self) -> Vector {
        self.pos + Vector::from_angle(self.angle) * BG_WIGGLE
    }
}

pub struct Background {
    pub t: f32,
    pub points: Vec<BgPoint>,
    pub color: Color,
}


impl Background {
    pub fn new(rng: &mut XorShiftRng) -> Self {
        let angle = Uniform::new(0.0, 10.0);
        let mut a = 0.0;
        Background {
            t: 0.0,
            points: (0..230).map(|i| {
                a += angle.sample(rng);
                BgPoint {
                    pos: SIZE / 2.0 + Vector::from_angle(i as f32 * 10.0) * i as f32 * 3.0,
                    angle: a,
            }}).collect(),
            color: Color::from_hex("#5e2a53"),
        }
    }

    pub fn update(&mut self, score: u32) {
        for pt in &mut self.points {
            pt.update(score as f32);
        }
    }

    pub fn chaos(&mut self, rng: &mut XorShiftRng) {
        let unif = Uniform::new(0, self.points.len());
        for _ in 0..4 {
            let i = unif.sample(rng);
            self.points[i].teleport(rng);
        }
    }

    pub fn draw(&self, gfx: &mut Graphics, _score: u32) {
        let pts : Vec<Vector> = self.points
            .iter()
            .map(|p| p.draw_pos())
            .collect();

        gfx.stroke_path(&pts, self.color);
    }
}