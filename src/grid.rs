use raylib::prelude::*;
use std::ops::{Index, IndexMut};

pub struct Grid {
    window_size: usize,
    grid_size: usize,
    square_size: usize,
    offset_x: usize,
    offset_y: usize,
    grid: Vec<Vec<bool>>,
    highlight_row: Option<usize>,
    highligt_column: Option<usize>,
}

impl Grid {
    pub fn new(
        window_size: usize,
        grid_size: usize,
        square_size: usize,
        offset_x: usize,
        offset_y: usize,
    ) -> Grid {
        Grid {
            window_size,
            grid_size,
            square_size,
            offset_x,
            offset_y,
            grid: vec![vec![false; grid_size]; grid_size],
            highlight_row: None,
            highligt_column: None,
        }
    }

    pub fn mouse_in_grid(&self, mouse_pos: Vector2) -> bool {
        return (mouse_pos.x as usize) < self.window_size
            && (mouse_pos.y as usize) < self.window_size;
    }

    pub fn set_cell(&mut self, state: bool, row: usize, column: usize) {
        self.grid[row][column] = state;
    }

    pub fn get_cell(&self, row: usize, column: usize) -> bool {
        return self.grid[row][column];
    }

    fn draw_square(
        &self,
        d: &mut RaylibDrawHandle<'_>,
        x: usize,
        y: usize,
        size: usize,
        color: Color,
        fill: bool,
    ) {
        if fill {
            d.draw_rectangle(x as i32, y as i32, size as i32, size as i32, color);
        } else {
            d.draw_rectangle_lines(x as i32, y as i32, size as i32, size as i32, color);
        }
    }

    pub fn highlight_cell(&mut self, row: usize, column: usize) {
        self.highlight_row = Some(row);
        self.highligt_column = Some(column);
    }

    pub fn toggle_cell(&mut self, row: usize, column: usize) {
        self.grid[row][column] = !self.grid[row][column];
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle<'_>) {
        let mut opacity: f32;
        for row in 0..self.grid_size {
            for column in 0..self.grid_size {
                opacity = if self.highlight_row == Some(row) && self.highligt_column == Some(column)
                {
                    1.0
                } else {
                    0.3
                };
                let fill = self.grid[row][column];
                self.draw_square(
                    d,
                    column * self.square_size,
                    row * self.square_size,
                    self.square_size,
                    Color::WHITE.alpha(if fill { 1.0 } else { opacity }),
                    fill,
                );
            }
        }
    }
}

impl Index<usize> for Grid {
    type Output = Vec<bool>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.grid[index]
    }
}

impl IndexMut<usize> for Grid {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.grid[index]
    }
}
