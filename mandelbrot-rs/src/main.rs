use std::path::Path;

use anyhow::{Context, Result};
use clap::Parser;
use image::ImageBuffer;
use mandelbrot_rs::config::*;
use mandelbrot_rs::opts::Cli;

fn main() -> Result<()> {
    let args = Cli::parse();
    let hue_array = args.get_hue_array()?;
    let (width, height) = args.resolution.to_dimensions();
    let palette = Configuration::get_config()?.get_palette("electric".into())?;
    let img = ImageBuffer::from_fn(width as u32, height as u32, |x, y| {
        let &frac = hue_array.get(x as usize).unwrap().get(y as usize).unwrap();
        palette.value(frac)
    });
    let path = Path::new("out/").join(&args.out_file);
    img.save(path).context("problem saving image")?;
    Ok(())
}
