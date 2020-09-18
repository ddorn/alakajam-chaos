// Example 1: The Square
// Open a window, and draw a colored square in it
use quicksilver::{
    geom::{Rectangle, Vector, Circle},
    graphics::Color,
    run, Graphics, Input, Result, Settings, Window,
};

struct Particle {
    pos: Vector,
    speed: f32,
    angle: f32,
    color: Color,
}


struct Game {
    particles: Vec<Particle>
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

    let bg_color = Color::from_rgba(2, 6, 23, 1.0);
    gfx.clear(bg_color);

    // Paint a blue square with a red outline in the center of our screen
    // It should have a top-left of (350, 100) and a size of (150, 100)
    let rect = Rectangle::new(Vector::new(350.0, 100.0), Vector::new(100.0, 100.0));
    let circle = Circle::new(Vector::new(350.0, 200.0), 42.0);

    let circle_color = Color { r: 0.0, g: 0.0, b: 0.0, a: 0.5};

    gfx.fill_rect(&rect, Color::ORANGE);
    gfx.fill_circle(&circle, circle_color);
    gfx.stroke_rect(&rect, Color::RED);
    // Send the data to be drawn
    gfx.present(&window)?;
    loop {
        while let Some(_) = input.next_event().await {}
    }
}
