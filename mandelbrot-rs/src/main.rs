use anyhow::{Context, Result};
use clap::Parser;
use image::{ImageBuffer, Rgb};
use mandelbrot_rs::opts::{get_hue_array, Cli};
use mandelbrot_rs::palette::ColorPalette;

fn main() -> Result<()> {
    let args = Cli::parse();
    let (width, height) = args.resolution.to_dimensions();
    let hue_array = get_hue_array(&args);
    let palette = ColorPalette::new(vec![
        (0., Rgb([0, 18, 25])),
        (0.1, Rgb([0, 18, 25])),
        (0.5, Rgb([20, 33, 61])),
        (0.8, Rgb([252, 163, 17])),
        (0.9, Rgb([229, 229, 229])),
        (0.99, Rgb([255, 255, 255])),
        (1., Rgb([0, 0, 0])),
    ])
    .unwrap();
    let img = ImageBuffer::from_fn(width as u32, height as u32, |x, y| {
        let &frac = hue_array.get(x as usize).unwrap().get(y as usize).unwrap();
        palette.value(frac)
    });
    img.save(args.out_file).context("problem saving image")?;
    Ok(())
}
