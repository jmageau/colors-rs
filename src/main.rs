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
use std::collections::VecDeque;

#[derive(Copy, Clone)]
struct PixelData {
    point: Point,
    color: Color
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
struct Point {
    x: u32,
    y: u32
}

#[derive(Copy, Clone)]
struct Color {
    r: u8,
    g: u8,
    b: u8
}

// TODO: pass as arguments
const SIZE_X: u32 = 256;
const SIZE_Y: u32 = 128;

fn main() {
    println!("Generating {}x{} image", SIZE_X, SIZE_Y);
    generate_image();
    println!("Complete");
}

fn generate_image() {
    let pixels = generate_pixels();
    create_image(pixels);
}

fn generate_pixels() -> Vec<PixelData> {
    let random_colors = random_colors();
    let active_points = vec![];
    place_pixels(random_colors, active_points)
}

fn place_pixels(mut colors: VecDeque<Color>, mut active_points: Vec<Point>) -> Vec<PixelData> {
    // TODO: change pixels from vector to ordered data structure
    let mut pixels = vec![];
    add_pixel(Point {x: SIZE_X / 2, y: SIZE_Y / 2}, colors.pop_front().unwrap(), &mut pixels, &mut active_points);
    while colors.len() > 0 {
        // TODO
    }
    pixels
}

fn add_pixel(point: Point, color: Color, pixels: &mut Vec<PixelData>, active_points: &mut Vec<Point>) {
    pixels.push(PixelData {point: point, color: color});
    active_points.retain(|&p| p != point);
    active_points.append(&mut free_neighbours(point, pixels));
    // TODO: instead of removing duplicates, use data structure with no duplicates
    active_points.sort();
    active_points.dedup();
}

fn free_neighbours(point: Point, pixels: &Vec<PixelData>) -> Vec<Point> {
    let mut neighbours = vec![];
    let occupied_points = pixels.into_iter()
        .filter(|&p| abs_sub(p.point.x, point.x) <= 1 && abs_sub(p.point.y, point.y) <= 1)
        .map(|p| p.point)
        .collect::<Vec<_>>();
    for i in -1..2 {
        for j in -1..2 {
            let new_x = point.x as i32 + i;
            let new_y = point.y as i32 + j;
            if new_x >= 0 && new_x < SIZE_X as i32 && new_y >= 0 && new_y < SIZE_Y as i32 {
                let new_point = Point {x: new_x as u32, y: new_y as u32};
                if (i != 0 || j != 0) && !occupied_points.contains(&new_point) {
                    neighbours.push(new_point);
                }
            }
        }
    }
    neighbours
}

fn abs_sub(a: u32, b: u32) -> u32 {
    if a > b {
        return a - b;
    }
    b - a
}

fn random_colors() -> VecDeque<Color> {
    let color_step =  (256f32 / ((SIZE_X*SIZE_Y) as f32).cbrt()) as u8;
    let mut colors = VecDeque::with_capacity(256 * 256 * 256 / color_step as usize);
    for r in (0..256).step_by(color_step as u32) {
        for g in (0..256).step_by(color_step as u32) {
            for b in (0..256).step_by(color_step as u32) {
                colors.push_back(Color {r: r as u8, g: g as u8, b: b as u8});
            }
        }
    }
    // TODO: check tuple
    rand::thread_rng().shuffle(colors.as_mut_slices().0);
    colors
}

fn create_image(pixels: Vec<PixelData>) {
    let mut img = RgbImage::new(SIZE_X, SIZE_Y);
    for p in pixels {
        img.put_pixel(p.point.x, p.point.y, Rgb::from_channels(p.color.r, p.color.g, p.color.b, 0));
    }
    let _ = img.save("output/img.png");
}
