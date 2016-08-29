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
use std::fs::create_dir_all;


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

#[derive(Eq, PartialEq, Copy, Clone)]
struct Color {
    r: u8,
    g: u8,
    b: u8
}

pub fn generate_image(size_x: u32, size_y: u32) {
    let mut random_colors = random_colors(size_x * size_y);
    place_pixels(&mut random_colors, size_x, size_y);
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
    // TODO: check tuple
    rand::thread_rng().shuffle(colors.as_mut_slices().0);
    colors
}

fn place_pixels(colors: &mut VecDeque<Color>, size_x: u32, size_y: u32) {
    let time_string = format!("{}", time::now().to_timespec().sec);
    create_dir_all(format!("output/{}", time_string)).unwrap();

    let mut pixels = vec![];
    // Free point with at least one occupied neighbour
    let mut active_free_points : Vec<(Point, Color)> = vec![];

    add_pixel(Point {x: size_x / 2, y: size_y / 2}, colors.pop_front().unwrap(), &mut pixels, &mut active_free_points, size_x, size_y);
    create_image(&pixels, size_x, size_y, &time_string, "img0");

    let mut color_distance_threshold = 2;
    let mut colors_counter = 0;
    while colors.len() > 0 {
        let color = colors.pop_front().unwrap();
        let (best_point, best_point_average_neighbour_color) = *(active_free_points.iter().min_by_key(|&&(_,c)| color_distance(color, c)).unwrap());
        if color_distance(color, best_point_average_neighbour_color) <= color_distance_threshold {
            add_pixel(best_point, color, &mut pixels, &mut active_free_points, size_x, size_y);
            colors_counter = 0;
            if (pixels.len() - 1) % 10 == 0 {
                println!("{}, {}", colors.len(), active_free_points.len() - 1);
                create_image(&pixels, size_x, size_y, &time_string, &format!("img{}",  pixels.len()));
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
    create_image(&pixels, size_x, size_y, &time_string, "!final");
}

fn add_pixel(point: Point, color: Color, pixels: &mut Vec<PixelData>, active_free_points: &mut Vec<(Point, Color)>, size_x: u32, size_y: u32) {
    let new_pixel = PixelData {point: point, color: color};
    pixels.push(new_pixel);
    active_free_points.retain(|&(p,_)| p != point);

    let mut new_active_free_points = free_neighbours(point, pixels, size_x, size_y).iter()
        .filter(|&&np| !active_free_points.iter().any(|&(p,_)| p == np))
        .map(|&p| (p, average_neighbour_color(p, pixels, size_x, size_y)))
        .collect::<Vec<_>>();
    active_free_points.append(&mut new_active_free_points);
}

fn color_distance(color1: Color, color2: Color) -> u32 {
    ((color2.r as i32 - color1.r as i32).pow(2) + (color2.g as i32 - color1.g as i32).pow(2) + (color2.b as i32 - color1.b as i32).pow(2)) as u32
}

fn neighbours(point: Point, size_x: u32, size_y: u32) -> Vec<Point> {
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
    neighbours
}

fn free_neighbours(point: Point, pixels: &Vec<PixelData>, size_x: u32, size_y: u32) -> Vec<Point> {
    let occupied_points = pixels.into_iter().map(|p| p.point).collect::<Vec<_>>();
    neighbours(point, size_x, size_y).into_iter()
        .filter(|p| !occupied_points.contains(p))
        .collect::<Vec<_>>()
}

fn occupied_neighbours(point: Point, pixels: &Vec<PixelData>, size_x: u32, size_y: u32) -> Vec<Point> {
    let occupied_points = pixels.into_iter().map(|p| p.point).collect::<Vec<_>>();
    neighbours(point, size_x, size_y).into_iter()
        .filter(|p| occupied_points.contains(p))
        .collect::<Vec<_>>()
}

fn average_neighbour_color(point: Point, pixels: &Vec<PixelData>, size_x: u32, size_y: u32) -> Color {
    let occupied_neighbours = occupied_neighbours(point, &pixels, size_x, size_y);
    let neighbour_pixels = pixels.into_iter().filter(|p| occupied_neighbours.contains(&p.point)).collect::<Vec<_>>();

    let neighbour_count = neighbour_pixels.len() as u8;
    let (r,g,b) = neighbour_pixels.into_iter().fold((0,0,0), |(r,g,b), &p| (r + p.color.r / neighbour_count, g + p.color.g / neighbour_count, b+p.color.b / neighbour_count));
    Color {r: r, g: g, b: b}
}

fn create_image(pixels: &Vec<PixelData>, size_x: u32, size_y: u32, directory: &str, filename: &str) {
    let mut img = RgbImage::new(size_x, size_y);
    for p in pixels {
        img.put_pixel(p.point.x, p.point.y, Rgb::from_channels(p.color.r, p.color.g, p.color.b, 0));
    }
    let _ = img.save(format!("output/{}/{}.png", directory, filename));
}
