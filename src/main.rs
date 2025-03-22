use wfc::{grid::Grid, image::*};

const SCREEN_WIDTH: i32 = 400;
const SCREEN_HEIGHT: i32 = 400;

const TITLE: &str = "Wave Function Collapse";

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title(TITLE)
        .build();

    // let image = load_image_from_png("assets/floor.png").unwrap();
    let image = Image::city();
    let mut grid = Grid::new(40, 40, &image);

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        grid.update();
        grid.draw(&mut d);
    }
}
