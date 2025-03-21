use rand::Rng;
use raylib::prelude::*;
use std::collections::VecDeque;

const SCREEN_WIDTH: i32 = 400;
const SCREEN_HEIGHT: i32 = 400;

const CELL_SIZE: i32 = 10;

const TITLE: &str = "Wave Function Collapse";

#[derive(Debug, Clone)]
struct Image {
    width: usize,
    height: usize,
    pixels: Vec<Color>,
}

impl Image {
    fn new(width: usize, height: usize) -> Self {
        let pixels = vec![Color::BLACK; width * height];
        Self {
            width,
            height,
            pixels,
        }
    }

    fn city() -> Self {
        let mut image = Self::new(9, 9);
        let pixels = &mut image.pixels;
        for i in 0..9 {
            pixels[i] = Color::WHITE;
            pixels[i + 9 * 8] = Color::WHITE;
            pixels[i * 9] = Color::WHITE;
            pixels[i * 9 + 8] = Color::WHITE;
        }
        for i in 1..8 {
            pixels[i + 9] = Color::BLACK;
            pixels[i + 9 * 7] = Color::BLACK;
        }
        for i in 2..7 {
            for j in 2..7 {
                pixels[i + j * 9] = Color::MAROON;
            }
        }

        image
    }

    fn slices(&self, slice_width: usize, slice_height: usize) -> Vec<Image> {
        let mut slices = Vec::new();
        for y in 0..self.height {
            for x in 0..self.width {
                let mut slice = Image::new(slice_width, slice_height);
                for sy in 0..slice_height {
                    for sx in 0..slice_width {
                        let px = (x + sx) % self.width;
                        let py = (y + sy) % self.height;
                        slice.pixels[sy * slice_width + sx] = self.pixels[py * self.width + px];
                    }
                }
                slices.push(slice);
            }
        }
        slices
    }

    fn draw(&self, d: &mut RaylibDrawHandle, x: i32, y: i32) {
        for sy in 0..self.height {
            for sx in 0..self.width {
                let color = self.pixels[sy * self.width + sx];
                d.draw_rectangle(x * CELL_SIZE + sx as i32 * CELL_SIZE, y * CELL_SIZE + sy as i32 * CELL_SIZE, CELL_SIZE, CELL_SIZE, color);
            }
        }
    }

    fn fits(&self, other: &Image, dx: i32, dy: i32) -> bool {
        match (dx, dy) {
            (0, -1) => self.fits_top(other),
            (0, 1) => self.fits_bottom(other),
            (-1, 0) => self.fits_left(other),
            (1, 0) => self.fits_right(other),
            _ => {
                println!("Invalid direction: ({}, {})", dx, dy);
                false
            }
        }
    }

    fn fits_top(&self, other: &Image) -> bool {
        // self fits other if the top two rows of self match the bottom two rows of other
        for y in 0..2 {
            for x in 0..self.width {
                if self.pixels[x + y * self.width] != other.pixels[x + (other.height - 2 + y) * other.width] {
                    return false;
                }
            }
        }
        true
    }

    fn fits_bottom(&self, other: &Image) -> bool {
        // self fits other if the bottom two rows of self match the top two rows of other
        for y in 0..2 {
            for x in 0..self.width {
                if self.pixels[x + (self.height - 2 + y) * self.width] != other.pixels[x + y * other.width] {
                    return false;
                }
            }
        }
        true
    }

    fn fits_left(&self, other: &Image) -> bool {
        // self fits other if the left two columns of self match the right two columns of other
        for y in 0..self.height {
            for x in 0..2 {
                if self.pixels[x + y * self.width] != other.pixels[(other.width - 2 + x) + y * other.width] {
                    return false;
                }
            }
        }
        true
    }

    fn fits_right(&self, other: &Image) -> bool {
        // self fits other if the right two columns of self match the left two columns of other
        for y in 0..self.height {
            for x in 0..2 {
                if self.pixels[(self.width - 2 + x) + y * self.width] != other.pixels[x + y * other.width] {
                    return false;
                }
            }
        }
        true
    }
}

#[derive(Debug, Clone)]
struct Cell {
    options: Vec<usize>,
    collapsed: bool,
}

impl Cell {
    fn new(option_count: usize) -> Self {
        Self {
            options: (0..option_count).collect(),
            collapsed: false,
        }
    }

    fn average_color(&self, tiles: &Vec<Image>) -> Color {
        let n = self.options.len() as u32;

        if n == 0 {
            return Color::MAGENTA;
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
        
        Color::new((r / n) as u8, (g / n) as u8, (b / n) as u8, 255)
    }

    fn draw(&self, d: &mut RaylibDrawHandle, x: i32, y: i32, tiles: &Vec<Image>) {
        d.draw_rectangle(x * CELL_SIZE, y * CELL_SIZE, CELL_SIZE, CELL_SIZE, self.average_color(tiles));
    }

    fn collapse(&mut self, rng: &mut rand::rngs::ThreadRng) {
        let index = rng.gen_range(0..self.options.len());
        self.options = vec![self.options[index]];
        self.collapsed = true;
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
                cell.draw(d, x as i32, y as i32, &self.tiles);
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
        self.cells[cell_index].collapse(&mut rand::thread_rng());

        let mut queue = VecDeque::new();
        queue.push_back((min_x, min_y));

        let neighbors = [(0, -1), (0, 1), (-1, 0), (1, 0)];

        println!("xxx");
        while let Some((x, y)) = queue.pop_front() {
            println!("Collapsing ({}, {})", x, y);
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

    let image = Image::city();
    let mut grid = Grid::new(40, 40, &image);

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::DARKGOLDENROD);

        grid.update();
        grid.draw(&mut d);

        //std::thread::sleep(std::time::Duration::from_millis(1000));
    }
}
