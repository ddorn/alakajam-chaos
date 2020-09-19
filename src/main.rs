use quicksilver::{
    geom::{Vector},
    graphics::{Color, VectorFont, FontRenderer, ResizeHandler},
    input::Event,
    run, Graphics, Input, Result, Settings, Window, Timer,
};

use rand::{Rng, RngCore, SeedableRng, distributions::{Uniform, Normal, Distribution}};
use rand_xorshift::XorShiftRng;

mod colors;
mod particles;
mod player;
mod shot;

use colors::*;
use particles::*;
use player::*;
use shot::*;

const SIZE: Vector = Vector { x: 1300.0, y: 800.0 };

struct Game {
    particles: Vec<Particle>,
    bg_color: Color,
    rng: XorShiftRng,
    font: FontRenderer,
    player: Player,
    shots: Vec<Shot>,
}

impl Game {
    fn new(font: FontRenderer) -> Self {
        Game { 
            particles: vec![],
            bg_color: Color::from_hex("#020812"),
            rng: XorShiftRng::from_seed([42; 16]),
            font: font,
            player: Player::new(),
            shots: vec![],
        }
    }

    /// Draw the entire game on the gfx. `prop` is the
    /// proportion of time between the last update and the next
    /// prop is in the range 0..1
    fn draw(&mut self, gfx: &mut Graphics, prop: f32, render_skip: usize) {
        gfx.clear(self.bg_color);

        for p in &self.particles {
            p.draw(gfx, prop);
        }

        self.font.draw(
            gfx, 
            &format!("Particles: {} \nSkip: {}", self.particles.len(), render_skip), 
            Color::WHITE, 
            Vector::new(10.0, 50.0)
        ).unwrap();
    }

    fn update(&mut self, input: &Input, mouse: Vector) {

        // Update and remove dead particles
        self.particles = self.particles
            .iter_mut()
            .filter_map(|p| if p.update() { Some(p.clone()) } else { None } )
            .collect();

        // Update and remove shots
        for s in &mut self.shots {
            self.particles.extend(s.update(&mut self.rng));
        }
        self.shots = self.shots
            .iter()
            .filter_map(|s| if s.alive() { Some(s.clone()) } else { None })
            .collect();

        self.particles.extend(self.player.update(mouse, &mut self.rng));
    }

    fn event(&mut self, event: Event, input: &Input, mouse: Vector) {
        match event {
            Event::PointerInput(p) => {
                if p.is_down() {
                    self.shots.push(
                        self.player.fire(mouse)
                    );
                }
            },
            _ => ()
        }
    }
}



fn main() {
    run(
        Settings {
            size: SIZE,
            title: "Square Example",
            resizable: true,
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

    let mut render_skip = 0;

    
    // Game loop
    loop {
        let mouse = gfx.screen_to_camera(&window, input.mouse().location());

        // Event handeling
        while let Some(event) = input.next_event().await {
            game.event(event, &input, mouse)
        }

        // We use a while loop rather than an if so that we can try to catch up in the event of having a slow down.
        while update_timer.tick() {
            game.update(&input, mouse);
        }

        // Unlike the update cycle drawing doesn't change our state
        // Because of this there is no point in trying to catch up if we are ever 2 frames late
        // Instead it is better to drop/skip the lost frames
        if let Some(frames) = draw_timer.exhaust() {
            render_skip += match frames.get() {
                0 | 1 => 0,
                s => s - 2,
            };

            let update_prop = update_timer.elapsed().as_secs_f32() / update_timer.period().as_secs_f32();

            game.draw(&mut gfx, update_prop, render_skip);
            // Send the data to be drawn
            gfx.present(&window)?;
        }
    }
}
