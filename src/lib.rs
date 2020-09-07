use wasm_bindgen::prelude::*;

mod utils;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(target: &str) {
    alert(&*format!("Hello, {}!", target));
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

#[wasm_bindgen]
pub struct World {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Point {
    row: u32,
    col: u32,
}

impl World {
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
}

#[wasm_bindgen]
impl World {
    pub fn tick(&mut self) {
        let points =
            (0..self.height).flat_map(|row| (0..self.width).map(move |col| Point { row, col }));

        let next = points
            .map(|point| {
                let cell = self.cells[self.index(&point)];
                match (cell, self.live_neighbors(&point)) {
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    (Cell::Dead, 3) => Cell::Alive,
                    (other, _) => other,
                }
            })
            .collect();

        self.cells = next;
    }

    pub fn new() -> World {
        let width = 64;
        let height = 64;

        let cells = (0..width * height)
            .map(|i| {
                if i % 2 == 0 || i % 7 == 0 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();

        World {
            width,
            height,
            cells,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }
}
