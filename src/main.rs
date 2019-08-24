mod color_functions;
mod image_functions;

fn main() {
    let size_x: u32 = 256;
    let size_y: u32 = 128;

    println!("Generating {}x{} image", size_x, size_y);
    let directory = format!("output/{}_{}x{}", chrono::Local::now().timestamp(), size_x, size_y);
    let duration = chrono::Duration::span(|| image_functions::generate_image(size_x, size_y, &directory));
    create_info_file(&directory, size_x, size_y, duration);
    println!("Complete");
}

fn create_info_file(directory: &str, size_x: u32, size_y: u32, duration: chrono::Duration) {
    let color_step = (256f32 / ((size_x * size_y) as f32).cbrt()) as u16;
    let contents = format!("Dimensions: {}x{}\nColor step: {}\nTime to complete: {}.{} seconds",
                           size_x, size_y, color_step, duration.num_seconds(), duration.num_milliseconds());

    std::fs::write(format!("{}/0info.txt", directory), contents).unwrap();
}
