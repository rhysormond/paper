use wasm_bindgen::prelude::*;

use crate::cell::Cell;
use crate::point::Point;
use crate::utils::{init, log};

#[wasm_bindgen]
pub struct World {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

impl World {
    fn points(&self) -> impl Iterator<Item = Point> + '_ {
        (0..self.height).flat_map(move |row| (0..self.width).map(move |col| Point { row, col }))
    }

    fn reset(&mut self) {
        log("resetting world");
        self.cells = self.points().map(|_p| Cell::Dead).collect();
    }

    fn index(&self, point: &Point) -> usize {
        (point.row * self.width + point.col) as usize
    }

    fn point(&self, row: &u32, col: &u32) -> Point {
        Point {
            row: row % self.height,
            col: col % self.width,
        }
    }

    fn neighbors(&self, point: &Point) -> Vec<Point> {
        let rows = [self.height - 1, 0, 1];
        let cols = [self.width - 1, 0, 1];
        rows.iter()
            .flat_map(|row| {
                cols.iter()
                    .map(move |col| self.point(&(point.row + row), &(point.col + col)))
            })
            .filter(|neighbor| neighbor != point)
            .collect()
    }

    fn live_neighbors(&self, point: &Point) -> u8 {
        self.neighbors(point)
            .iter()
            .map(|neighbor| self.cells[self.index(neighbor)] as u8)
            .sum()
    }

    fn next_state(&self, point: &Point) -> Cell {
        let cell = self.cells[self.index(point)];
        let neighbors = self.live_neighbors(point);
        match (cell, neighbors) {
            (Cell::Alive, x) if x < 2 => Cell::Dead,
            (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
            (Cell::Alive, x) if x > 3 => Cell::Dead,
            (Cell::Dead, 3) => Cell::Alive,
            (other, _) => other,
        }
    }
}

#[wasm_bindgen]
impl World {
    pub fn new(width: u32, height: u32) -> World {
        init();
        log("initializing new world");
        let mut world = World {
            width,
            height,
            cells: vec![],
        };
        world.reset();
        world
    }

    pub fn tick(&mut self) {
        self.cells = self.points().map(|p| self.next_state(&p)).collect();
    }

    pub fn toggle_cell(&mut self, row: u32, col: u32) {
        log(&format!("toggling cell ({}, {})", row, col));
        let idx = self.index(&Point { row, col });
        self.cells[idx].toggle();
    }

    pub fn get_cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }
}

#[cfg(test)]
mod tests {
    use super::{Cell, World};

    pub fn initial_state() -> Vec<Cell> {
        vec![
            Cell::Dead,
            Cell::Alive,
            Cell::Dead,
            Cell::Dead,
            Cell::Alive,
            Cell::Alive,
            Cell::Dead,
            Cell::Dead,
            Cell::Dead,
        ]
    }

    #[test]
    fn works() {
        let mut world = World {
            height: 3,
            width: 3,
            cells: initial_state(),
        };
        world.tick();
        assert_eq!(world.cells, [Cell::Alive; 9]);
        world.tick();
        assert_eq!(world.cells, [Cell::Dead; 9]);
    }
}
