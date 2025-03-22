use raylib::prelude::*;
use std::collections::{HashMap, VecDeque};
use super::image::Image;
use super::cell::Cell;
use super::direction::Direction;

type TileIndex = usize;

fn compute_adjacencies(tiles: &Vec<Image>) -> HashMap<(TileIndex, Direction, TileIndex), bool> {
    let mut adjacencies = HashMap::new();
    for (i, tile) in tiles.iter().enumerate() {
        for (j, other) in tiles.iter().enumerate() {
            for dir in [Direction::North, Direction::South, Direction::East, Direction::West].iter() {
                let tiles_fit = tile.fits(&other, dir.offsets());
                adjacencies.insert((i, dir.clone(), j), tiles_fit);
            }
        }
    }

    adjacencies
}

pub struct Grid {
    tiles: Vec<Image>,
    width: usize,
    height: usize,
    cells: Vec<Cell>,
    adjacencies: HashMap<(TileIndex, Direction, TileIndex), bool>,
    rng: rand::rngs::ThreadRng,
}

impl Grid {
    pub fn new(width: usize, height: usize, image: &Image) -> Self {
        let tiles = image.slices(3, 3);
        let adjacencies = compute_adjacencies(&tiles);
        let mut cells = vec![];
        for _ in 0..width * height {
            cells.push(Cell::new(tiles.len()));
        }
        Self {
            width,
            height,
            cells,
            tiles,
            adjacencies,
            rng: rand::thread_rng(),
        }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        for y in 0..self.height {
            for x in 0..self.width {
                let cell = &self.cells[y * self.width + x];
                cell.draw(d, x as i32, y as i32);
            }
        }
    }

    pub fn update(&mut self) {
        let cell = self.find_next_uncollapsed();
        if let Some((x, y)) = cell {
            self.collapse(x, y);
            self.propagate(x, y);

            for cell in self.cells.iter_mut() {
                cell.update_color(&self.tiles);
            }
        }
    }

    fn find_next_uncollapsed(&self) -> Option<(usize, usize)> {
        let mut min_options = usize::MAX;
        let mut min_x = 0;
        let mut min_y = 0;
        for y in 0..self.height {
            for x in 0..self.width {
                let cell = &self.cells[y * self.width + x];
                if !cell.is_collapsed() {
                    let n = cell.option_count();
                    if n < min_options && n > 1 {
                        min_options = n;
                        min_x = x;
                        min_y = y;
                    }
                }
            }
        }

        if min_options == 1 {
            return None;
        }

        Some((min_x, min_y))
    }

    fn collapse(&mut self, min_x: usize, min_y: usize) {
        let cell_index = min_y * self.width + min_x;
        let min_cell = &mut self.cells[cell_index];
        min_cell.collapse(&mut self.rng);
    }

    fn propagate(&mut self, min_x: usize, min_y: usize) {
        for cell in self.cells.iter_mut() {
            cell.checked = false;
        }

        let mut queue = VecDeque::new();
        
        queue.push_back((min_x, min_y));

        while let Some((x, y)) = queue.pop_front() {
            let cell = &mut self.cells[y * self.width + x];

            if cell.checked {
                continue;
            }

            cell.checked = true;

            let cell = cell.clone();

            for dir in [Direction::North, Direction:: East, Direction::South, Direction::West].iter() {
                let (dx, dy) = dir.offsets();
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;

                if !(nx >= 0 && ny >= 0 && nx < self.width as i32 && ny < self.height as i32) {
                    continue;
                }
                
                let nindex = ny as usize * self.width + nx as usize;
                let neighbor = &mut self.cells[nindex];
                if neighbor.checked || neighbor.is_collapsed() {
                    continue;
                }

                let before = neighbor.option_count();
                neighbor.reduce_options(&cell, *dir, &self.adjacencies);

                if neighbor.option_count() == 0 {
                    println!("⚠️ Contradiction: No options left at ({}, {})", nx, ny);
                }

                if neighbor.option_count() < before {
                    queue.push_back((nx as usize, ny as usize));
                }
            }
        }
    }
}
