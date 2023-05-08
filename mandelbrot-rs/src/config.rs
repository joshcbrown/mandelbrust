use crate::mandelbrot::Complex;
use crate::palette::ColorPalette;
use crate::palette::ConfigRGB;
use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Configuration {
    pub color_palettes: Vec<ColorPalette>,
    pub named_points: Vec<NamedPoint>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NamedPoint {
    pub name: String,
    pub point: Complex,
    pub zoom: usize,
}

impl Configuration {
    pub fn get_config() -> Result<Self> {
        let file = std::fs::File::open("config.yaml")?;
        serde_yaml::from_reader(file).context("problem reading yaml")
    }

    pub fn get_palette(&self, name: &str) -> Result<ColorPalette> {
        self.search_name(name, |c| &c.color_palettes, |p| p.name.as_str())
    }

    pub fn get_named_point(&self, name: &str) -> Result<NamedPoint> {
        self.search_name(name, |c| &c.named_points, |p| p.name.as_str())
    }

    fn search_name<T, F, G>(&self, name: &str, get_vec: F, get_name: G) -> Result<T>
    where
        F: Fn(&Self) -> &[T],
        G: Fn(&T) -> &str,
        T: Clone,
    {
        let search_vec = get_vec(self);
        if let Ok(i) = search_vec.binary_search_by_key(&name, get_name) {
            Ok(search_vec[i].clone())
        } else {
            Err(anyhow!("{} not found in yaml", name))
        }
    }
}

// don't fucking look here, for your own good
impl Default for Configuration {
    fn default() -> Self {
        let color_palettes = vec![
            ColorPalette {
                name: "greyscale".to_string(),
                color_vals: vec![
                    ConfigRGB {
                        value: 0.0,
                        red: 0,
                        green: 0,
                        blue: 0,
                    },
                    ConfigRGB {
                        value: 1.0,
                        red: 255,
                        green: 255,
                        blue: 255,
                    },
                ],
            },
            ColorPalette {
                name: "electric".to_string(),
                color_vals: vec![
                    ConfigRGB {
                        value: 0.0,
                        red: 0,
                        green: 18,
                        blue: 25,
                    },
                    ConfigRGB {
                        value: 0.1,
                        red: 0,
                        green: 18,
                        blue: 25,
                    },
                    ConfigRGB {
                        value: 0.5,
                        red: 20,
                        green: 33,
                        blue: 61,
                    },
                    ConfigRGB {
                        value: 0.8,
                        red: 252,
                        green: 163,
                        blue: 17,
                    },
                    ConfigRGB {
                        value: 0.9,
                        red: 229,
                        green: 229,
                        blue: 229,
                    },
                    ConfigRGB {
                        value: 0.9999,
                        red: 255,
                        green: 255,
                        blue: 255,
                    },
                    ConfigRGB {
                        value: 1.0,
                        red: 0,
                        green: 0,
                        blue: 0,
                    },
                ],
            },
        ];

        let named_points = vec![
            NamedPoint {
                name: "capillary".to_string(),
                point: Complex {
                    re: -0.0452407411,
                    im: 0.9868162204352258,
                },
                zoom: 51200000,
            },
            NamedPoint {
                name: "chloro-zoom".to_string(),
                point: Complex {
                    re: 0.281717921930775,
                    im: 0.5771052841488505,
                },
                zoom: 102400000000,
            },
            NamedPoint {
                name: "chlorophyll".to_string(),
                point: Complex {
                    re: 0.281717921930775,
                    im: 0.5771052841488505,
                },
                zoom: 25600000000,
            },
            NamedPoint {
                name: "circle".to_string(),
                point: Complex {
                    re: 0.432539867562512,
                    im: 0.226118675951765,
                },
                zoom: 25000000000000,
            },
            NamedPoint {
                name: "divergence".to_string(),
                point: Complex {
                    re: 0.2549870375144766,
                    im: -0.0005679790528465,
                },
                zoom: 200000,
            },
            NamedPoint {
                name: "doubletentacle".to_string(),
                point: Complex {
                    re: -0.745428,
                    im: 0.113009,
                },
                zoom: 400000,
            },
            NamedPoint {
                name: "knife".to_string(),
                point: Complex {
                    re: -1.25066,
                    im: 0.02012,
                },
                zoom: 75000,
            },
            NamedPoint {
                name: "spiral".to_string(),
                point: Complex {
                    re: -0.7771204433106587,
                    im: 0.1268572387863619,
                },
                zoom: 20000,
            },
            NamedPoint {
                name: "splotches".to_string(),
                point: Complex {
                    re: -0.74529,
                    im: 0.113075,
                },
                zoom: 80000,
            },
            NamedPoint {
                name: "tendril".to_string(),
                point: Complex {
                    re: -0.235125,
                    im: 0.827215,
                },
                zoom: 260000,
            },
            NamedPoint {
                name: "tentacle".to_string(),
                point: Complex {
                    re: -0.7453,
                    im: 0.1127,
                },
                zoom: 10000,
            },
            NamedPoint {
                name: "tilt".to_string(),
                point: Complex {
                    re: -0.16,
                    im: 1.035,
                },
                zoom: 900,
            },
            NamedPoint {
                name: "tilt-zoom-out".to_string(),
                point: Complex {
                    re: -0.16,
                    im: 1.035,
                },
                zoom: 700,
            },
            NamedPoint {
                name: "void".to_string(),
                point: Complex {
                    re: -1.25066,
                    im: 0.02012,
                },
                zoom: 200000,
            },
        ];

        Self {
            color_palettes,
            named_points,
        }
    }
}
