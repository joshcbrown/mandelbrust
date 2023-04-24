use crate::mandelbrot::Complex;
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// filepath to save the output image to
    #[arg(short, long)]
    out_file: Option<String>,
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
}

#[derive(Debug)]
pub struct Interval {
    pub lower: f64,
    pub upper: f64,
}

pub fn parse_args() -> Config {
    let args = Cli::parse();
    let out_file = args.out_file.unwrap_or("penis.png".into());
    match args.command {
        Commands::Centre { x, y, zoom } => {
            let zoom = zoom.unwrap_or(8);
            let centre = Complex::new(x, y);
            let (x_range, y_range) = get_intervals(centre, zoom as f64);
            Config {
                x_range,
                y_range,
                zoom,
                out_file,
            }
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
