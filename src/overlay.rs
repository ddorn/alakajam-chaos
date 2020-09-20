use quicksilver::geom::{Vector, Rectangle };
use quicksilver::graphics::{Color, Graphics, FontRenderer};
use super::{SIZE, Particle, Shape};

pub struct Overlay {
    pub text: String,
    pub color: Color,
    pub visible: bool,
    pub height: f32,
    pub frame: i32,
}

impl Overlay {
    pub fn game_over() -> Self {
        Overlay {
            text: String::from("GAME OVER!"),
            color: Color::RED,
            visible: true,
            height: 120.0,
            frame: 0,
        }
    }

    pub fn pause() -> Self {
        Overlay {
            text: String::from("Paused"),
            color: Color::YELLOW,
            visible: true,
            height: 120.0,
            frame: 0,
        }
    }

    pub fn powerup() -> Self {
        Overlay {
            text: String::from("Pick two"),
            color: Color::GREEN,
            visible:true,
            height: 120.0,
            frame: 0,
        }
    }

    pub fn wave(nb: u32) -> Self {
        Overlay {
            text: format!("Wave {}", nb),
            color: Color::ORANGE,
            visible: true,
            height: 120.0,
            frame: 0,
        }
    }
    
    pub fn particles(&mut self) -> Vec<Particle> {
        if !self.visible { return vec![]; }

        self.frame += 1;
        let size = 12.0;
        let s = 50.0;
        if self.frame % 6 < 1 {
            vec![
                Particle {
                    pos: Vector::new(0.0, (SIZE.y + self.height) / 2.0),
                    speed: s,
                    shape: Shape::Shard(size / s, 2.5, true),
                    color: self.color,
                    ..Particle::default()
                }, 
                Particle {
                    pos: Vector::new(SIZE.x, (SIZE.y - self.height) / 2.0),
                    speed: s,
                    angle: 180.0,
                    shape: Shape::Shard(size / s, 2.5, true),
                    color: self.color,
                    ..Particle::default()
                }, 

            ]
        } else { vec![] }
    }


    pub fn draw(&self, gfx: &mut Graphics, font: &mut FontRenderer) {
        if !self.visible { return; }

        let w = 36.0 * self.text.len() as f32;
        let h = 40.0;

        let rect = Rectangle::new(
            Vector::new(0.0, SIZE.y / 2.0 - self.height / 2.0),
            Vector::new(SIZE.x, self.height),
        );
        gfx.fill_rect(&rect, Color::WHITE.with_alpha(0.2));

        font.draw(
            gfx, 
            &self.text,
            self.color,
            Vector::new(SIZE.x / 2.0 - w / 2.0, SIZE.y / 2.0 + h / 2.0),
        ).unwrap();
    }
}