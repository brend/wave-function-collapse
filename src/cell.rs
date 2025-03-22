use rand::{rngs::ThreadRng, Rng};
use raylib::prelude::*;
use super::image::Image;

pub const CELL_SIZE: i32 = 10;

#[derive(Debug, Clone)]
pub struct Cell {
    options: Vec<usize>,
    collapsed: bool,
    color: Color,
}

impl Cell {
    pub fn new(option_count: usize) -> Self {
        Self {
            options: (0..option_count).collect(),
            collapsed: false,
            color: Color::PEACHPUFF,
        }
    }

    fn update_color(&mut self, tiles: &Vec<Image>) {
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

    pub fn collapse(&mut self, rng: &mut ThreadRng, tiles: &Vec<Image>) {
        let index = rng.gen_range(0..self.options.len());
        self.options = vec![self.options[index]];
        self.collapsed = true;
        self.update_color(tiles);
    }

    pub fn is_collapsed(&self) -> bool {
        self.collapsed
    }

    pub fn option_count(&self) -> usize {
        self.options.len()
    }

    pub fn reduce_options(&mut self, other: &Cell, dx: i32, dy: i32, tiles: &Vec<Image>) {
        let before = self.options.len();
        self.options.retain(|&x| other.options.iter().any(|&y| tiles[x].fits(&tiles[y], dx, dy)));
        if self.options.len() != before {
            self.update_color(tiles);
        }
        if self.options.len() == 1 {
            self.collapsed = true;
        }
    }
}
