use anyhow::{Context, Result};
use clap::Parser;
use image::ImageBuffer;
use mandelbrust_cli::opts::Cli;

fn main() -> Result<()> {
    let args = Cli::parse();
    let hue_array = args.get_hue_array()?;
    let (width, height) = args.resolution.to_dimensions();
    let palette = args.get_palette()?;
    let img = ImageBuffer::from_fn(width as u32, height as u32, |x, y| {
        let frac = hue_array[x as usize][y as usize];
        palette.value(frac)
    });
    img.save(&args.out_file).context("problem saving image")?;
    Ok(())
}
