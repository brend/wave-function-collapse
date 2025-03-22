use std::collections::HashMap;

use rand::{rngs::ThreadRng, Rng};
use raylib::prelude::*;
use super::image::Image;
use super::direction::Direction;

pub const CELL_SIZE: i32 = 10;

type TileIndex = usize;

#[derive(Debug, Clone)]
pub struct Cell {
    options: Vec<usize>,
    collapsed: bool,
    pub checked: bool,
    color: Color,
}

impl Cell {
    pub fn new(option_count: usize) -> Self {
        Self {
            options: (0..option_count).collect(),
            collapsed: false,
            checked: false,
            color: Color::PEACHPUFF,
        }
    }

    pub fn update_color(&mut self, tiles: &Vec<Image>) {
        let n = self.options.len() as u32;

        if n == 0 {
            self.color = Color::MAGENTA;
            return;
        }
        
        let mut r: u32 = 0;
        let mut g: u32 = 0;
        let mut b: u32 = 0;
        for option_index in self.options.iter() {
            let option = &tiles[*option_index];
            let mx = option.width / 2;
            let my = option.height / 2;
            let color = option.pixels[my * option.width + mx];
            r += color.r as u32;
            g += color.g as u32;
            b += color.b as u32;
        }
        
        self.color = Color::new((r / n) as u8, (g / n) as u8, (b / n) as u8, 255)
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle, x: i32, y: i32) {
        d.draw_rectangle(x * CELL_SIZE, y * CELL_SIZE, CELL_SIZE, CELL_SIZE, self.color);
    }

    pub fn collapse(&mut self, rng: &mut ThreadRng) {
        if self.collapsed {
            return;
        }
        let index = rng.gen_range(0..self.options.len());
        self.options = vec![self.options[index]];
        self.collapsed = true;
    }

    pub fn is_collapsed(&self) -> bool {
        self.collapsed
    }

    pub fn option_count(&self) -> usize {
        self.options.len()
    }

    pub fn reduce_options(&mut self, other: &Cell, dir: Direction, adjacencies: &HashMap<(TileIndex, Direction, TileIndex), bool>) {
        if self.collapsed {
            return;
        }
        self.options.retain(|&x| {
            other.options.iter().any(|&y| {
                adjacencies.get(&(x, dir, y)).copied().unwrap_or(false)
            })
        });
        if self.options.len() == 1 {
            self.collapsed = true;
        }
    }
}
