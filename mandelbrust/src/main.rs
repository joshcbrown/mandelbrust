use anyhow::{Context, Result};
use eframe::egui::{Image, Sense, Slider, Ui};
use eframe::emath::Align;
use eframe::epaint::ColorImage;
use eframe::{egui, run_native};
use image::{ImageBuffer, Rgb};
use mandelbruhst_cli::mandelbrot::Complex;
use mandelbruhst_cli::opts::{
    get_intervals, Cli, Commands, Interval, PlottingAlgorithm, Resolution,
};

pub fn main() -> Result<(), eframe::Error> {
    run_native(
        "mandelbrot explorer",
        Default::default(),
        Box::new(|_cc| Box::new(App::default())),
    )
}

pub struct App {
    centre: Complex,
    zoom: f64,
    zoom_multiplier: f32,
    image_texture: Option<egui::TextureHandle>,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("heading")
            .resizable(false)
            .show(ctx, |ui| {
                ui.with_layout(egui::Layout::top_down_justified(Align::Center), |ui| {
                    ui.heading("mandelbust")
                });
            });

        egui::SidePanel::left("config")
            .resizable(false)
            .show(ctx, |ui| {
                ui.with_layout(egui::Layout::top_down_justified(Align::Min), |ui| {
                    let mut refresh = false;
                    ui.columns(2, |ui| {
                        if ui[0].button("+").clicked() {
                            self.zoom *= self.zoom_multiplier as f64;
                            refresh = true;
                        }
                        if ui[1].button("-").clicked() {
                            self.zoom /= self.zoom_multiplier as f64;
                            refresh = true;
                        }
                    });
                    ui.add(Slider::new(&mut self.zoom_multiplier, 1.0..=10.));
                    if refresh {
                        self.refresh_image(ui).unwrap();
                    }
                })
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            if self.image_texture.is_none() {
                self.refresh_image(ui).unwrap();
            }

            let texture = self.image_texture.clone().unwrap();
            let image_response =
                ui.add(Image::new(&texture, texture.size_vec2()).sense(Sense::click()));

            if image_response.clicked() {
                let rect = image_response.rect;
                let rel_position = image_response.hover_pos().unwrap() - rect.left_top();
                let (x_bounds, y_bounds) = get_intervals(self.centre, self.zoom);
                self.centre = Complex::new(
                    x_bounds.lerp(rel_position.x as f64 / rect.width() as f64),
                    y_bounds.lerp(rel_position.y as f64 / rect.height() as f64),
                );
                self.refresh_image(ui).unwrap();
            }
        });
    }
}

impl Default for App {
    fn default() -> Self {
        Self {
            centre: Complex::id(),
            zoom: 8.,
            zoom_multiplier: 2.,
            image_texture: None,
        }
    }
}

impl App {
    fn refresh_image(&mut self, ui: &Ui) -> Result<()> {
        let args = Cli {
            out_file: "".to_string(),
            max_iters: 5000,
            bailout: 1e9,
            resolution: Resolution::Med,
            palette: "electric".to_string(),
            algorithm: PlottingAlgorithm::SmoothHistogram,
            command: Commands::Centre {
                x: self.centre.re,
                y: self.centre.im,
                zoom: self.zoom as usize,
            },
        };
        let hue_array = args.get_hue_array()?;
        let (width, height) = args.resolution.to_dimensions();
        let palette = args.get_palette()?;
        let buf = ImageBuffer::from_fn(width as u32, height as u32, |x, y| {
            let frac = hue_array[x as usize][y as usize];
            palette.value(frac)
        });
        buf.save("img.png")?;
        let image = ColorImage::from_rgb([960, 540], &buf);
        self.image_texture = Some(ui.ctx().load_texture("image", image, Default::default()));
        Ok(())
    }
}
