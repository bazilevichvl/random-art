use clap::Parser;
use image::{GrayImage, RgbImage};
use random_art::generators::{generate_image, GrayscaleGenerator, RgbGenerator};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about=None)]
struct Config {
    #[clap(
        help = "Maximal depth of the expression tree",
        short,
        long,
        default_value_t = 10
    )]
    depth: usize,
    #[clap(help = "Output image width", short, long, default_value_t = 256)]
    w: u32,
    #[clap(help = "Output image height", short, long, default_value_t = 256)]
    h: u32,
    #[clap(help = "Output file name", short, long, default_value_t=String::from("output.png"))]
    output: String,
    #[clap(
        help = "Produce colored image",
        short,
        long,
        action,
        default_value_t = false
    )]
    colored: bool,
}
fn main() {
    let config = Config::parse();
    println!("{:#?}", config);

    if config.colored {
        let mut image = RgbImage::new(config.w, config.h);
        let gen = RgbGenerator::new(config.depth);

        generate_image(&mut image, gen);
        image.save(config.output).expect("Can't save to a file");
    } else {
        let mut image = GrayImage::new(config.w, config.h);
        let gen = GrayscaleGenerator::new(config.depth);

        generate_image(&mut image, gen);
        image.save(config.output).expect("Can't save to a file");
    }
}
