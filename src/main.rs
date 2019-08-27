mod color_functions;
mod image_functions;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "colors", about = "Generates colourful images")]
struct Opt {
    #[structopt(help = "The image width.")]
    width: u32,
    #[structopt(help = "The image height.")]
    height: u32,
    #[structopt(default_value = "0.5", help = "The horizontal starting point.")]
    horizontal_start_point: f32,
    #[structopt(default_value = "0.5", help = "The vertical starting point.")]
    vertical_start_point: f32,
    #[structopt(
        short = "o",
        long = "output",
        default_value = "ColorsOutput",
        help = "The output directory."
    )]
    output_directory: String,
}

fn get_progress_bar(length: u64) -> indicatif::ProgressBar {
    let progress_bar = indicatif::ProgressBar::new(length);
    progress_bar.set_style(
        indicatif::ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {msg} {wide_bar:0.cyan/blue} {pos}/{len} {eta}")
            .progress_chars("#>-"),
    );
    progress_bar
}

fn main() {
    let opt: Opt = Opt::from_args();
    let directory = format!(
        "output/{}_{}x{}",
        chrono::Local::now().timestamp(),
        opt.width,
        opt.height
    );
    let progress_bar = get_progress_bar((opt.width * opt.height).into());
    progress_bar.set_message(&format!("Creating {}x{} image", opt.width, opt.height));

    let start_time = std::time::Instant::now();
    // TODO: Update progress bar
    image_functions::generate_image(opt.width, opt.height, &directory, &progress_bar);
    let duration = start_time.elapsed();

    progress_bar.finish_with_message(&format!("Created {}x{} image", opt.width, opt.height));

    create_info_file(&directory, opt.width, opt.height, &duration);
}

fn create_info_file(directory: &str, size_x: u32, size_y: u32, duration: &std::time::Duration) {
    let color_step = (256f32 / ((size_x * size_y) as f32).cbrt()) as u16;
    let contents = format!(
        "Dimensions: {}x{}\nColor step: {}\nTime to complete: {}.{} seconds",
        size_x,
        size_y,
        color_step,
        duration.as_secs(),
        duration.subsec_millis()
    );

    std::fs::write(format!("{}/0info.txt", directory), contents).unwrap();
}
