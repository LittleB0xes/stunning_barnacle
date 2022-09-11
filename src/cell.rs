use macroquad::prelude::*;
use std::time::{SystemTime, UNIX_EPOCH};

use super::{HEIGHT, WIDTH};

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

pub fn cell_incubator(cell_a: u32, cell_b: u32, cell_c: u32, cell_d: u32) -> Vec<Cell> {
    rand::srand(SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64);

    
    let mut particles: Vec<Cell> = Vec::new();
    for _i in 0..cell_a {
        let x: u32 = rand::gen_range(0, WIDTH as u32);
        let y: u32 = rand::gen_range(0, HEIGHT as u32);


        particles.push(Cell::new(x as f32, y as f32, YELLOW));
    }

    for _i in 0..cell_b {
        let x: u32 = rand::gen_range(0, WIDTH as u32);
        let y: u32 = rand::gen_range(0, HEIGHT as u32);

        particles.push(Cell::new(x as f32, y as f32, BLUE));
    }
    
    for _i in 0..cell_c {
        let x: u32 = rand::gen_range(0, WIDTH as u32);
        let y: u32 = rand::gen_range(0, HEIGHT as u32);

        particles.push(Cell::new(x as f32, y as f32, RED));
    }
    for _i in 0..cell_d {
        let x: u32 = rand::gen_range(0, WIDTH as u32);
        let y: u32 = rand::gen_range(0, HEIGHT as u32);

        particles.push(Cell::new(x as f32, y as f32, WHITE));
    }

    particles

}