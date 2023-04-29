use image::Rgb;

pub struct ColorPalette {
    colors: Vec<(f64, Rgb<u8>)>,
}

impl ColorPalette {
    pub fn new(colors: Vec<(f64, Rgb<u8>)>) -> Option<ColorPalette> {
        if colors.is_empty() {
            return None;
        }

        let mut sorted_colors = colors;
        sorted_colors.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        let first = sorted_colors.first().unwrap().0;
        let last = sorted_colors.last().unwrap().0;

        if first != 0.0 || last != 1.0 {
            return None;
        }

        Some(ColorPalette {
            colors: sorted_colors,
        })
    }

    pub fn value(&self, value: f64) -> Rgb<u8> {
        if value > 1. {
            return self.colors.last().unwrap().1;
        }
        match self
            .colors
            .binary_search_by(|&(v, _)| v.partial_cmp(&value).unwrap())
        {
            Ok(i) => self.colors[i].1,
            Err(i) => {
                let (v1, c1) = self.colors[i - 1];
                let (v2, c2) = self.colors[i];

                let t = (value - v1) / (v2 - v1);

                let r = c1[0] + (t * (c2[0] as f64 - c1[0] as f64)) as u8;
                let g = c1[1] + (t * (c2[1] as f64 - c1[1] as f64)) as u8;
                let b = c1[2] + (t * (c2[2] as f64 - c1[2] as f64)) as u8;

                Rgb([r, g, b])
            }
        }
    }
}
