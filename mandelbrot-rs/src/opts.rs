use crate::mandelbrot::Complex;
use crate::mandelbrot::{generate_escape_counts, generate_hist_counts, normalise_escape_counts};
use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// filepath to save the output image to
    #[arg(short, long, default_value = "mandelbrot.png")]
    pub out_file: String,
    /// number of iterations to perform before deciding if a point is in the set
    #[arg(short, long, default_value_t = 2000)]
    max_iters: usize,
    /// resolution of the output image.
    #[arg(value_enum, default_value_t = Resolution::High)]
    pub resolution: Resolution,
    /// algorithm to plot the image using
    #[arg(value_enum, default_value_t = PlottingAlgorithm::Histogram)]
    algorithm: PlottingAlgorithm,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Resolution {
    Low,
    Med,
    High,
}

impl Resolution {
    pub fn to_dimensions(self) -> (usize, usize) {
        match self {
            Resolution::Low => (320, 180),
            Resolution::Med => (960, 540),
            Resolution::High => (1920, 1080),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum PlottingAlgorithm {
    Vanilla,
    Smooth,
    Histogram,
    SmoothHistogram,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Centre {
        x: f64,
        y: f64,
        #[arg(short, long, default_value_t = 8)]
        zoom: usize,
    },
}

#[derive(Debug)]
pub struct Interval {
    pub lower: f64,
    pub upper: f64,
}

pub fn get_hue_array(args: &Cli) -> Vec<Vec<f64>> {
    let (width, height): (usize, usize) = args.resolution.to_dimensions();
    let (x_range, y_range) = match args.command {
        Commands::Centre { x, y, zoom } => get_intervals(Complex::new(x, y), zoom as f64),
    };
    let escape_counts = match args.algorithm {
        PlottingAlgorithm::Histogram | PlottingAlgorithm::Vanilla => generate_escape_counts(
            &x_range,
            &y_range,
            width,
            height,
            args.max_iters,
            |escape_count, _| escape_count,
        ),
        PlottingAlgorithm::Smooth | PlottingAlgorithm::SmoothHistogram => generate_escape_counts(
            &x_range,
            &y_range,
            width,
            height,
            args.max_iters,
            |escape_count, escape_val| {
                if escape_count < args.max_iters {
                    let nu = (escape_val.abs_value_sq().log2() / 2.).log2();
                    ((escape_count + 1) as f64 - nu) as usize
                } else {
                    args.max_iters
                }
            },
        ),
    };

    match args.algorithm {
        PlottingAlgorithm::Vanilla | PlottingAlgorithm::Smooth => {
            normalise_escape_counts(&escape_counts, args.max_iters)
        }
        PlottingAlgorithm::Histogram | PlottingAlgorithm::SmoothHistogram => {
            generate_hist_counts(&escape_counts, args.max_iters, width * height)
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
