use sdl2::pixels::Color;
pub struct Cell {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub color: Color,
}

impl Cell {
    pub fn new(x: f32, y: f32, color: Color) -> Self {
        Self {
            x,
            y,
            vx: 0.0,
            vy: 0.0,
            color
        }
    }
}