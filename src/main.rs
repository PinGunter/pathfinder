use pathfinding::a_star;
use raylib::prelude::*;

mod grid;
mod pathfinding;
mod ui;
use grid::Grid;
use ui::UI;

const GRID_WINDOW_SIZE: usize = 1000;
const GRID_SIZE: usize = 25;
const SQUARE_SIZE: usize = GRID_WINDOW_SIZE / GRID_SIZE;
const WINDOW_W: i32 = GRID_WINDOW_SIZE as i32 + 200;
const WINDOW_H: i32 = GRID_WINDOW_SIZE as i32;

enum ClickMode {
    NONE,
    START,
    WALL,
    GOAL,
}

fn main() {
    assert!((GRID_WINDOW_SIZE as f64 / GRID_SIZE as f64) == SQUARE_SIZE as f64);
    let (mut rl, thread) = raylib::init()
        .size(WINDOW_W, WINDOW_H)
        .title("Pathfinder")
        .build();

    let mut grid: Grid = Grid::new(GRID_WINDOW_SIZE, GRID_SIZE, SQUARE_SIZE, 0, 0);
    let mut ui: UI = UI::new();
    let mut click_mode: ClickMode = ClickMode::NONE;

    while !rl.window_should_close() {
        // event polling
        let mouse_position = rl.get_mouse_position();

        if grid.mouse_in_grid(mouse_position) {
            let row = mouse_position.y as i32 / SQUARE_SIZE as i32;
            let column = mouse_position.x as i32 / SQUARE_SIZE as i32;
            if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
                match click_mode {
                    ClickMode::START => grid.set_cell(grid::CellType::START, row, column),
                    ClickMode::WALL => grid.set_cell(grid::CellType::WALL, row, column),
                    ClickMode::GOAL => grid.set_cell(grid::CellType::GOAL, row, column),
                    _ => {}
                }
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
            1001,
            0,
            198,
            50,
            "Place starting \nplace",
            Color::WHITE,
            Color::BLACK,
            Color::WHEAT,
            &mut || click_mode = ClickMode::START,
        );

        ui.button(
            &mut d,
            1001,
            52,
            198,
            50,
            "Draw Walls",
            Color::WHITE,
            Color::BLACK,
            Color::WHEAT,
            &mut || click_mode = ClickMode::WALL,
        );

        ui.button(
            &mut d,
            1001,
            104,
            198,
            50,
            "Draw Goal",
            Color::WHITE,
            Color::BLACK,
            Color::WHEAT,
            &mut || click_mode = ClickMode::GOAL,
        );

        ui.button(
            &mut d,
            1001,
            156,
            198,
            50,
            "Clear",
            Color::WHITE,
            Color::BLACK,
            Color::WHEAT,
            &mut || grid.clear(),
        );
        ui.button(
            &mut d,
            1001,
            208,
            198,
            50,
            "START",
            Color::WHITE,
            Color::BLACK,
            Color::WHEAT,
            &mut || {
                let t_path = &a_star(
                    grid.get_start().unwrap_or_default(),
                    grid.get_goal().unwrap_or_default(),
                    &grid,
                );
                for cell in t_path {
                    println!("{}, {}", cell.row, cell.column);
                    grid.set_cell(grid::CellType::PATH, cell.row, cell.column);
                }
                if t_path.is_empty() {
                    println!("No possible path found");
                }
            },
        );
    }
}
