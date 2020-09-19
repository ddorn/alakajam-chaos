use quicksilver::geom::{Vector, Circle};
use quicksilver::graphics::{Color, Graphics};


#[derive(Copy, Clone, Debug)]
pub struct Particle {
    pub pos: Vector,
    pub speed: f32,
    pub angle: f32,
    pub damp: f32,
    pub angular_vel: f32,
    pub color: Color,
}

impl Particle {
    pub fn update(&mut self) -> bool {
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

    pub fn draw(&self, gfx: &mut Graphics, prop: f32) {
        gfx.fill_circle(
            &Circle::new(
                self.pos + Vector::from_angle(self.angle) * (self.speed * prop),
                3.0 * self.speed.sqrt()), 
            self.color.with_alpha(self.speed / 10.0),
        );
    }
}