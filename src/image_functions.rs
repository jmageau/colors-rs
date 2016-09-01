extern crate time;
extern crate image;
extern crate rand;

use self::image::{
    RgbImage,
    Pixel,
    Rgb
};
use self::rand::Rng;
use std::collections::VecDeque;
use std::collections::HashMap;
use std::fs::create_dir_all;


#[derive(Clone, Eq, PartialEq, Hash)]
struct Point {
    x: u32,
    y: u32
}

#[derive(Clone, Eq, PartialEq)]
struct Color {
    r: u8,
    g: u8,
    b: u8
}


pub fn generate_image(size_x: u32, size_y: u32, directory: &str) {
    let mut random_colors = random_colors(size_x * size_y);
    place_pixels(&mut random_colors, size_x, size_y, directory);
}

fn random_colors(count: u32) -> VecDeque<Color> {
    let color_step =  (256f32 / (count as f32).cbrt()) as u16;
    let mut colors = VecDeque::with_capacity(256 * 256 * 256 / color_step as usize);
    for r in (0..256).step_by(color_step) {
        for g in (0..256).step_by(color_step) {
            for b in (0..256).step_by(color_step) {
                colors.push_back(Color {r: (r + color_step) as u8, g: (g + color_step) as u8, b: (b + color_step) as u8});
            }
        }
    }
    rand::thread_rng().shuffle(colors.as_mut_slices().0);
    colors
}

fn place_pixels(colors: &mut VecDeque<Color>, size_x: u32, size_y: u32, directory: &str) {
    create_dir_all(&directory).unwrap();
    let image_interval = size_x * size_y / 512;

    let mut pixels = HashMap::with_capacity((size_x*size_y) as usize);
    // Pixels with at least one free neighbour
    let mut active_pixels = HashMap::with_capacity(((size_x+size_y)*2) as usize);

    add_pixel(Point {x: size_x / 2, y: size_y / 2}, colors.pop_front().unwrap(), &mut pixels, &mut active_pixels, size_x, size_y);
    create_image(&pixels, size_x, size_y, &directory, "img0");

    let mut color_distance_threshold = 2;
    let mut colors_counter = 0;
    let mut i = 1;
    while colors.len() > 0 {
        let color = colors.pop_front().unwrap();
        let best_point = get_best_point(&color, &active_pixels, color_distance_threshold);
        if best_point.is_some() {
            let free_neighbours = free_neighbours(&best_point.unwrap(), &pixels, size_x, size_y);
            let point = rand::thread_rng().choose(&free_neighbours).unwrap().clone();
            add_pixel(point, color, &mut pixels, &mut active_pixels, size_x, size_y);
            colors_counter = 0;
            if (pixels.len() as u32 - 1) % image_interval == 0 {
                println!("{}, {}, {}", colors.len(), active_pixels.len() - 1, color_distance_threshold);
                create_image(&pixels, size_x, size_y, &directory, &format!("img{}",  i));
                i += 1;
                color_distance_threshold -= 1;
                if color_distance_threshold < 2 {
                    color_distance_threshold = 2;
                }
            }
        } else {
            colors.push_back(color);
            colors_counter += 1;
            if colors_counter >= colors.len() {
                color_distance_threshold *= 2;
                colors_counter = 0;
            }
        }
    }
    create_image(&pixels, size_x, size_y, &directory, "0final");
}

fn add_pixel(point: Point, color: Color, pixels: &mut HashMap<Point, Color>, active_pixels: &mut HashMap<Point, Color>, size_x: u32, size_y: u32) {
    pixels.insert(point.clone(), color.clone());
    active_pixels.insert(point, color);

    let active_points_to_remove = active_pixels.iter()
        .map(|(p,_)| p.clone())
        .filter(|p| free_neighbours(p, &pixels, size_x, size_y).len() == 0)
        .collect::<Vec<_>>();
    for p in &active_points_to_remove {
        active_pixels.remove(p);
    }
}

fn color_distance(color1: &Color, color2: &Color) -> u32 {
    ((color2.r as i32 - color1.r as i32).pow(2) + (color2.g as i32 - color1.g as i32).pow(2) + (color2.b as i32 - color1.b as i32).pow(2)) as u32
}

fn get_best_point(color: &Color, active_pixels: &HashMap<Point, Color>, color_distance_threshold: u32) -> Option<(Point)> {
    active_pixels.iter().find(|&(_,c)| color_distance(color, c) < color_distance_threshold).map(|(p,_)| p.clone())
}

fn free_neighbours(point: &Point, pixels: &HashMap<Point, Color>, size_x: u32, size_y: u32) -> Vec<Point> {
    let mut neighbours = vec![];
    if point.y > 0 {
        neighbours.push(Point {x: point.x, y: point.y - 1});
    }
    if point.x < size_x - 1 {
        neighbours.push(Point {x: point.x + 1, y: point.y});
    }
    if point.y < size_y - 1 {
        neighbours.push(Point {x: point.x, y: point.y + 1});
    }
    if point.x > 0 {
        neighbours.push(Point {x: point.x - 1, y: point.y});
    }
    neighbours.into_iter().filter(|p| !pixels.contains_key(p)).collect()
}

fn create_image(pixels: &HashMap<Point, Color>, size_x: u32, size_y: u32, directory: &str, filename: &str) {
    let mut img = RgbImage::new(size_x, size_y);
    for (p,c) in pixels {
        img.put_pixel(p.x, p.y, Rgb::from_channels(c.r, c.g, c.b, 0));
    }
    let _ = img.save(format!("{}/{}.png", directory, filename));
}
