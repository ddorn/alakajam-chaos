// Example 1: The Square
// Open a window, and draw a colored square in it
use quicksilver::{
    geom::{Rectangle, Vector, Circle},
    graphics::{Color, VectorFont, FontRenderer},
    run, Graphics, Input, Result, Settings, Window, Timer
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
    damp: f32,
    angular_vel: f32,
    color: Color,
}

impl Particle {
    fn update(&mut self) -> bool {
        self.pos = self.pos + Vector::from_angle(self.angle) * self.speed;
        self.speed *= self.damp;
        self.angle += self.angular_vel;
        self.angle %= 360.0;
        // self.angular_vel *= self.damp;


        self.speed > 0.5
            && self.pos.x > -100.0
            && self.pos.y > -100.0
            && self.pos.x < 5000.0
            && self.pos.y < 5000.0

    }

    fn draw(&self, gfx: &mut Graphics, prop: f32) {
        gfx.fill_circle(
            &Circle::new(
                self.pos + Vector::from_angle(self.angle) * (self.speed * prop),
                3.0 * self.speed.sqrt()), 
            self.color.with_alpha(self.speed / 10.0),
        );
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
    fn update(&mut self, input: &Input, rng: &mut XorShiftRng) -> Vec<Particle>
    {

        // Move towards the cursor
        let mouse = input.mouse().location();
        let dir = mouse - self.pos;
        let dist = dir.len().max(1.0);  // Avoid div by 0
        self.pos += dir * ((dist * 0.1).min(20.0) / dist); // max N pixels, otherwise prop to dist

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

    /// Draw the entire game on the gfx. `prop` is the
    /// proportion of time between the last update and the next
    /// prop is in the range 0..1
    fn draw(&mut self, gfx: &mut Graphics, prop: f32) {
        gfx.clear(self.bg_color);

        for p in &self.particles {
            p.draw(gfx, prop);
        }

        self.font.draw(gfx, &format!("Particles: {}", self.particles.len()), Color::WHITE, Vector::new(10.0, 50.0)).unwrap();
    }

    fn update(&mut self, input: &Input) {

        // Update and remove dead particles
        self.particles = self.particles
            .iter_mut()
            .filter_map(|p| if p.update() { Some(p.clone()) } else { None } )
            .collect();

        self.particles.extend(self.player.update(input, &mut self.rng));
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

    let mut update_timer = Timer::time_per_second(30.0);
    let mut draw_timer = Timer::time_per_second(60.0);

    // Game loop
    loop {
        // Event handeling
        while let Some(_) = input.next_event().await {

        }

        // We use a while loop rather than an if so that we can try to catch up in the event of having a slow down.
        while update_timer.tick() {
            game.update(&input);
        }

        // Unlike the update cycle drawing doesn't change our state
        // Because of this there is no point in trying to catch up if we are ever 2 frames late
        // Instead it is better to drop/skip the lost frames
        if draw_timer.exhaust().is_some() {
            let update_prop = update_timer.elapsed().as_secs_f32() / update_timer.period().as_secs_f32();

            game.draw(&mut gfx, update_prop);
            // Send the data to be drawn
            gfx.present(&window)?;
        }
    }
}
