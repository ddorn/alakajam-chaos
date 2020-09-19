// Example 1: The Square
// Open a window, and draw a colored square in it
use quicksilver::{
    geom::{Rectangle, Vector, Circle},
    graphics::{Color, VectorFont, FontRenderer},
    run, Graphics, Input, Result, Settings, Window,
};

use rand::{Rng, RngCore, SeedableRng, distributions::{Uniform, Normal, Distribution}};
use rand_xorshift::XorShiftRng;

mod colors;
use colors::*;

#[derive(Copy, Clone, Debug)]
struct Particle {
    pos: Vector,
    speed: f32,
    angle: f32,
    color: Color,
}

impl Particle {
    fn update(&mut self) -> bool {
        self.pos = self.pos + Vector::from_angle(self.angle) * self.speed;
        self.speed -= 1.0;

        self.speed > 0.0 
            && self.pos.x > -100.0
            && self.pos.y > -100.0
            && self.pos.x < 5000.0
            && self.pos.y < 5000.0
    }
}

struct Player {
    pos: Vector,
}


impl Player {
    fn new() -> Self {
        Player {
            pos: Vector::new(200.0, 200.0),
        }
    }
    fn update(&mut self, rng: &mut XorShiftRng) -> Vec<Particle>
    {
        let angle = Uniform::new(0.0, 360.0);
        let speed = Normal::new(15.0, 3.0);
        let hue = Normal::new(36.0, 3.0);

        let mut ps = vec![];
        for _ in 0..4 {
            ps.push(Particle {
                pos: self.pos,
                speed: speed.sample(rng) as f32,
                angle: angle.sample(rng),
                color: hsv2rgb(hue.sample(rng) as f32, 1.0, 1.0)
            });
        }

        ps
    }
}

struct Game {
    particles: Vec<Particle>,
    bg_color: Color,
    rng: XorShiftRng,
    font: FontRenderer,
    player: Player,
}

impl Game {
    fn new(font: FontRenderer) -> Self {
        Game { 
            particles: vec![],
            bg_color: Color::from_hex("#020812"),
            rng: XorShiftRng::from_seed([42; 16]),
            font: font,
            player: Player::new(),
        }
    }

    fn draw(&mut self, gfx: &mut Graphics) {
        gfx.clear(self.bg_color);

        for p in &self.particles {
            gfx.fill_circle(
                &Circle::new(p.pos, 10.0), 
                p.color
            );
        }

        self.font.draw(gfx, &format!("Particles: {}", self.particles.len()), Color::WHITE, Vector::new(10.0, 50.0)).unwrap();
    }

    fn update(&mut self) {

        self.particles.extend(self.player.update(&mut self.rng));

        // Update and remove dead particles
        let mut new = vec![];
        for p in &mut self.particles {
            if p.update() {
                new.push(p.clone())
            }
        }
        self.particles = new;
    }
}



fn main() {
    run(
        Settings {
            title: "Square Example",
            ..Settings::default()
        },
        app,
    );
}

async fn app(window: Window, mut gfx: Graphics, mut input: Input) -> Result<()> {
    let ttf = VectorFont::load("ThaleahFat.ttf").await?;
    let font = ttf.to_renderer(&gfx, 72.0)?;

    let mut game = Game::new(font);

    // Game loop
    loop {
        // Event handeling
        while let Some(_) = input.next_event().await {

        }
        
        game.update();

        game.draw(&mut gfx);
        // Send the data to be drawn
        gfx.present(&window)?;
    }
}
