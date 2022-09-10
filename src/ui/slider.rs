use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;

use crate::EventType;


pub struct HSlider{
   pub rect: Rect,
   pub cursor_position: f32,
   bg_color: Color,
   active_color: Color,
   pub linked_event: (Color, Color),
   clicked: bool

}

impl HSlider {
    pub fn new(x: i32, y: i32, width: u32, height:u32, color: Color, back_color: Color, linked_event: (Color, Color)) -> HSlider {
        let bg_color = Color::RGBA(back_color.r, back_color.g, back_color.b, 125);
        HSlider {
            rect: Rect::new(x, y, width, height),
            cursor_position: 0.5,
            bg_color,
            active_color: color,
            linked_event,
            clicked: false
        }
    }
    pub fn render(&self, canvas: &mut WindowCanvas) {
        let active_width = self.rect.width() as f32 * self.cursor_position;
        let active_rect = Rect::new(self.rect.x, self.rect.y, active_width as u32, self.rect.height());

        // First slider background
        canvas.set_draw_color(self.bg_color);
        canvas.fill_rect(self.rect);

        // ... then active rectangle
        canvas.set_draw_color(self.active_color); //self.active_color);
        canvas.fill_rect(active_rect);
    }

    pub fn click_on(&mut self, click_position: i32) {
        let delta_position = (self.rect.width() as f32 * self.cursor_position) as i32;
        if click_position > self.rect.x + delta_position {
            self.click_up()
        } else {
            self.click_down()
        }
    }

    pub fn click_up(&mut self) {
        self.cursor_position += 0.01;
    }

    pub fn click_down(&mut self) {
        self.cursor_position -= 0.01;
    }
}