use raylib::prelude::*;

pub struct Button<'a> {
    pos_x: i32,
    pos_y: i32,
    width: i32,
    height: i32,
    label: &'static str,
    text_color: Color,
    background_color: Color,
    border_color: Color,
    on_click: Box<dyn Fn() + 'a>,
    is_hovered: bool,
    is_clicked: bool,
}

impl<'a> Button<'a> {
    pub fn new(
        pos_x: i32,
        pos_y: i32,
        width: i32,
        height: i32,
        label: &'static str,
        text_color: Color,
        background_color: Color,
        border_color: Color,
        on_click: Box<dyn Fn() + 'a>,
    ) -> Self {
        Self {
            pos_x,
            pos_y,
            width,
            height,
            label,
            text_color,
            background_color,
            border_color,
            on_click,
            is_hovered: false,
            is_clicked: false,
        }
    }

    pub fn render(&self, d: &mut RaylibDrawHandle<'_>) {
        d.draw_rectangle(
            self.pos_x,
            self.pos_y,
            self.width,
            self.height,
            self.background_color,
        );
        d.draw_rectangle_lines(
            self.pos_x - 1,
            self.pos_y - 1,
            self.width + 2,
            self.height + 2,
            if self.is_hovered {
                self.border_color
            } else {
                self.background_color
            },
        );
        d.draw_text(
            self.label,
            self.pos_x + 10,
            self.pos_y + 10,
            20,
            self.text_color,
        );
    }
    pub fn poll_events(&mut self, rl: &mut RaylibHandle) {
        let mouse_pos = rl.get_mouse_position();
        self.is_hovered = if (mouse_pos.x as i32) > self.pos_x
            && (mouse_pos.x as i32) < self.pos_x + self.width
            && (mouse_pos.y as i32) > self.pos_y
            && (mouse_pos.y as i32) < self.pos_y + self.height
        {
            true
        } else {
            false
        };
        self.is_clicked = false;

        if self.is_hovered && rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
            {
                self.is_clicked = true;
                (self.on_click)();
            }
        }
    }
}
