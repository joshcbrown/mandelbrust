use crate::mandelbrot::Complex;
use crate::palette::ColorPalette;
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

    pub fn get_palette(&self, name: String) -> Result<ColorPalette> {
        // TODO: make this generic
        if let Ok(i) = self
            .color_palettes
            .binary_search_by(|point| point.name.cmp(&name))
        {
            Ok(self.color_palettes[i].clone())
        } else {
            Err(anyhow!("{} not found in yaml", name))
        }
    }
}
