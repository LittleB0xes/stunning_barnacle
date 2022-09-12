use macroquad::prelude::*;
use self::slider::HSlider;

use self::button::Button;

pub mod slider;
pub mod button;

// button and other stuff

pub enum EventType {
    ColorInteraction(Color, Color),
    ColorIncrease(Color, i32),
    ColorDecrease(Color, i32),
}

pub struct UI {
    pub update_flag: bool,
    pub h_sliders: Vec<HSlider>,
    pub buttons: Vec<Button>,
}

impl UI {
    pub fn init() -> Self {
        Self {
            update_flag: false,
            h_sliders: Vec::new(),
            buttons: Vec::new(),
        }

    }
    pub fn add_hslider(&mut self, x: f32, y: f32, width: f32, height: f32, color: Color, back_color: Color, linked_event: EventType) {
        self.h_sliders.push(HSlider::new(x, y, width, height, color, back_color, linked_event));
    }

    pub fn add_button(&mut self, x: f32, y: f32, w: f32, h: f32, label: String, label_color: Color, bg_color: Color, linked_event: EventType) {
        self.buttons.push(Button::new(x, y, w, h, label, label_color, bg_color, linked_event));
    }

    pub fn update(&mut self, mouse_x: f32, mouse_y: f32) {

        for slider in self.h_sliders.iter_mut() {
            if point_inside_rect(mouse_x, mouse_y, slider.rect) {
                slider.click_on(mouse_x);
            }
        }
        for button in self.buttons.iter_mut() {
            button.clicked = false;
            if point_inside_rect(mouse_x, mouse_y, button.rect) {
                button.clicked = true;

            }
        }
    }

    pub fn render(&mut self){
        for slider in self.h_sliders.iter() {
            slider.render();
        }

        for button in self.buttons.iter() {
            button.render();
        }
    }
}

fn point_inside_rect(x: f32, y: f32, rect: Rect) -> bool {
    x >= rect.x && x <= rect.x + rect.w && y >= rect.y && y <= rect.y + rect.h
}
