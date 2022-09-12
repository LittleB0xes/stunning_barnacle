use macroquad::prelude::*;
use macroquad::rand::gen_range;
use std::time::{SystemTime, UNIX_EPOCH};


//use std::time::{Instant, Duration};
use std::f32;


use ui::{UI, EventType};
mod ui;

use cell::Cell;
mod cell;

use interaction::*;
use interaction::Rules;
mod interaction;

const WIDTH: f32 = 500.0;
const UI_WIDTH: f32 = 300.0;
const HEIGHT: f32 = 500.0;





#[macroquad::main(window_conf)]
async fn main() {
    rand::srand(SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64);
    let color_list = [YELLOW, BLUE, RED, WHITE];
    let mut rules = Rules::new();

    let mut ui = UI::init();
    let mut index = 0;
    for part_a in color_list.iter() {
        for part_b in color_list.iter() {
            ui.add_hslider(
                WIDTH + 10.0,
                32.0 + (index * 20) as f32,
                150.0,
                16.0,
                *part_a,
                *part_b,
                EventType::ColorInteraction(*part_a, *part_b)
            );
            index += 1;
        }

    }

    // Spawn some randomized particles
    let mut particles: Vec<Cell> = cell::cell_incubator(1000, 1000, 1000, 1000);

    loop {
        // Keyboard entries
        if is_key_pressed(KeyCode::S) {
            for slider in ui.h_sliders.iter_mut() {
                slider.cursor_position = gen_range(0.4, 0.6);
            }
        }
        else if is_key_pressed(KeyCode::D) {
            particles = cell::cell_incubator(1000, 1000, 1000, 1000);
        }
        if is_mouse_button_pressed(MouseButton::Left) {
            ui.update(mouse_position().0, mouse_position().1);
        }
        // Updating rules
        for slider in ui.h_sliders.iter() {
            let slider_event = match slider.linked_event {
                EventType::ColorInteraction(x,y) => (x,y),
                _ => (WHITE, WHITE),

            };
            let value = 0.5 - slider.cursor_position;
            rules.set_force(slider_event.0,  slider_event.1, value);
        }
        // Updating interaction
        // It acually use a brute force method. A futur step
        // is to use quadtree


        interaction(&mut particles,&rules);
        clear_background(BLACK);
        
        for part in particles.iter() {
            draw_circle(part.x, part.y, 1.5, part.color);
            //draw_rectangle(part.x, part.y, 3.0, 3.0, part.color);
        }

        ui.render();


        // print some useful data
        for slider in ui.h_sliders.iter() {
            let slider_event = match slider.linked_event {
                EventType::ColorInteraction(x,y) => (x,y),
                _ => (WHITE, WHITE),

            };
            let value = rules.get_force(slider_event.0, slider_event.1);
            let x = slider.rect.x + slider.rect.w + 10.0;
            let y = slider.rect.y + slider.rect.h * 0.8;
            
            draw_text( &format!("{:.2}", value), x, y, 16.0, WHITE);
        }
        
        //draw_text( &format!("FPS: {}", get_fps()), WIDTH + 10.0, 15.0, 16.0, WHITE);
        draw_text( "'D' to randomize positions", WIDTH + 10.0, HEIGHT - 48.0, 16.0, WHITE);
        draw_text( "'S' to randomize rules", WIDTH + 10.0, HEIGHT - 24.0, 16.0, WHITE);
        next_frame().await
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Stunning Barnacle".to_owned(),
        window_width: (WIDTH + UI_WIDTH) as i32,
        window_height: HEIGHT as i32,
        fullscreen: false,
        high_dpi: true,
        ..Default::default()
    }
}


//fn main() -> Result<(), String> {
//
//    let sdl_context = sdl2::init()?;
//
//    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
//
//    // load font
//    let font = ttf_context.load_font("./assets/Hack-Regular.ttf", 12)?;
//
//    let video_subsystem = sdl_context.video()?;
//    let _image_context = sdl2::image::init(InitFlag::PNG)?;
//
//    let window = video_subsystem.window("Stunning Barnacle", UI_WIDTH + WIDTH as u32, HEIGHT as u32)
//        .position_centered()
//        .allow_highdpi()
//        .resizable()
//        .allow_highdpi()
//        .build()
//        .expect("Could not initialize video subsystem");
//    
//    let mut canvas = window.into_canvas().build().expect("Could not create a canvas");
//    canvas.set_logical_size(UI_WIDTH + WIDTH as u32, HEIGHT as u32).expect("Logical Size Error");
//
//    // Blendmode is important to use transparency
//    canvas.set_blend_mode(BlendMode::Blend);
//
//    let texture_creator = canvas.texture_creator();
//    let mut event_pump = sdl_context.event_pump()?;
//
//
//    // Random init
//    let mut rng = rand::thread_rng();
//
//    let mut ui = UI::init();
//    let mut rules = Rules::new();
//    let color_list = [Color::YELLOW, Color::BLUE, Color::RED, Color::WHITE];
//
//    let mut index = 0;
//    for part_a in color_list.iter() {
//        for part_b in color_list.iter() {
//            ui.add_hslider(WIDTH as i32 + 10, 32 + index * 20, 150, 16, *part_a, *part_b, (*part_a, *part_b));
//            index += 1;
//        }
//
//    }
//    
//
//    // Spawn some randomized particles
//    let mut particles: Vec<Cell> = cell::cell_incubator(1000, 1000, 1000, 1000);
//    
//
//
//
//    // Main Loop
//    'running: loop {
//        //let begining = Instant::now();
//
//        // Updating rules
//        for slider in ui.h_sliders.iter() {
//            let value = 0.5 - slider.cursor_position;
//            rules.set_force(slider.linked_event.0, slider.linked_event.1, value);
//        }
//        // Updating interaction
//        // It acually use a brute force method. A futur step
//        // is to use quadtree
//        interaction(&mut particles, &rules);
//
//        // Check ui
//        let mut mouse_x: i32 = -1;
//        let mut mouse_y: i32 = -1;
//
//        // Rendering
//        canvas.clear();
//        canvas.set_draw_color(Color::RGB(0,0,0));
//        canvas.fill_rect(Rect::new(0,0,WIDTH as u32 + UI_WIDTH, HEIGHT as u32))?;
//
//        for part in particles.iter() {
//            let destination_rect = Rect::new(part.x as i32, part.y as i32, 2, 2);
//            //canvas.set_draw_color(Color::YELLOW);
//            canvas.set_draw_color(part.color);
//            canvas.fill_rect(destination_rect)?;
//        }
//
//        for event in event_pump.poll_iter() {
//            match event {
//                Event::Quit {..} |
//                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
//                    break 'running;
//                },
//                Event::KeyDown { keycode: Some(Keycode::A), .. } => {
//                    for slider in ui.h_sliders.iter_mut() {
//                        slider.cursor_position = ((0.4 + 0.2 * rng.gen::<f32>()) * 100.0).round() / 100.0;
//                    }
//                },
//                Event::KeyDown { keycode: Some(Keycode::Z), .. } => {
//                    particles = cell::cell_incubator(1000, 1000, 1000, 1000);
//                },
//                Event::MouseButtonDown {x, y, .. } => {
//                    mouse_x = x;
//                    mouse_y = y;
//                }
//                _ => {}
//            }
//        }
//        
//        // UI update
//        ui.update(mouse_x, mouse_y);
//
//
//
//        // Time per frame calculation
//        //let end = Instant::now();
//        //let tps = format!("TPF: {}",  (end - begining).as_millis());
//        //render_text(&mut canvas, &texture_creator, &font, WIDTH as i32 + 10, 0,&tps, Color::WHITE);
//
//        render_text(&mut canvas, &texture_creator, &font, WIDTH as i32 + 10, HEIGHT as i32 - 20,"'A' to randomize rules", Color::WHITE)?;
//        render_text(&mut canvas, &texture_creator, &font, WIDTH as i32 + 10, HEIGHT as i32 - 40,"'Z' to reset", Color::WHITE)?;
//        
//        // Slider rendering
//        ui.render(&mut canvas)?;
//
//        // print some useful data
//        for slider in ui.h_sliders.iter() {
//            let value = rules.get_force(slider.linked_event.0, slider.linked_event.1);
//            let x = slider.rect.x() + slider.rect.width() as i32 + 10;
//            let y = slider.rect.y();
//            render_text(&mut canvas, &texture_creator, &font, x, y, &format!("{:.2}", value), Color::WHITE)?;
//        }
//        
//        canvas.present();
//        //::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
//    }
//    Ok(())
//}
