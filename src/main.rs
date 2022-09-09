use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::rect::Rect;
use sdl2::keyboard::Keycode;
use sdl2::image::InitFlag;
use sdl2::render::{WindowCanvas, TextureCreator};

use rand::prelude::*;
use sdl2::ttf::Font;
use sdl2::video::WindowContext;

use std::time::{Instant, Duration};
use std::f32;

const WIDTH: f32 = 640.0;
const UI_WIDTH: u32 = 200;
const HEIGHT: f32 = 640.0;


struct Cell {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    color: Color,
}

impl Cell {
    fn new(x: f32, y: f32, color: Color) -> Self {
        Self {
            x,
            y,
            vx: 0.0,
            vy: 0.0,
            color
        }
    }
}

fn interaction(particles: &mut Vec<Cell>) {
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
                let rules_factor = rules(particles[i].color, particles[j].color);
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

fn rules(color_a: Color, color_b: Color) -> f32 {
    match (color_a, color_b) {
        (Color::YELLOW, Color::YELLOW) => -0.15,
        (Color::YELLOW, Color::BLUE) => 0.14,
        (Color::YELLOW, Color::RED) => 0.11,
        (Color::YELLOW, Color::WHITE) => 0.0,
        (Color::BLUE, Color::BLUE) => -0.03,
        (Color::BLUE, Color::YELLOW) => -0.12,
        (Color::BLUE, Color::RED) => 0.12,
        (Color::RED, Color::RED) => 0.3,
        (Color::RED, Color::YELLOW) => 0.12,
        (Color::RED, Color::BLUE) => 0.12,
        (Color::WHITE, Color::YELLOW) => -0.05,
        (Color::WHITE, Color::BLUE) => 0.05,

        _ => 0.0,
    }
}

fn render_text(canvas: &mut WindowCanvas, texture_creator: &TextureCreator<WindowContext>, font: &Font, text: &str, color: Color) -> Result<(), String>{
    let text_size = font.size_of(text).unwrap();

    let surface = font
        .render(text)
        .blended(color)
        .map_err(|e| e.to_string())?;


    let texture = texture_creator
        .create_texture_from_surface(&surface)
        .map_err(|e| e.to_string())?;


    //let TextureQuery { width, height, .. } = texture.query();

    let target = Rect::new(
    WIDTH as i32 + 10 ,
    0,
    text_size.0,
    text_size.1,
    );

    canvas.copy(&texture, None, Some(target))?;

    Ok(())
}

fn main() -> Result<(), String> {

    let sdl_context = sdl2::init()?;

    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;

    // load font
    let font = ttf_context.load_font("./assets/Hack-Regular.ttf", 16)?;

    let video_subsystem = sdl_context.video()?;
    let _image_context = sdl2::image::init(InitFlag::PNG)?;

    let window = video_subsystem.window("Cell Life", UI_WIDTH + WIDTH as u32, HEIGHT as u32)
        .position_centered()
        .allow_highdpi()
        .resizable()
        .allow_highdpi()
        .build()
        .expect("Could not initialize video subsystem");
    
    let mut canvas = window.into_canvas().build().expect("Could not create a canvas");
    canvas.set_logical_size(UI_WIDTH + WIDTH as u32, HEIGHT as u32).expect("Logical Size Error");

    let texture_creator = canvas.texture_creator();


    

    let mut event_pump = sdl_context.event_pump()?;

    let mut rng = rand::thread_rng();

    // Spawn some randomized particles
    let mut particles: Vec<Cell> = Vec::new();
    for _i in 0..1500 {
        let x: u32 = rng.gen::<u32>() % (WIDTH as u32);
        let y: u32 = rng.gen::<u32>() % (HEIGHT as u32);

        particles.push(Cell::new(x as f32, y as f32, Color::YELLOW));
    }

    for _i in 0..1500 {
        let x: u32 = rng.gen::<u32>() % (WIDTH as u32);
        let y: u32 = rng.gen::<u32>() % (HEIGHT as u32);

        particles.push(Cell::new(x as f32, y as f32, Color::BLUE));
    }
    
    for _i in 0..1500 {
        let x: u32 = rng.gen::<u32>() % (WIDTH as u32);
        let y: u32 = rng.gen::<u32>() % (HEIGHT as u32);

        particles.push(Cell::new(x as f32, y as f32, Color::RED));
    }
    for _i in 0..1500 {
        let x: u32 = rng.gen::<u32>() % (WIDTH as u32);
        let y: u32 = rng.gen::<u32>() % (HEIGHT as u32);

        particles.push(Cell::new(x as f32, y as f32, Color::WHITE));
    }
    // Main Loop
    'running: loop {
        let begining = Instant::now();

        // Updating
        interaction(&mut particles);

        // Rendering
        canvas.clear();
        canvas.set_draw_color(Color::RGB(0,0,0));
        canvas.fill_rect(Rect::new(0,0,WIDTH as u32 + UI_WIDTH, HEIGHT as u32))?;

        for part in particles.iter() {
            let destination_rect = Rect::new(part.x as i32, part.y as i32, 2, 2);
            //canvas.set_draw_color(Color::YELLOW);
            canvas.set_draw_color(part.color);
            canvas.fill_rect(destination_rect)?;

        }

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                },
                _ => {}
            }
        }
        let end = Instant::now();
        let tps = format!("TPF: {}",  (end - begining).as_millis());
        

        render_text(&mut canvas, &texture_creator, &font, &tps, Color::WHITE);
        canvas.present();
        //::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
    Ok(())
}
