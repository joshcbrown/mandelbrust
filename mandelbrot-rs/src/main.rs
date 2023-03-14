use anyhow::Result;
use image::{Rgb, RgbImage};
use mandelbrot_rs::mandelbrot::Complex;

fn main() {
    let mut img = RgbImage::new(4096, 4096);
    for x in 0..img.width() {
        for y in 0..img.width() {
            let re = lerp(-2., 2., x as f64 / img.width() as f64);
            let im = lerp(-2., 2., y as f64 / img.height() as f64);
            let c = Complex::new(re, im);
            let escape_count = c.escape_count(Complex::id(), 8., 200);
            let frac = escape_count as f64 / 200. * 256.;
            let col = Rgb([frac as u8, frac as u8, frac as u8]);
            img.put_pixel(x, y, col);
        }
    }
    img.save("penis.png").unwrap();
}

fn lerp(start: f64, end: f64, frac: f64) -> f64 {
    start + (end - start) * frac
}
