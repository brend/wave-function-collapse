use raylib::prelude::*;

#[derive(Debug, Clone)]
pub struct Image {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<Color>,
}

impl Image {
    pub fn new(width: usize, height: usize) -> Self {
        let pixels = vec![Color::BLACK; width * height];
        Self {
            width,
            height,
            pixels,
        }
    }

    pub fn city() -> Self {
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

    pub fn slices(&self, slice_width: usize, slice_height: usize) -> Vec<Image> {
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

    pub fn draw(&self, d: &mut RaylibDrawHandle, x: i32, y: i32, cell_size: i32) {
        for sy in 0..self.height {
            for sx in 0..self.width {
                let color = self.pixels[sy * self.width + sx];
                d.draw_rectangle(
                    (x + sx as i32) * cell_size, 
                    (y + sy as i32) * cell_size, 
                    cell_size, cell_size, color);
            }
        }
    }

    pub fn fits(&self, other: &Image, offset: (i32, i32)) -> bool {
        match offset {
            (0, -1) => self.fits_top(other),
            (0, 1) => self.fits_bottom(other),
            (-1, 0) => self.fits_left(other),
            (1, 0) => self.fits_right(other),
            _ => {
                println!("Invalid direction: ({}, {})", offset.0, offset.1);
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

use std::path::Path;

pub fn load_image_from_png<P: AsRef<Path>>(path: P) -> Result<Image, String> {
    // Load the image using Raylib's Image struct (not tied to a window)
    let rl_image = raylib::prelude::Image::load_image(path.as_ref().to_str().unwrap())
        .map_err(|e| format!("Failed to load image: {:?}", e))?;

    let width = rl_image.width() as usize;
    let height = rl_image.height() as usize;

    // Get raw pixel data as Vec<Color>
    let pixels = rl_image.get_image_data().to_vec();

    Ok(Image {
        width,
        height,
        pixels,
    })
}