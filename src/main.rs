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
const SIZE_X: u32 = 32;
const SIZE_Y: u32 = 16;

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
    let mut random_colors = random_colors();
    place_pixels(&mut random_colors)
}

fn place_pixels(colors: &mut VecDeque<Color>) -> Vec<PixelData> {
    let mut pixels = vec![];
    // Pixels with at least one free neighbour
    let mut active_pixels = vec![];
    // Free point with at least one occupied neighbour
    let mut active_free_points = vec![];

    add_pixel(Point {x: SIZE_X / 2, y: SIZE_Y / 2}, colors.pop_front().unwrap(), &mut pixels, &mut active_pixels, &mut active_free_points);

    let mut color_distance_threshold = 2;
    while colors.len() > 0 {
        println!("{}, {}, {}", colors.len(), active_pixels.len(), active_free_points.len());
        let c = colors.pop_front().unwrap();
        let best_point = *(active_free_points.iter().min_by_key(|&&p| color_distance(c, average_neighbour_color(p, &pixels))).unwrap());
        if color_distance(c, average_neighbour_color(best_point, &pixels)) <= color_distance_threshold {
            add_pixel(best_point, c, &mut pixels, &mut active_pixels, &mut active_free_points);
            color_distance_threshold /= 4;
        } else {
            colors.push_back(c);
            color_distance_threshold *= 2;
        }
    }
    pixels
}

fn add_pixel(point: Point, color: Color, pixels: &mut Vec<PixelData>, active_pixels: &mut Vec<PixelData>, active_free_points: &mut Vec<Point>) {
    let new_pixel = PixelData {point: point, color: color};
    pixels.push(new_pixel);
    active_pixels.push(new_pixel);
    // TODO: change free_neighbours argument to active_pixels
    active_pixels.retain(|&p| free_neighbours(p.point, pixels).len() > 0);
    active_free_points.retain(|&p| p != point);
    active_free_points.append(&mut free_neighbours(point, pixels));
    active_free_points.sort();
    active_free_points.dedup();
}

fn color_distance(color1: Color, color2: Color) -> u32 {
    ((color2.r as i32 - color1.r as i32).pow(2) + (color2.g as i32 - color1.g as i32).pow(2) + (color2.b as i32 - color1.b as i32).pow(2)) as u32
}

fn neighbours(point: Point) -> Vec<Point> {
    let mut neighbours = vec![];
    for i in -1..2 {
        for j in -1..2 {
            let new_x = point.x as i32 + i;
            let new_y = point.y as i32 + j;
            if new_x >= 0 && new_x < SIZE_X as i32 && new_y >= 0 && new_y < SIZE_Y as i32 {
                let new_point = Point {x: new_x as u32, y: new_y as u32};
                if i != 0 || j != 0 {
                    neighbours.push(new_point);
                }
            }
        }
    }
    neighbours
}

fn free_neighbours(point: Point, active_pixels: &Vec<PixelData>) -> Vec<Point> {
    let occupied_points = active_pixels.into_iter().map(|p| p.point).collect::<Vec<_>>();
    neighbours(point).into_iter()
        .filter(|p| !occupied_points.contains(p))
        .collect::<Vec<_>>()
}

fn occupied_neighbours(point: Point, active_pixels: &Vec<PixelData>) -> Vec<Point> {
    let occupied_points = active_pixels.into_iter().map(|p| p.point).collect::<Vec<_>>();
    neighbours(point).into_iter()
        .filter(|p| occupied_points.contains(p))
        .collect::<Vec<_>>()
}

fn average_neighbour_color(point: Point, pixels: &Vec<PixelData>) -> Color {
    let occupied_neighbours = occupied_neighbours(point, &pixels);
    let neighbour_pixels = pixels.into_iter().filter(|p| occupied_neighbours.contains(&p.point)).collect::<Vec<_>>();

    let neighbour_count = neighbour_pixels.len() as u8;
    let (r,g,b) = neighbour_pixels.into_iter().fold((0,0,0), |(r,g,b), &p| (r + p.color.r / neighbour_count, g + p.color.g / neighbour_count, b+p.color.b / neighbour_count));
    Color {r: r, g: g, b: b}
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
