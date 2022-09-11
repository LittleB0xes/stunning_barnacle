use macroquad::prelude::*;
use self::slider::HSlider;

pub mod slider;

// button and other stuff

pub struct UI {
    pub h_sliders: Vec<HSlider>,
}

impl UI {
    pub fn init() -> Self {
        let h_sliders = Vec::new();
        Self {
            h_sliders,
        }

    }
    pub fn add_hslider(&mut self, x: f32, y: f32, width: f32, height: f32, color: Color, back_color: Color, linked_event: (Color, Color)) {
        self.h_sliders.push(HSlider::new(x, y, width, height, color, back_color, linked_event));
    }

    pub fn update(&mut self, mouse_x: f32, mouse_y: f32) {

        for slider in self.h_sliders.iter_mut() {
            if point_inside_rect(mouse_x, mouse_y, slider.rect) {
                slider.click_on(mouse_x);
            }
        }
    }

    pub fn render(&mut self){
        for slider in self.h_sliders.iter() {
            slider.render();
        }
    }
}

fn point_inside_rect(x: f32, y: f32, rect: Rect) -> bool {
    x >= rect.x && x <= rect.x + rect.x + rect.w && y >= rect.y && y <= rect.y + rect.h
}
