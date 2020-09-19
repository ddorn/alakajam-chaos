use quicksilver::{
    geom::{Vector, Transform},
    graphics::{Color, VectorFont, FontRenderer, },
    input::{Event, Key},
    run, Graphics, Input, Result, Settings, Window, Timer,
};

use rand::{SeedableRng, distributions::{Uniform, Normal, Distribution}};
use rand_xorshift::XorShiftRng;
use std::mem::swap;

mod colors;
mod particles;
mod player;
mod shot;
mod enemy;

use colors::*;
use particles::*;
use player::*;
use shot::*;
use enemy::*;

const SIZE: Vector = Vector { x: 1300.0, y: 800.0 };

fn sqrti(x: u32) -> u32 {
    (x as f64).sqrt() as u32
}

/// Return whether a vector is in the screen, with a 50 pixels margin
fn in_screen(pos: &Vector) -> bool {
    pos.x > -50.0
    && pos.y > -50.0
    && pos.x < SIZE.x + 50.0
    && pos.y < SIZE.y + 50.0
}


pub struct Game {
    // Utilities
    bg_color: Color,
    rng: XorShiftRng,
    font: FontRenderer,
    // Entities
    particles: Vec<Particle>,
    player: Player,
    enemies: Vec<Enemy>,
    shots: Vec<Shot>,
    // General
    frame: u32,
    paused: bool,
    score: u32,
    shake: i32,
}

impl Game {
    fn new(font: FontRenderer) -> Self {
        Game { 
            bg_color: Color::from_hex("#020812"),
            rng: XorShiftRng::from_seed([42; 16]),
            font: font,

            particles: vec![],
            player: Player::new(),
            shots: vec![],
            enemies: vec![ Enemy::new(Vector::ONE * -50.0, 5)],

            paused: false,
            score: 0,
            frame: 0,
            shake: 0,
        }
    }

    /// Draw the entire game on the gfx. `prop` is the
    /// proportion of time between the last update and the next
    /// prop is in the range 0..1
    fn draw(&mut self, gfx: &mut Graphics, mut prop: f32, render_skip: usize) {
        if self.paused || self.player.life == 0 {
            // Otherwise things jitter when paused.
            prop = 0.0;
        }
        gfx.clear(self.bg_color);

        if self.shake > 0 {
            self.shake -= 1;
            let unif = Uniform::new(-15.0, 15.0);
            let x = unif.sample(&mut self.rng);
            let y = unif.sample(&mut self.rng);
            gfx.set_transform(Transform::translate(Vector::new(x, y)));
        } else {
            gfx.set_transform(Transform::IDENTITY);
        }

        for p in &self.particles {
            p.draw(gfx, prop);
        }

        self.font.draw(
            gfx, 
            &format!(
                "Score: {}
Life: {}
Enemies: {}
Particles: {}
Skip: {}", 
                self.score, 
                "<3 ".repeat(self.player.life as usize),
                self.enemies.len(),
                self.particles.len(), 
                render_skip,
            ), 
            Color::WHITE, 
            Vector::new(10.0, 50.0)
        ).unwrap();

        if self.player.life == 0 {
            self.font.draw(
                gfx, 
                &"GAME OVER", 
                Color::RED, 
                Vector::new(SIZE.x / 2.0 - 170.0, SIZE.y / 2.0),
            ).unwrap();
        } else if self.paused {
            self.font.draw(
                gfx, 
                &"Paused !", 
                Color::YELLOW, 
                Vector::new(SIZE.x / 2.0 - 140.0, SIZE.y / 2.0),
            ).unwrap();
        }
    }

    fn update(&mut self, mouse: Vector) {
        if self.paused { return; }
        if self.player.life == 0 { return; }

        self.frame += 1;

        // Spawn enemies if needed
        self.spawn_enemy();

        // Update and remove dead particles
        // We do it first so particles added this frame can
        // be drawn where they spawn at least once
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
            .filter_map(|s| if s.alive { Some(s.clone()) } else { None })
            .collect();

        // Update and remove enemies
        let mut enn = vec![];
        let mut new_enn = vec![];
        swap(&mut enn, &mut self.enemies);
        self.enemies = enn
            .iter()
            .filter_map(|e| {
                let mut e = e.clone();
                new_enn.extend(e.update(self));

                if e.alive {
                    Some(e)
                } else {
                    self.score += 1;
                    None
                }
            })
            .collect();
        self.enemies.extend(new_enn);

        // Update the player
        Player::update(mouse, self);
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
            Event::KeyboardInput(e) => {
                if e.is_down() {
                    match e.key() {
                        Key::P => self.toggle_pause(),
                        Key::R => {
                            // Entities
                            self.player = Player::new();
                            self.enemies = vec![ Enemy::new(Vector::ONE * -50.0, 5)];
                            self.shots = vec![];
                            // General
                            self.frame = 0;
                            self.paused = false;
                            self.score = 0;
                        }
                        _ => (),
                    }
                }
            }
            _ => ()
        }
    }

    fn toggle_pause(&mut self) {
        self.paused = !self.paused;
    }

    fn spawn_enemy(&mut self) {
        if self.frame % (42 + sqrti(self.score)) != 17 {return;}

        // Find a position out of the screen
        let unif = Uniform::new(-500.0, 2000.0);
        let mut pos = Vector::ZERO;
        while in_screen(&pos) {
            pos.x = unif.sample(&mut self.rng);
            pos.y = unif.sample(&mut self.rng);
        }

        let unif = Uniform::new_inclusive(1, (self.score as f64).sqrt() as usize / 4 + 2);
        let life = unif.sample(&mut self.rng);

        self.enemies.push(
            Enemy::new(pos, life as u32)
        );
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
            game.update(mouse);
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
