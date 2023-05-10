use crate::mandelbrot::Complex;
use crate::palette::ColorPalette;
use crate::palette::ConfigRGB;
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Configuration {
    pub color_palettes: HashMap<String, ColorPalette>,
    pub named_points: HashMap<String, NamedPoint>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NamedPoint {
    pub point: Complex,
    pub zoom: usize,
}

impl Configuration {
    pub fn get_palette(&self, name: &str) -> Result<&ColorPalette> {
        self.color_palettes
            .get(name)
            .ok_or(anyhow!(format!("palette {} in palettes", name)))
    }

    pub fn get_named_point(&self, name: &str) -> Result<&NamedPoint> {
        self.named_points
            .get(name)
            .ok_or(anyhow!(format!("point {} not in points", name)))
    }
}

// don't fucking look here, for your own good
impl Default for Configuration {
    fn default() -> Self {
        let color_palettes = HashMap::from_iter([
            (
                "greyscale".into(),
                ColorPalette {
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
            ),
            (
                "electric".into(),
                ColorPalette {
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
            ),
        ]);

        let named_points = HashMap::from_iter([
            (
                "capillary".into(),
                NamedPoint {
                    point: Complex {
                        re: -0.0452407411,
                        im: 0.9868162204352258,
                    },
                    zoom: 51200000,
                },
            ),
            (
                "chloro-zoom".into(),
                NamedPoint {
                    point: Complex {
                        re: 0.281717921930775,
                        im: 0.5771052841488505,
                    },
                    zoom: 102400000000,
                },
            ),
            (
                "chlorophyll".into(),
                NamedPoint {
                    point: Complex {
                        re: 0.281717921930775,
                        im: 0.5771052841488505,
                    },
                    zoom: 25600000000,
                },
            ),
            (
                "circle".into(),
                NamedPoint {
                    point: Complex {
                        re: 0.432539867562512,
                        im: 0.226118675951765,
                    },
                    zoom: 25000000000000,
                },
            ),
            (
                "divergence".into(),
                NamedPoint {
                    point: Complex {
                        re: 0.2549870375144766,
                        im: -0.0005679790528465,
                    },
                    zoom: 200000,
                },
            ),
            (
                "doubletentacle".into(),
                NamedPoint {
                    point: Complex {
                        re: -0.745428,
                        im: 0.113009,
                    },
                    zoom: 400000,
                },
            ),
            (
                "knife".into(),
                NamedPoint {
                    point: Complex {
                        re: -1.25066,
                        im: 0.02012,
                    },
                    zoom: 75000,
                },
            ),
            (
                "spiral".into(),
                NamedPoint {
                    point: Complex {
                        re: -0.7771204433106587,
                        im: 0.1268572387863619,
                    },
                    zoom: 20000,
                },
            ),
            (
                "splotches".into(),
                NamedPoint {
                    point: Complex {
                        re: -0.74529,
                        im: 0.113075,
                    },
                    zoom: 80000,
                },
            ),
            (
                "tendril".into(),
                NamedPoint {
                    point: Complex {
                        re: -0.235125,
                        im: 0.827215,
                    },
                    zoom: 260000,
                },
            ),
            (
                "tentacle".into(),
                NamedPoint {
                    point: Complex {
                        re: -0.7453,
                        im: 0.1127,
                    },
                    zoom: 10000,
                },
            ),
            (
                "tilt".into(),
                NamedPoint {
                    point: Complex {
                        re: -0.16,
                        im: 1.035,
                    },
                    zoom: 900,
                },
            ),
            (
                "tilt-zoom-out".into(),
                NamedPoint {
                    point: Complex {
                        re: -0.16,
                        im: 1.035,
                    },
                    zoom: 700,
                },
            ),
            (
                "void".into(),
                NamedPoint {
                    point: Complex {
                        re: -1.25066,
                        im: 0.02012,
                    },
                    zoom: 200000,
                },
            ),
        ]);

        Self {
            color_palettes,
            named_points,
        }
    }
}
