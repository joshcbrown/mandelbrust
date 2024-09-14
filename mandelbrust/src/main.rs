use anyhow::Result;
use eframe::egui::{DragValue, Image, Key, Sense, Slider, Ui};
use eframe::emath::Align;
use eframe::epaint::ColorImage;
use eframe::{egui, run_native};
use image::{ImageBuffer, Rgb};
use mandelbrust_cli::config::{Configuration, NamedPoint};
use mandelbrust_cli::mandelbrot::Complex;
use mandelbrust_cli::opts::{get_intervals, Cli, Commands, PlottingAlgorithm, Resolution};

pub fn main() -> Result<(), eframe::Error> {
    run_native(
        "mandelbrot explorer",
        Default::default(),
        Box::new(|_cc| Box::<App>::default()),
    )
}

pub struct App {
    config: Configuration,
    centre: Complex,
    zoom: f64,
    zoom_multiplier: f32,
    palette: String,
    landmark: String,
    image_texture: Option<egui::TextureHandle>,
    image: Option<ImageBuffer<Rgb<u8>, Vec<u8>>>,
    iterations: usize,
    palette_cycles: usize,
    new_palette_name: String,
}

impl Default for App {
    fn default() -> Self {
        Self {
            config: confy::load("mandelbrot-rs", "config").unwrap(),
            centre: Complex::id(),
            zoom: 8.,
            zoom_multiplier: 2.,
            palette: "electric".into(),
            landmark: "".into(),
            image_texture: None,
            image: None,
            iterations: 5000,
            palette_cycles: 1,
            new_palette_name: "".into(),
        }
    }
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
            .show(ctx, |ui| self.render_opts(ui));

        egui::CentralPanel::default().show(ctx, |ui| self.render_image(ui));
    }
}

impl App {
    fn refresh_image(&mut self) -> Result<()> {
        let args = Cli {
            out_file: "".to_string(),
            max_iters: self.iterations,
            bailout: 1e9,
            resolution: Resolution::Med,
            palette: self.palette.clone(),
            algorithm: PlottingAlgorithm::SmoothHistogram,
            command: Commands::Centre {
                x: self.centre.re,
                y: self.centre.im,
                zoom: self.zoom as usize,
            },
            palette_repeats: self.palette_cycles,
        };
        let hue_array = args.get_hue_array()?;
        let (width, height) = args.resolution.to_dimensions();
        let palette = args.get_palette()?;
        self.image = Some(ImageBuffer::from_fn(width as u32, height as u32, |x, y| {
            palette.value(hue_array[x as usize][y as usize])
        }));
        Ok(())
    }

    fn render_opts(&mut self, ui: &mut Ui) {
        ui.with_layout(egui::Layout::top_down_justified(Align::Min), |ui| {
            ui.label("zoom controls");
            let mut refresh = false;
            ui.columns(2, |ui| {
                ui[0].vertical_centered(|ui| {
                    if ui.button("      +      ").clicked() {
                        self.zoom *= self.zoom_multiplier as f64;
                        refresh = true;
                    }
                });
                ui[1].vertical_centered(|ui| {
                    if ui.button("      -      ").clicked() {
                        self.zoom /= self.zoom_multiplier as f64;
                        refresh = true;
                    }
                })
            });
            ui.add_space(20.);
            ui.label("iterations");
            let iterations_field = ui.add(DragValue::new(&mut self.iterations).speed(5.));

            if (iterations_field.lost_focus()
                && iterations_field
                    .ctx
                    .input(|input| input.key_pressed(Key::Enter)))
                || (iterations_field
                    .ctx
                    .input(|input| input.pointer.any_released()))
            {
                self.refresh_image().unwrap();
            }
            ui.label("palette cycles");
            let cycles_field = ui.add(DragValue::new(&mut self.palette_cycles).speed(1.));

            if (cycles_field.lost_focus()
                && cycles_field
                    .ctx
                    .input(|input| input.key_pressed(Key::Enter)))
                || (cycles_field.ctx.input(|input| input.pointer.any_released()))
            {
                self.refresh_image().unwrap();
            }

            ui.add_space(20.);
            ui.label(format!("centre: {}", self.centre.to_string()));

            ui.add_space(20.);
            ui.label("sensitivity");
            ui.add(Slider::new(&mut self.zoom_multiplier, 1.1..=10.));
            if refresh {
                self.refresh_image().unwrap();
            }
            ui.add_space(20.);
            let mut conf = self.config.clone();
            egui::ComboBox::from_label("palette")
                .selected_text(self.palette.clone())
                .show_ui(ui, |ui| {
                    let palettes = conf.color_palettes.keys();
                    for option in palettes {
                        if ui
                            .selectable_value(&mut self.palette, option.into(), option)
                            .clicked()
                        {
                            self.refresh_image().unwrap();
                        };
                    }
                });

            ui.add_space(20.);
            ui.text_edit_singleline(&mut self.new_palette_name);
            ui.add_space(10.);
            if ui.button("save new landmark").clicked() {
                let landmark = NamedPoint {
                    point: self.centre,
                    zoom: self.zoom as _,
                };
                conf.named_points
                    .insert(self.new_palette_name.clone(), landmark);
                confy::store("mandelbrot-rs", "config", conf.clone()).unwrap();
                self.config = conf.clone();
            }

            ui.add_space(20.);
            egui::ComboBox::from_label("landmark")
                .selected_text(if self.landmark.is_empty() {
                    "...".into()
                } else {
                    self.landmark.clone()
                })
                .show_ui(ui, |ui| {
                    let points = conf.named_points.keys();
                    for point_name in points {
                        if ui
                            .selectable_value(&mut self.landmark, point_name.into(), point_name)
                            .clicked()
                        {
                            let point = self.config.named_points.get(point_name).unwrap();
                            self.centre = point.point;
                            self.zoom = point.zoom as f64;
                            self.refresh_image().unwrap();
                        }
                    }
                });
            ui.add_space(20.);
            if ui.button("save image").clicked() {
                self.refresh_image().unwrap();
                self.image.take().unwrap().save("mandelbrot.png").unwrap();
            };
        });
    }

    fn render_image(&mut self, ui: &mut Ui) {
        // TODO: resize image with image crate according to window size
        if self.image_texture.is_none() {
            self.refresh_image().unwrap();
        }
        if let Some(image) = self.image.take() {
            self.image_texture = Some(ui.ctx().load_texture(
                "image",
                ColorImage::from_rgb([960, 540], &image),
                Default::default(),
            ));
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
            self.refresh_image().unwrap();
        }
    }
}
