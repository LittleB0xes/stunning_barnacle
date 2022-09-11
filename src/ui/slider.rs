use macroquad::prelude::*;

pub struct HSlider{
   pub rect: Rect,
   pub cursor_position: f32,
   bg_color: Color,
   active_color: Color,
   pub linked_event: (Color, Color),

}

impl HSlider {
    pub fn new(x: f32, y: f32, width: f32, height:f32, color: Color, back_color: Color, linked_event: (Color, Color)) -> HSlider {
        let bg_color = Color::new(back_color.r, back_color.g, back_color.b, 0.5);
        HSlider {
            rect: Rect::new(x, y, width, height),
            cursor_position: 0.5,
            bg_color,
            active_color: color,
            linked_event,
        }
    }
    pub fn render(&self) {
        let active_width = self.rect.w * self.cursor_position;
        let active_rect = Rect::new(self.rect.x, self.rect.y, active_width, self.rect.h);

        // First slider background
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, self.bg_color);

        // ... then active rectangle
        draw_rectangle(active_rect.x, active_rect.y, active_rect.w, active_rect.h, self.active_color);
    }

    pub fn click_on(&mut self, click_position: f32) {
        let delta_position = self.rect.w *  self.cursor_position;
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