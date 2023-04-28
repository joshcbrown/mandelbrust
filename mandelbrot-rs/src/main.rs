use anyhow::Result;
use image::{ImageBuffer, Rgb};
use mandelbrot_rs::mandelbrot::generate_escape_counts;
use mandelbrot_rs::mandelbrot::Complex;
use mandelbrot_rs::opts::{parse_args, Interval};
use mandelbrot_rs::palette::ColorPalette;

fn main() -> Result<()> {
    let config = parse_args();
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
    let (width, height) = (1920, 1080);
    let escape_counts = generate_escape_counts(&config.x_range, &config.y_range, width, height);
    eprintln!(
        "{}, {}",
        escape_counts.len(),
        escape_counts.get(0).unwrap().len()
    );
    let img = ImageBuffer::from_fn(width, height, |x, y| {
        let &escape_count = escape_counts
            .get(x as usize)
            .unwrap()
            .get(y as usize)
            .unwrap();
        let frac = escape_count as f64 / 2000.;
        palette.value(frac)
    });
    img.save(config.out_file)?;
    Ok(())
}
