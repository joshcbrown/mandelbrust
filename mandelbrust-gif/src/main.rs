use image::{imageops::resize, DynamicImage, GenericImageView, ImageBuffer};
use kdam::tqdm;
use mandelbrust_cli::{
    config::Configuration,
    opts::{Cli, PlottingAlgorithm},
};

fn main() {
    let conf: Configuration = confy::load("mandelbrust", "config").unwrap();
    let place = conf.get_named_point("circle").unwrap();
    let mut zoom = 8.;
    for i in tqdm!(0..30) {
        let conf = Cli {
            out_file: format!("out/{}.png", i),
            max_iters: 10000,
            bailout: 1e9,
            resolution: mandelbrust_cli::opts::Resolution::High,
            palette: "warm".into(),
            palette_repeats: 50,
            algorithm: PlottingAlgorithm::Smooth,
            command: mandelbrust_cli::opts::Commands::Centre {
                x: place.point.re,
                y: place.point.im,
                zoom: zoom as usize,
            },
        };

        let hue_array = conf.get_hue_array().unwrap();
        println!("done calculating");
        let (width, height) = conf.resolution.to_dimensions();
        let palette = conf.get_palette().unwrap();
        let img = ImageBuffer::from_fn(width as u32, height as u32, |x, y| {
            let frac = hue_array[x as usize][y as usize];
            palette.value(frac)
        });
        let mut dyn_image = DynamicImage::ImageRgb8(img);
        for j in 0..27 {
            dyn_image = crop_image(dyn_image);
            let scaled = resize(
                &dyn_image,
                960,
                540,
                image::imageops::FilterType::CatmullRom,
            );
            scaled.save(format!("out/{}.png", i * 27 + j)).unwrap();
        }
        zoom *= 26.7027;
    }
}

fn crop_image(mut image: DynamicImage) -> DynamicImage {
    let (width, height) = image.dimensions();
    let x_start = width / 20; // Start 5% in from the left edge
    let y_start = height / 20; // Start 5% in from the top edge

    image.crop(
        x_start,
        y_start,
        (width as f64 * 0.90) as u32,
        (height as f64 * 0.90) as u32,
    )
}
