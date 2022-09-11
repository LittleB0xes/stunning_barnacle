use sdl2::pixels::Color;

use crate::cell::Cell;

use super::{HEIGHT, WIDTH};


pub fn interaction(particles: &mut Vec<Cell>, rules: &Rules) {
    let part_number = particles.len();
    for i in 0..part_number {
        let mut ax = 0.0;
        let mut ay = 0.0;
        for j in 0..part_number {
            let dx = particles[i].x - particles[j].x;
            let dy = particles[i].y - particles[j].y;
            //let dist = dx*dx + dy*dy;
            let dist = f32::sqrt(dx*dx + dy*dy);
            if dist > 0.0 && dist < 80.0 {
                let rules_factor = rules.get_force(particles[i].color, particles[j].color);
                let force = rules_factor / dist;
                ax += force * dx;
                ay += force * dy;
            }            
        }
        particles[i].vx = (particles[i].vx + ax) * 0.25;
        particles[i].vy = (particles[i].vy + ay) * 0.25;
        let next_x = particles[i].x + particles[i].vx;
        let next_y = particles[i].y + particles[i].vy;

        if next_x < 0.0 || next_x > WIDTH {
            particles[i].vx *= -1.0;
        }

        if next_y < 0.0|| next_y > HEIGHT{
            particles[i].vy *= -1.0;
        }

        particles[i].x += particles[i].vx * 0.8; 
        particles[i].y += particles[i].vy * 0.8; 
    }
}

pub struct Rules {
    rules: [[f32; 4]; 4],
    //friction: f32,
}

impl Rules {
    pub fn new() -> Self {
        Self {
            rules: [
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
            ],
               
            //friction: 0.0,
        }
    }
    pub fn set_force(&mut self, color_a: Color, color_b: Color, value: f32) {
        let i: usize = match color_a {
            Color::YELLOW   => 0,
            Color::BLUE     => 1,
            Color::RED      => 2,
            Color::WHITE    => 3,
            _               => 0,
        };
        let j: usize = match color_b {
            Color::YELLOW   => 0,
            Color::BLUE     => 1,
            Color::RED      => 2,
            Color::WHITE    => 3,
            _               => 0,
        };
        self.rules[i][j] = value;

    }

    pub fn get_force(&self, color_a: Color, color_b: Color) -> f32 {
        let i: usize = match color_a {
            Color::YELLOW   => 0,
            Color::BLUE     => 1,
            Color::RED      => 2,
            Color::WHITE    => 3,
            _               => 0,
        };
        let j: usize = match color_b {
            Color::YELLOW   => 0,
            Color::BLUE     => 1,
            Color::RED      => 2,
            Color::WHITE    => 3,
            _               => 0,
        };
        self.rules[i][j]
    }

}
