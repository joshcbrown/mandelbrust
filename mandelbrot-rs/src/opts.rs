use crate::mandelbrot::Complex;
use anyhow::{anyhow, Result};
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// filepath to save the output image to
    #[arg(short, long)]
    out_file: Option<String>,
    #[arg(short, long)]
    resolution: Option<String>,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Centre { x: f64, y: f64, zoom: Option<usize> },
}

#[derive(Debug)]
pub struct Config {
    pub x_range: Interval,
    pub y_range: Interval,
    pub zoom: usize,
    pub out_file: String,
    pub width: usize,
    pub height: usize,
}

#[derive(Debug)]
pub struct Interval {
    pub lower: f64,
    pub upper: f64,
}

pub fn parse_args() -> Result<Config> {
    let args = Cli::parse();
    let out_file = args.out_file.unwrap_or("penis.png".into());
    let (width, height): (usize, usize) = match args.resolution.unwrap_or("high".into()).as_str() {
        "low" => Ok((320, 180)),
        "med" => Ok((960, 540)),
        "high" => Ok((1920, 1080)),
        _ => Err(anyhow!("resolution must be one of low, med, or high.")),
    }?;
    match args.command {
        Commands::Centre { x, y, zoom } => {
            let zoom = zoom.unwrap_or(8);
            let centre = Complex::new(x, y);
            let (x_range, y_range) = get_intervals(centre, zoom as f64);
            Ok(Config {
                x_range,
                y_range,
                zoom,
                out_file,
                width,
                height,
            })
        }
    }
}

fn get_intervals(centre: Complex, zoom: f64) -> (Interval, Interval) {
    return (
        Interval {
            lower: centre.re - 16. / zoom,
            upper: centre.re + 16. / zoom,
        },
        Interval {
            lower: centre.im - 9. / zoom,
            upper: centre.im + 9. / zoom,
        },
    );
}
