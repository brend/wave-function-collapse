use raylib::prelude::*;

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
        for y in 0..(self.height - slice_height + 1) {
            for x in 0..(self.width - slice_width + 1) {
                let mut slice = Image::new(slice_width, slice_height);
                for sy in 0..slice_height {
                    for sx in 0..slice_width {
                        slice.pixels[sy * slice_width + sx] = self.pixels[(y + sy) * self.width + x + sx];
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
}

#[derive(Debug, Clone)]
struct Cell {
    options: Vec<Image>,
}

impl Cell {
    fn new(options: &Vec<Image>) -> Self {
        Self {
            options: options.clone(),
        }
    }

    fn average_color(&self) -> Color {
        let mut r: u32 = 0;
        let mut g: u32 = 0;
        let mut b: u32 = 0;
        for option in &self.options {
            let mx = option.width / 2;
            let my = option.height / 2;
            let color = option.pixels[my * option.width + mx];
            r += color.r as u32;
            g += color.g as u32;
            b += color.b as u32;
        }
        let n = self.options.len() as u32;
        Color::new((r / n) as u8, (g / n) as u8, (b / n) as u8, 255)
    }

    fn draw(&self, d: &mut RaylibDrawHandle, x: i32, y: i32) {
        d.draw_rectangle(x * CELL_SIZE, y * CELL_SIZE, CELL_SIZE, CELL_SIZE, self.average_color());
        d.draw_rectangle_lines(x * CELL_SIZE, y * CELL_SIZE, CELL_SIZE, CELL_SIZE, Color::LIGHTGRAY);
    }
}

struct Grid {
    width: usize,
    height: usize,
    cells: Vec<Cell>,
}

impl Grid {
    fn new(width: usize, height: usize, options: &Vec<Image>) -> Self {
        let cells = vec![Cell::new(options); width * height];
        Self {
            width,
            height,
            cells,
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
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title(TITLE)
        .build();

    let image = Image::city();
    let slices = image.slices(3, 3);
    let mut grid = Grid::new(40, 40, &slices);

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::DARKGOLDENROD);

        grid.draw(&mut d);
    }
}
