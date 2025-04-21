use raylib::prelude::*;
use std::ops::{Index, IndexMut};

#[derive(Eq, PartialEq, Hash, Clone, Copy, Default)]
pub struct Cell {
    pub row: i32,
    pub column: i32,
}

#[derive(Clone, Copy, PartialEq)]
pub enum CellType {
    EMPTY,
    WALL,
    START,
    GOAL,
    PATH,
}

pub struct Grid {
    window_size: usize,
    grid_size: usize,
    square_size: usize,
    offset_x: usize,
    offset_y: usize,
    grid: Vec<Vec<CellType>>,
    highlight: Option<Cell>,
    goal: Option<Cell>,
    start: Option<Cell>,
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
            grid: vec![vec![CellType::EMPTY; grid_size]; grid_size],
            highlight: None,
            goal: None,
            start: None,
        }
    }

    pub fn mouse_in_grid(&self, mouse_pos: Vector2) -> bool {
        return (mouse_pos.x as usize) < self.window_size
            && (mouse_pos.y as usize) < self.window_size;
    }

    pub fn set_cell(&mut self, state: CellType, row: i32, column: i32) {
        if state == CellType::GOAL {
            if self.goal.is_some() {
                self.grid[self.goal.as_ref().unwrap().row as usize]
                    [self.goal.as_ref().unwrap().column as usize] = CellType::EMPTY;
            }
            self.goal = Some(Cell { row, column });
        }
        if state == CellType::START {
            if self.start.is_some() {
                self.grid[self.start.as_ref().unwrap().row as usize]
                    [self.start.as_ref().unwrap().column as usize] = CellType::EMPTY;
            }
            self.start = Some(Cell { row, column });
        }
        self.grid[row as usize][column as usize] = state;
    }

    pub fn get_cell(&self, row: usize, column: usize) -> CellType {
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

    pub fn highlight_cell(&mut self, row: i32, column: i32) {
        self.highlight = Some(Cell { row, column });
    }

    fn cell_type_to_color(&self, cell_type: CellType) -> Color {
        match cell_type {
            CellType::EMPTY => Color::WHITE,
            CellType::GOAL => Color::RED,
            CellType::START => Color::GREEN,
            CellType::WALL => Color::WHITE,
            CellType::PATH => Color::BLUE,
        }
    }

    pub fn clear(&mut self) {
        self.grid = vec![vec![CellType::EMPTY; self.grid_size]; self.grid_size];
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle<'_>) {
        let mut opacity: f32;
        for row in 0..self.grid_size {
            for column in 0..self.grid_size {
                opacity = match &self.highlight {
                    Some(cell) if cell.row as usize == row && cell.column as usize == column => 1.0,
                    _ => 0.3,
                };
                let fill = self.grid[row][column] != CellType::EMPTY;
                self.draw_square(
                    d,
                    column * self.square_size,
                    row * self.square_size,
                    self.square_size,
                    self.cell_type_to_color(self.grid[row][column])
                        .alpha(if fill { 1.0 } else { opacity }),
                    fill,
                );
            }
        }
    }

    pub fn get_start(&self) -> Option<Cell> {
        return self.start;
    }

    pub fn get_goal(&self) -> Option<Cell> {
        return self.goal;
    }

    pub fn get_grid_size(&self) -> usize {
        return self.grid_size;
    }
}

impl Index<usize> for Grid {
    type Output = Vec<CellType>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.grid[index]
    }
}

impl IndexMut<usize> for Grid {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.grid[index]
    }
}
