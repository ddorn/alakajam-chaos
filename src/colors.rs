pub use quicksilver::graphics::Color;

///  Convert a HSV color in the range 0..1 to a Color
pub fn hsv2rgb(h: f32, s: f32, v: f32) -> Color
{
    let mut hh = h;

    if s <= 0.0 {       
        return Color { r: v, g: v, b: v, a: 1.0 }
    }
    
    if hh >= 360.0 { hh = 0.0; }

    hh /= 60.0;
    let i = hh.floor();
    // i = (long)hh;
    let ff = hh - i;

    let p = v * (1.0 - s);
    let q = v * (1.0 - (s * ff));
    let t = v * (1.0 - (s * (1.0 - ff)));

    let (r, g, b) = match i as i8 {
        0 | _ => (v, t, p),
        1 => (q, v, p),
        2 => (p, v, t),
        3 => (p, q, v),
        4 => (t, p, v),
        5 => (v, p, q),
    };

    Color { r, g, b, a: 1.0 }
}