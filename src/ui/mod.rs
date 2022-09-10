use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use sdl2::rect::Rect;

use crate::EventType;

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
    pub fn add_hslider(&mut self, x: i32, y: i32, width: u32, height: u32, color: Color, back_color: Color, linked_event: (Color, Color)) {
        self.h_sliders.push(HSlider::new(x, y, width, height, color, back_color, linked_event));
    }

    pub fn update(&mut self, mouse_x: i32, mouse_y: i32, button_state: bool) {

        for slider in self.h_sliders.iter_mut() {
            if point_inside_rect(mouse_x, mouse_y, slider.rect) {
                slider.click_on(mouse_x);
            }
        }
    }

    pub fn render(&mut self, canvas: &mut WindowCanvas) {
        for slider in self.h_sliders.iter() {
            slider.render(canvas);
        }
    }
}

fn point_inside_rect(x: i32, y: i32, rect: Rect) -> bool {
    x >= rect.x && x <= rect.x + rect.x + rect.width() as i32 && y >= rect.y && y <= rect.y + rect.height() as i32
}
