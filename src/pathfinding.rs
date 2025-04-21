use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

use crate::grid::{Cell, Grid};

const INFINITY: f32 = 999999.9;

fn reconstruct_path(came_from: HashMap<Cell, Cell>, current: Cell) -> Vec<Cell> {
    let mut total_path = vec![current];
    let mut m_current = current;
    while came_from.contains_key(&m_current) {
        m_current = came_from[&m_current];
        total_path.push(m_current);
    }
    total_path.remove(0);
    total_path.pop();
    return total_path;
}

fn h(cell: Cell, goal: Cell) -> f32 {
    return (((goal.column - cell.column).pow(2) + (goal.row - cell.row).pow(2)) as f32).sqrt();
}

fn d(current: Cell, other: Cell, grid: &Grid) -> f32 {
    match grid.get_cell(other.row as usize, other.column as usize) {
        crate::grid::CellType::WALL => INFINITY,
        crate::grid::CellType::GOAL => -1.0,
        _ => 1.0,
    }
}

fn get_neighbours(cell: Cell, grid: &Grid) -> Vec<Cell> {
    let mut neighbours: Vec<Cell> = Vec::new();
    let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    for (row, column) in directions {
        let row_condition = cell.row + row >= 0 && cell.row + row < grid.get_grid_size() as i32;
        let column_condition =
            cell.column + column >= 0 && cell.column + column < grid.get_grid_size() as i32;
        if row_condition && column_condition && !(row == 0 && column == 0) {
            neighbours.push(Cell {
                row: cell.row + row,
                column: cell.column + column,
            });
        }
    }
    return neighbours;
}

struct CellCost {
    pub cell: Cell,
    pub cost: f32,
}

impl PartialEq for CellCost {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl Eq for CellCost {}

impl PartialOrd for CellCost {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // Reverse the comparison to make BinaryHeap a min-heap
        other.cost.partial_cmp(&self.cost)
    }
}

impl Ord for CellCost {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse the comparison to make BinaryHeap a min-heap
        other
            .cost
            .partial_cmp(&self.cost)
            .unwrap_or(Ordering::Equal)
    }
}

pub fn a_star(start: Cell, end: Cell, grid: &Grid) -> Vec<Cell> {
    let mut open_set: BinaryHeap<CellCost> = BinaryHeap::new();
    let mut came_from: HashMap<Cell, Cell> = HashMap::new();
    let mut g_score: HashMap<Cell, f32> = HashMap::new();
    let mut f_score: HashMap<Cell, f32> = HashMap::new();

    open_set.push(CellCost {
        cell: start,
        cost: 0.0,
    });

    g_score.insert(start, 0.0);

    f_score.insert(start, h(start, end));

    while !open_set.is_empty() {
        let current = open_set.pop().unwrap();

        if current.cost >= INFINITY {
            continue;
        }

        if current.cell == end {
            return reconstruct_path(came_from, current.cell);
        }

        for neighbour in get_neighbours(current.cell, grid) {
            let tentative_g_score = match g_score.get(&current.cell) {
                Some(score) => score + d(current.cell, neighbour, grid),
                _ => d(current.cell, neighbour, grid),
            };

            let current_g_score = g_score.get(&neighbour).or(Some(&INFINITY)).unwrap();
            if tentative_g_score < *current_g_score {
                came_from.insert(neighbour, current.cell);
                g_score.insert(neighbour, tentative_g_score);
                f_score.insert(neighbour, tentative_g_score + h(neighbour, end));
                open_set.push(CellCost {
                    cell: neighbour,
                    cost: tentative_g_score + h(neighbour, end),
                });
            }
        }
    }

    return vec![Default::default(); 0];
}
