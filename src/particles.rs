use quicksilver::geom::{Vector, Circle, Triangle };
use quicksilver::graphics::{Color, Graphics, Vertex, Mesh, Element};

#[derive(Copy, Clone, Debug)]
pub enum Shape {
    /// Circle(size)
    Circle(f32),
    /// Shard(size, ratio)
    Shard(f32, f32)
}

#[derive(Copy, Clone, Debug)]
pub struct Particle {
    pub pos: Vector,
    pub speed: f32,
    pub angle: f32,
    pub damp: f32,
    pub accel: f32,
    pub angular_vel: f32,
    pub shape: Shape,
    pub color: Color,
}

impl Particle {
    pub fn update(&mut self) -> bool {
        self.pos = self.pos + Vector::from_angle(self.angle) * self.speed;

        self.speed = (self.speed + self.accel) * self.damp;

        self.angle = (self.angle + self.angular_vel) % 360.0;
        self.angular_vel *= self.damp;


        self.speed > 2.0
            && self.pos.x > -100.0
            && self.pos.y > -100.0
            && self.pos.x < 5000.0
            && self.pos.y < 5000.0

    }

    pub fn draw(&self, gfx: &mut Graphics, prop: f32) {
        match self.shape {
            Shape::Circle(size) => {
                gfx.fill_circle(
                    &Circle::new(
                        self.pos + Vector::from_angle(self.angle) * (self.speed * prop),
                        size * self.speed.sqrt()), 
                    self.color.with_alpha(self.speed / 10.0),
                );
            }
            Shape::Shard(size, ratio) => {
                let vertices = {
                    let vel = Vector::from_angle(self.angle) * self.speed * size;
                    let cross = Vector::new(-vel.y, vel.x);

                    let front = Vertex {
                        pos: self.pos + vel,
                        uv: None,
                        color: Color::GREEN,
                    };
                    let left = Vertex {
                        pos: self.pos + cross,
                        uv: None,
                        color: Color::RED,
                    };
                    let right = Vertex {
                        pos: self.pos - cross,
                        uv: None,
                        color: Color::ORANGE,
                    };
                    let back = Vertex {
                        pos: self.pos - vel * ratio,
                        uv: None,
                        color: Color::BLUE.with_alpha(0.0),
                    };

                    vec![front, left, right, back]
                };
                // A triangle is simply a pointer to indices of the vertices
                let elements = vec![
                    Element::Triangle([0, 1, 2]),
                    Element::Triangle([1, 2, 3]),
                ];
                // Bring the vertices and the triangle elements together to define a mesh
                let mesh = Mesh {
                    vertices,
                    elements,
                    image: None,
                };
                // Pass a reference to the mesh to the graphics object to draw
                gfx.draw_mesh(&mesh);
            }
        }
    }
}