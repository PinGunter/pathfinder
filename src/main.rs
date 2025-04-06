use raylib::prelude::*;

mod grid;
mod ui;

use grid::Grid;
use ui::UI;

const GRID_WINDOW_SIZE: usize = 1000;
const GRID_SIZE: usize = 25;
const SQUARE_SIZE: usize = GRID_WINDOW_SIZE / GRID_SIZE;
const WINDOW_W: i32 = GRID_WINDOW_SIZE as i32 + 200;
const WINDOW_H: i32 = GRID_WINDOW_SIZE as i32;

fn main() {
    assert!((GRID_WINDOW_SIZE as f64 / GRID_SIZE as f64) == SQUARE_SIZE as f64);
    let (mut rl, thread) = raylib::init()
        .size(WINDOW_W, WINDOW_H)
        .title("Pathfinder")
        .build();

    let mut grid: Grid = Grid::new(GRID_WINDOW_SIZE, GRID_SIZE, SQUARE_SIZE, 0, 0);
    let mut ui: UI = UI::new();

    while !rl.window_should_close() {
        // event polling
        let mouse_position = rl.get_mouse_position();

        if grid.mouse_in_grid(mouse_position) {
            let row = mouse_position.y as usize / SQUARE_SIZE;
            let column = mouse_position.x as usize / SQUARE_SIZE;
            if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
                grid.toggle_cell(row, column);
            }
            grid.highlight_cell(row, column);
        }

        ui.poll_events(&mut rl);

        // Start rendering
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        // draw grid
        grid.draw(&mut d);

        // draw button
        ui.button(
            &mut d,
            1000,
            0,
            200,
            50,
            "Draw Walls",
            Color::WHITE,
            Color::BLACK,
            Color::WHEAT,
            Box::new(|| println!("Drawing walls")),
        );

        ui.button(
            &mut d,
            1000,
            52,
            200,
            50,
            "Draw Goal",
            Color::WHITE,
            Color::BLACK,
            Color::WHEAT,
            Box::new(|| println!("Drawing goals")),
        );
    }
}
