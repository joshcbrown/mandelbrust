use crate::config::Configuration;
use crate::mandelbrot::Complex;
use crate::mandelbrot::{generate_escape_counts, generate_hist_counts, normalise_escape_counts};
use anyhow::{anyhow, Result};
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
    #[arg(short, long, value_enum, default_value_t = Resolution::High)]
    pub resolution: Resolution,
    /// algorithm to plot the image using
    #[arg(short, long, value_enum, default_value_t = PlottingAlgorithm::Histogram)]
    algorithm: PlottingAlgorithm,
    #[command(subcommand)]
    command: Commands,
}

impl Cli {
    pub fn get_hue_array(&self) -> Result<Vec<Vec<f64>>> {
        let (width, height): (usize, usize) = self.resolution.to_dimensions();
        let (x_range, y_range) = match &self.command {
            &Commands::Centre { x, y, zoom } => get_intervals(Complex::new(x, y), zoom as f64),
            Commands::CentreString { name } => {
                let file = std::fs::File::open("config.yaml")?;
                let config: Configuration = serde_yaml::from_reader(file)?;
                let centre = if let Ok(i) = config
                    .named_points
                    .binary_search_by(|point| point.name.cmp(&name))
                {
                    config.named_points[i].clone()
                } else {
                    return Err(anyhow!("{} not found in yaml", name));
                };

                get_intervals(centre.point, centre.zoom as f64)
            }
        };
        let escape_counts = match self.algorithm {
            PlottingAlgorithm::Histogram | PlottingAlgorithm::Vanilla => generate_escape_counts(
                &x_range,
                &y_range,
                width,
                height,
                self.max_iters,
                |escape_count, _| escape_count as f64,
            ),
            PlottingAlgorithm::Smooth | PlottingAlgorithm::SmoothHistogram => {
                generate_escape_counts(
                    &x_range,
                    &y_range,
                    width,
                    height,
                    self.max_iters,
                    |escape_count, escape_val| {
                        if escape_count < self.max_iters {
                            let nu = (escape_val.abs_value_sq().log2() / 2.).log2();
                            (escape_count + 1) as f64 - nu
                        } else {
                            self.max_iters as f64
                        }
                    },
                )
            }
        };

        Ok(match self.algorithm {
            PlottingAlgorithm::Vanilla | PlottingAlgorithm::Smooth => {
                normalise_escape_counts(&escape_counts, self.max_iters)
            }
            PlottingAlgorithm::Histogram | PlottingAlgorithm::SmoothHistogram => {
                generate_hist_counts(&escape_counts, self.max_iters, width * height)
            }
        })
    }
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
    CentreString {
        /// name of corresponding NamedPoint in config.yaml
        name: String,
    },
}

#[derive(Debug)]
pub struct Interval {
    pub lower: f64,
    pub upper: f64,
}

impl Interval {
    pub fn lerp(&self, frac: f64) -> f64 {
        self.lower + (self.upper - self.lower) * frac
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
