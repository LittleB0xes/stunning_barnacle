use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::rect::Rect;
use sdl2::keyboard::Keycode;
use sdl2::image::InitFlag;
use sdl2::render::{WindowCanvas, TextureCreator, BlendMode};

use rand::prelude::*;
use sdl2::ttf::Font;
use sdl2::video::WindowContext;

//use std::time::{Instant, Duration};
use std::f32;


use ui::UI;
mod ui;

use cell::Cell;
mod cell;

use interaction::*;
use interaction::Rules;
mod interaction;

const WIDTH: f32 = 500.0;
const UI_WIDTH: u32 = 300;
const HEIGHT: f32 = 500.0;




fn render_text(canvas: &mut WindowCanvas, texture_creator: &TextureCreator<WindowContext>, font: &Font,x: i32, y: i32, text: &str, color: Color) -> Result<(), String>{
    let text_size = font.size_of(text).unwrap();

    let surface = font
        .render(text)
        .blended(color)
        .map_err(|e| e.to_string())?;


    let texture = texture_creator
        .create_texture_from_surface(&surface)
        .map_err(|e| e.to_string())?;



    let target = Rect::new(
    x,
    y,
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
    let font = ttf_context.load_font("./assets/Hack-Regular.ttf", 12)?;

    let video_subsystem = sdl_context.video()?;
    let _image_context = sdl2::image::init(InitFlag::PNG)?;

    let window = video_subsystem.window("Stunning Barnacle", UI_WIDTH + WIDTH as u32, HEIGHT as u32)
        .position_centered()
        .allow_highdpi()
        .resizable()
        .allow_highdpi()
        .build()
        .expect("Could not initialize video subsystem");
    
    let mut canvas = window.into_canvas().build().expect("Could not create a canvas");
    canvas.set_logical_size(UI_WIDTH + WIDTH as u32, HEIGHT as u32).expect("Logical Size Error");

    // Blendmode is important to use transparency
    canvas.set_blend_mode(BlendMode::Blend);

    let texture_creator = canvas.texture_creator();
    let mut event_pump = sdl_context.event_pump()?;


    // Random init
    let mut rng = rand::thread_rng();

    let mut ui = UI::init();
    let mut rules = Rules::new();
    let color_list = [Color::YELLOW, Color::BLUE, Color::RED, Color::WHITE];

    let mut index = 0;
    for part_a in color_list.iter() {
        for part_b in color_list.iter() {
            ui.add_hslider(WIDTH as i32 + 10, 32 + index * 20, 150, 16, *part_a, *part_b, (*part_a, *part_b));
            index += 1;
        }

    }
    

    // Spawn some randomized particles
    let mut particles: Vec<Cell> = cell::cell_incubator(1000, 1000, 1000, 1000);
    



    // Main Loop
    'running: loop {
        //let begining = Instant::now();

        // Updating rules
        for slider in ui.h_sliders.iter() {
            let value = 0.5 - slider.cursor_position;
            rules.set_force(slider.linked_event.0, slider.linked_event.1, value);
        }
        // Updating interaction
        // It acually use a brute force method. A futur step
        // is to use quadtree
        interaction(&mut particles, &rules);

        // Check ui
        let mut mouse_x: i32 = -1;
        let mut mouse_y: i32 = -1;

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
                Event::KeyDown { keycode: Some(Keycode::A), .. } => {
                    for slider in ui.h_sliders.iter_mut() {
                        slider.cursor_position = ((0.4 + 0.2 * rng.gen::<f32>()) * 100.0).round() / 100.0;
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::Z), .. } => {
                    particles = cell::cell_incubator(1000, 1000, 1000, 1000);
                },
                Event::MouseButtonDown {x, y, .. } => {
                    mouse_x = x;
                    mouse_y = y;
                }
                _ => {}
            }
        }
        
        // UI update
        ui.update(mouse_x, mouse_y);



        // Time per frame calculation
        //let end = Instant::now();
        //let tps = format!("TPF: {}",  (end - begining).as_millis());
        //render_text(&mut canvas, &texture_creator, &font, WIDTH as i32 + 10, 0,&tps, Color::WHITE);

        render_text(&mut canvas, &texture_creator, &font, WIDTH as i32 + 10, HEIGHT as i32 - 20,"'A' to randomize rules", Color::WHITE)?;
        render_text(&mut canvas, &texture_creator, &font, WIDTH as i32 + 10, HEIGHT as i32 - 40,"'Z' to reset", Color::WHITE)?;
        
        // Slider rendering
        ui.render(&mut canvas)?;

        // print some useful data
        for slider in ui.h_sliders.iter() {
            let value = rules.get_force(slider.linked_event.0, slider.linked_event.1);
            let x = slider.rect.x() + slider.rect.width() as i32 + 10;
            let y = slider.rect.y();
            render_text(&mut canvas, &texture_creator, &font, x, y, &format!("{:.2}", value), Color::WHITE)?;
        }
        
        canvas.present();
        //::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
    Ok(())
}
