use raylib::prelude::*;
use std::collections::VecDeque;
use super::image::Image;
use super::cell::Cell;

pub struct Grid {
    tiles: Vec<Image>,
    width: usize,
    height: usize,
    cells: Vec<Cell>,
    rng: rand::rngs::ThreadRng,
}

impl Grid {
    pub fn new(width: usize, height: usize, image: &Image) -> Self {
        let tiles = image.slices(3, 3);
        let mut cells = vec![];
        for _ in 0..width * height {
            cells.push(Cell::new(tiles.len()));
        }
        Self {
            width,
            height,
            cells,
            tiles,
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
        if let Some((x, y)) = self.find_next_uncollapsed() {
            self.collapse(x, y);
            self.propagate(x, y);
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
        min_cell.collapse(&mut self.rng, &self.tiles);
    }

    fn propagate(&mut self, min_x: usize, min_y: usize) {
        let cell_index = min_y * self.width + min_x;
        let min_cell = &self.cells[cell_index].clone();

        let mut queue = VecDeque::new();
        
        queue.push_back((min_x, min_y));

        let neighbors = [(0, -1), (0, 1), (-1, 0), (1, 0)];

        while let Some((x, y)) = queue.pop_front() {
            for (dx, dy) in neighbors {
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;

                if !(nx >= 0 && ny >= 0 && nx < self.width as i32 && ny < self.height as i32) {
                    continue;
                }
                
                let nindex = ny as usize * self.width + nx as usize;
                let neighbor = &mut self.cells[nindex];
                if neighbor.is_collapsed() {
                    continue;
                }

                let before = neighbor.option_count();
                neighbor.reduce_options(&min_cell, dx, dy, &self.tiles);

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
