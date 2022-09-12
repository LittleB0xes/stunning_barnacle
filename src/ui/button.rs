use macroquad::prelude::*;

use super::EventType;


/// Basic pressable button with label
pub struct Button {
    pub rect: Rect,
    bg_color: Color,
    label_color: Color,
    label: String,
    pub linked_event: EventType,
    pub clicked: bool,
}

impl Button {

    /// Create a new button
    pub fn new(x: f32, y:f32, w:f32, h: f32, label: String, label_color:Color, bg_color: Color, linked_event: EventType) -> Button{
        Button {
            rect: Rect::new(x, y, w, h),
            bg_color,
            label_color,
            label,
            linked_event,
            clicked: false,
        }
    }

    pub fn render(&self) {

        // Render the box
        draw_rectangle(
            self.rect.x,
            self.rect.y,
            self.rect.w,
            self.rect.h,
            self.bg_color
        );

        // Render the label
        draw_text(&self.label, self.rect.x, self.rect.y + self.rect.h, 32.0, self.label_color);
    }
}