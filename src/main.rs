// generate multiple images to make animation, slightly different start point and same color order

extern crate colors;
use colors::image_functions::generate_image;

fn main() {
    let size_x: u32 = 64;
    let size_y: u32 = 64;
    println!("Generating {}x{} image", size_x, size_y);
    generate_image(size_x, size_y);
    println!("Complete");
}
