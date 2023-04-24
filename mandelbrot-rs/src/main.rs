use anyhow::Result;
use image::{ImageBuffer, Rgb};
use mandelbrot_rs::mandelbrot::Complex;
use mandelbrot_rs::opts::{parse_args, Interval};

fn main() -> Result<()> {
    let config = parse_args();
    eprintln!("config = {:?}", config);
    let (width, height) = (1920, 1080);
    let img = ImageBuffer::from_fn(width, height, |x, y| {
        let re = lerp(&config.x_range, x as f64 / width as f64);
        let im = lerp(&config.y_range, y as f64 / height as f64);
        let c = Complex::new(re, im);
        let escape_count = c.escape_count(Complex::id(), 8., 1000);
        let frac = escape_count as f64 / 200. * 256.;
        Rgb([frac as u8, frac as u8, frac as u8])
    });
    img.save(config.out_file)?;
    Ok(())
}

fn lerp(interval: &Interval, frac: f64) -> f64 {
    interval.lower + (interval.upper - interval.lower) * frac
}
