// generate multiple images to make animation, slightly different start point and same color order

#![feature(step_by)]
extern crate image;
extern crate rand;

use image::{
    RgbImage,
    Pixel,
    Rgb
};
use rand::Rng;

struct PixelData {
    point: Point,
    color: Color
}

struct Point {
    x: u32,
    y: u32
}

struct Color {
    r: u8,
    g: u8,
    b: u8
}

fn main() {
    // TODO: pass as arguments
    let size_x = 128;
    let size_y = 64;

    println!("Generating {}x{} image", size_x, size_y);
    generate_image(size_x, size_y);
    println!("Complete");
}

fn generate_image(size_x: u32, size_y: u32) {
    let pixels = generate_pixels(size_x, size_y);
    create_image(size_x, size_y, pixels);
}

fn generate_pixels(size_x: u32, size_y: u32) -> Vec<PixelData> {
    let color_step =  (256f32 / ((size_x*size_y) as f32).cbrt()) as u8;
    let mut random_colors = random_colors(color_step);

    (0..100).map(|p| PixelData {point: Point {x: p % size_x, y: p % size_y}, color: Color {r: p as u8, g: p as u8, b:p as u8}}).collect::<Vec<_>>()
}

fn random_colors(color_step: u8) -> Vec<Color> {
    let mut colors = Vec::with_capacity(256 * 256 * 256 / color_step as usize);
    for r in (0..256).step_by(color_step as u32) {
        for g in (0..256).step_by(color_step as u32) {
            for b in (0..256).step_by(color_step as u32) {
                colors.push(Color {r: r as u8, g: g as u8, b: b as u8});
            }
        }
    }
    rand::thread_rng().shuffle(&mut colors);
    colors
}

fn create_image(size_x: u32, size_y: u32, pixels: Vec<PixelData>) {
    let mut img = RgbImage::new(size_x, size_y);
    for p in pixels {
        img.put_pixel(p.point.x, p.point.y, Rgb::from_channels(p.color.r, p.color.g, p.color.b, 0));
    }
    let _ = img.save("output/img.png");
}
