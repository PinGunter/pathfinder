use raylib::prelude::*;

pub struct UI {
    mouse_position: Vector2,
    mouse_button: [bool; 3],
}

impl UI {
    pub fn new() -> Self {
        Self {
            mouse_position: Vector2 { x: 0.0, y: 0.0 },
            mouse_button: [false; 3],
        }
    }

    pub fn poll_events(&mut self, rl: &mut RaylibHandle) {
        self.mouse_position = rl.get_mouse_position();
        self.mouse_button[0] = rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT);
        self.mouse_button[1] = rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_RIGHT);
        self.mouse_button[2] = rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_MIDDLE);
    }

    pub fn button<'a>(
        &self,
        d: &mut RaylibDrawHandle<'_>,
        pos_x: i32,
        pos_y: i32,
        width: i32,
        height: i32,
        label: &'static str,
        text_color: Color,
        background_color: Color,
        border_color: Color,
        on_click: &mut dyn FnMut(),
    ) {
        let is_hovered = if (self.mouse_position.x as i32) > pos_x
            && (self.mouse_position.x as i32) < pos_x + width
            && (self.mouse_position.y as i32) > pos_y
            && (self.mouse_position.y as i32) < pos_y + height
        {
            true
        } else {
            false
        };

        if is_hovered && self.mouse_button[0] {
            {
                (on_click)();
            }
        }

        d.draw_rectangle(pos_x, pos_y, width, height, background_color);
        d.draw_rectangle_lines(
            pos_x - 1,
            pos_y - 1,
            width + 2,
            height + 2,
            if is_hovered {
                border_color
            } else {
                background_color
            },
        );
        d.draw_text(label, pos_x + 10, pos_y + 10, 20, text_color);
    }
}
