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
    
    // Add all the color slider
    let mut index: f32= 0.0;
    for part_a in color_list.iter() {
        for part_b in color_list.iter() {
            ui.add_hslider(
                WIDTH + 10.0,
                32.0 + index * 20.0,
                150.0,
                16.0,
                *part_a,
                *part_b,
                EventType::ColorInteraction(*part_a, *part_b)
            );
            index += 1.0;
        }
    }

    // add buttons
    index = 0.0;
    for part in color_list.iter() {
        // Increase button
        ui.add_button(
            WIDTH + 10.0 + 18.0 * index,
            330.0 + 32.0,
            16.0, 16.0,
            String::from("+"),
            BLACK, *part,
            EventType::ColorIncrease(*part, 100)
        );
        ui.add_button(
            WIDTH + 10.0 + 18.0 * (index + 1.0),
            330.0 + 32.0,
            16.0, 16.0,
            String::from("-"),
            BLACK, *part,
            EventType::ColorDecrease(*part, 100)
        );
        index += 4.0;

    }


    // Spawn some randomized particles
    // Particles amount Yellow/blue/red/white
    let mut particles_amount: [i32; 4] = [0, 0, 0, 0];
    let mut particles: Vec<Cell> = cell::cell_incubator(
        particles_amount[0],
        particles_amount[1],
        particles_amount[2],
        particles_amount[3],
    );

    loop {
        // Keyboard entries
        let mut ui_flag = false;
        if is_key_pressed(KeyCode::S) {
            for slider in ui.h_sliders.iter_mut() {
                slider.cursor_position = gen_range(0.4, 0.6);
                ui_flag = true;
            }
        }
        else if is_key_pressed(KeyCode::D) {
            particles = cell::cell_incubator(
                particles_amount[0],
                particles_amount[1],
                particles_amount[2],
                particles_amount[3],
            );
        }
        if is_mouse_button_pressed(MouseButton::Left) {
            ui.update(mouse_position().0, mouse_position().1);
            ui_flag = true;
        }

        // Updating rules if necessary
        if ui_flag {
            for slider in ui.h_sliders.iter() {
                let slider_event = match slider.linked_event {
                    EventType::ColorInteraction(x,y) => (x,y),
                    _ => (WHITE, WHITE),

                };
                let value = 0.5 - slider.cursor_position;
                rules.set_force(slider_event.0,  slider_event.1, value);
            }

            // Update particule amount
            for button in ui.buttons.iter() {
                if button.clicked {
                    match button.linked_event {
                        EventType::ColorIncrease(color, amount) => {
                            if particles_amount[value_of(color)] < 2000 {
                                particles_amount[value_of(color)] += amount;

                                particles = cell::cell_incubator(
                                    particles_amount[0],
                                    particles_amount[1],
                                    particles_amount[2],
                                    particles_amount[3],
                                );

                            }
                        },
                        EventType::ColorDecrease(color, amount) => {
                            if particles_amount[value_of(color)] > 0 {
                                particles_amount[value_of(color)] -= amount;
                                particles = cell::cell_incubator(
                                    particles_amount[0],
                                    particles_amount[1],
                                    particles_amount[2],
                                    particles_amount[3],
                                );
                            }
                        },
                        _ => {}
                    };

                }
            }
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