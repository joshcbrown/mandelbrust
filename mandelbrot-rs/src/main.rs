use anyhow::Result;
use image::{ImageBuffer, Rgb, RgbImage};
use mandelbrot_rs::mandelbrot::Complex;

fn main() {
    let (width, height) = (2048, 2048);
    let img = ImageBuffer::from_fn(width, height, |x, y| {
        let re = lerp(
            0.281717921930774,
            0.281717921930775,
            x as f64 / width as f64,
        );
        let im = lerp(
            0.5771052841488504,
            0.5771052841488506,
            y as f64 / height as f64,
        );
        let c = Complex::new(re, im);
        let escape_count = c.escape_count(Complex::id(), 8., 1000);
        let frac = escape_count as f64 / 200. * 256.;
        Rgb([frac as u8, frac as u8, frac as u8])
    });
    img.save("penis.png").unwrap();
}

fn lerp(start: f64, end: f64, frac: f64) -> f64 {
    start + (end - start) * frac
}
