use anyhow::Context;
use anyhow::Result;
use image::{ImageBuffer, Rgb};
use mandelbrot_rs::mandelbrot::generate_escape_counts;
use mandelbrot_rs::mandelbrot::generate_hist_counts;
use mandelbrot_rs::opts::parse_args;
use mandelbrot_rs::palette::ColorPalette;

fn main() -> Result<()> {
    let config = parse_args().context("problem parsing config")?;
    eprintln!("config = {:?}", config);
    let palette = ColorPalette::new(vec![
        (0., Rgb([0, 18, 25])),
        (0.1, Rgb([0, 18, 25])),
        (0.5, Rgb([20, 33, 61])),
        (0.8, Rgb([252, 163, 17])),
        (0.9, Rgb([229, 229, 229])),
        (0.95, Rgb([255, 255, 255])),
        (1., Rgb([0, 0, 0])),
    ])
    .unwrap();
    let escape_counts = generate_escape_counts(
        &config.x_range,
        &config.y_range,
        config.width,
        config.height,
    );
    let hist_counts = generate_hist_counts(&escape_counts, 2000, config.width * config.height);
    let img = ImageBuffer::from_fn(config.width as u32, config.height as u32, |x, y| {
        let &frac = hist_counts
            .get(x as usize)
            .unwrap()
            .get(y as usize)
            .unwrap();
        palette.value(frac)
    });
    img.save(config.out_file)?;
    Ok(())
}
