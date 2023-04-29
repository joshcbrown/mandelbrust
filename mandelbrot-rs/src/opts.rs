use crate::mandelbrot::Complex;
use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// filepath to save the output image to
    #[arg(short, long, default_value = "mandelbrot.png")]
    out_file: String,
    /// number of iterations to perform before deciding if a point is in the set
    #[arg(short, long, default_value_t = 2000)]
    max_iters: usize,
    /// resolution of the output image.
    #[arg(value_enum, default_value_t = Resolution::High)]
    resolution: Resolution,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Resolution {
    Low,
    Med,
    High,
}

impl Resolution {
    fn to_dimensions(self) -> (usize, usize) {
        match self {
            Resolution::Low => (320, 180),
            Resolution::Med => (960, 540),
            Resolution::High => (1920, 1080),
        }
    }
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
    pub max_iters: usize,
}

#[derive(Debug)]
pub struct Interval {
    pub lower: f64,
    pub upper: f64,
}

pub fn parse_args() -> Config {
    let args = Cli::parse();
    let (width, height): (usize, usize) = args.resolution.to_dimensions();
    match args.command {
        Commands::Centre { x, y, zoom } => {
            let zoom = zoom.unwrap_or(8);
            let centre = Complex::new(x, y);
            let (x_range, y_range) = get_intervals(centre, zoom as f64);
            Config {
                x_range,
                y_range,
                zoom,
                out_file: args.out_file,
                width,
                height,
                max_iters: args.max_iters,
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
