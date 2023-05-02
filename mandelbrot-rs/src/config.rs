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
