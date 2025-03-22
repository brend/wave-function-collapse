use rand::Rng;
use raylib::prelude::*;
use std::collections::VecDeque;

mod image;
use image::{load_image_from_png, Image};

const SCREEN_WIDTH: i32 = 400;
const SCREEN_HEIGHT: i32 = 400;

const CELL_SIZE: i32 = 10;

const TITLE: &str = "Wave Function Collapse";

#[derive(Debug, Clone)]
struct Cell {
    options: Vec<usize>,
    collapsed: bool,
    color: Color,
}

impl Cell {
    fn new(option_count: usize) -> Self {
        Self {
            options: (0..option_count).collect(),
            collapsed: false,
            color: Color::PEACHPUFF,
        }
    }

    pub fn average_color(&self) -> Color {
        self.color
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

    fn draw(&self, d: &mut RaylibDrawHandle, x: i32, y: i32) {
        d.draw_rectangle(x * CELL_SIZE, y * CELL_SIZE, CELL_SIZE, CELL_SIZE, self.color);
    }

    fn collapse(&mut self, rng: &mut rand::rngs::ThreadRng, tiles: &Vec<Image>) {
        let index = rng.gen_range(0..self.options.len());
        self.options = vec![self.options[index]];
        self.collapsed = true;
        self.update_color(tiles);
    }
}

struct Grid {
    tiles: Vec<Image>,
    width: usize,
    height: usize,
    cells: Vec<Cell>,
}

impl Grid {
    fn new(width: usize, height: usize, image: &Image) -> Self {
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
        }
    }

    fn draw(&self, d: &mut RaylibDrawHandle) {
        for y in 0..self.height {
            for x in 0..self.width {
                let cell = &self.cells[y * self.width + x];
                cell.draw(d, x as i32, y as i32);
            }
        }
    }

    fn update(&mut self) {
        let mut min_options = usize::MAX;
        let mut min_x = 0;
        let mut min_y = 0;
        for y in 0..self.height {
            for x in 0..self.width {
                let cell = &self.cells[y * self.width + x];
                if !cell.collapsed {
                    let n = cell.options.len();
                    if n < min_options && n > 1 {
                        min_options = n;
                        min_x = x;
                        min_y = y;
                    }
                }
            }
        }

        if min_options == 1 {
            return;
        }

        let cell_index = min_y * self.width + min_x;
        self.cells[cell_index].collapse(&mut rand::thread_rng(), &self.tiles);

        let mut queue = VecDeque::new();
        queue.push_back((min_x, min_y));

        let neighbors = [(0, -1), (0, 1), (-1, 0), (1, 0)];

        while let Some((x, y)) = queue.pop_front() {
            let index = y * self.width + x;
            let options = self.cells[index].options.clone();

            for (dx, dy) in neighbors {
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;

                if nx >= 0 && ny >= 0 && nx < self.width as i32 && ny < self.height as i32 {
                    let nindex = ny as usize * self.width + nx as usize;
                    let neighbor = &mut self.cells[nindex];
                    if neighbor.collapsed {
                        continue;
                    }

                    let before = neighbor.options.len();
                    neighbor.options.retain(|&neighbor_option| {
                        options.iter().any(|&opt| self.tiles[opt].fits(&self.tiles[neighbor_option], dx, dy))
                    });

                    if neighbor.options.is_empty() {
                        println!("⚠️ Contradiction: No options left at ({}, {})", nx, ny);
                    }

                    if neighbor.options.len() < before {
                        queue.push_back((nx as usize, ny as usize));
                        neighbor.update_color(&self.tiles);
                    }
                }
            }
        }
    }
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title(TITLE)
        .build();

    //let image = Image::city();
    let image = load_image_from_png("assets/floor.png").unwrap();
    let mut grid = Grid::new(40, 40, &image);

    // HACK
    for cell in grid.cells.iter_mut() {
        cell.update_color(&grid.tiles);
    }

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::DARKGOLDENROD);

        grid.update();
        grid.draw(&mut d);
    }
}
