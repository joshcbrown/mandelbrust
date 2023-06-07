use crate::opts::Interval;
use anyhow::{anyhow, Result};
use image::Rgb;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ColorPalette {
    pub color_vals: Vec<ConfigRGB>,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct ConfigRGB {
    pub value: f64,
    // NOTE: this makes the yaml code really verbose, ideally would like to change this into
    // [red, green, blue] in a vec or hex code
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl ConfigRGB {
    pub fn to_rgb(&self) -> Rgb<u8> {
        Rgb([self.red, self.green, self.blue])
    }

    pub fn lerp(&self, o: &Self, value: f64) -> Rgb<u8> {
        let r_interval = Interval {
            lower: self.red as f64,
            upper: o.red as f64,
        };
        let g_interval = Interval {
            lower: self.green as f64,
            upper: o.green as f64,
        };
        let b_interval = Interval {
            lower: self.blue as f64,
            upper: o.blue as f64,
        };
        let frac = (value - self.value) / (o.value - self.value);

        Rgb([
            r_interval.lerp(frac) as u8,
            g_interval.lerp(frac) as u8,
            b_interval.lerp(frac) as u8,
        ])
    }
}

impl ColorPalette {
    // TODO: switch config to use this instead to enforce constraints
    pub fn new(color_vals: Vec<ConfigRGB>) -> Result<ColorPalette> {
        if color_vals.len() < 2 {
            return Err(anyhow!("need color vals"));
        }

        let mut sorted_colors = color_vals;
        sorted_colors.sort_by(|a, b| a.value.partial_cmp(&b.value).unwrap());

        let first = sorted_colors
            .first()
            .expect("more than 2 vals is an assertion")
            .value;
        let last = sorted_colors.last().unwrap().value;

        if first != 0.0 || last != 1.0 {
            return Err(anyhow!("need vals for 0.0 and 1.0"));
        }

        Ok(ColorPalette {
            color_vals: sorted_colors,
        })
    }

    pub fn value(&self, value: f64) -> Rgb<u8> {
        if value > 1. {
            return self.color_vals.last().unwrap().to_rgb();
        }

        match self
            .color_vals
            .binary_search_by(|&color| color.value.partial_cmp(&value).unwrap())
        {
            Ok(i) => self.color_vals[i].to_rgb(),
            Err(i) => {
                let c1 = self.color_vals[i - 1];
                let c2 = self.color_vals[i];
                c1.lerp(&c2, value)
            }
        }
    }

    pub fn repeat(mut self, n: usize) -> Self {
        let len = self.color_vals.len();
        if len <= 2 {
            return self;
        }
        let space_between = 1 as f64 / ((len - 1) * n) as f64;

        let mut new_vals: Vec<ConfigRGB> = std::iter::repeat(&self.color_vals[..len - 1])
            .take(n)
            .enumerate()
            .flat_map(|(i, pal)| {
                pal.iter().enumerate().map(move |(j, &config_rgb)| {
                    let value = (i * (len - 1) + j) as f64 * space_between;
                    ConfigRGB {
                        value,
                        ..config_rgb
                    }
                })
            })
            .collect();
        new_vals.push(self.color_vals[len - 1]);
        self.color_vals = new_vals;
        println!("{:#?}", self);
        self
    }
}
