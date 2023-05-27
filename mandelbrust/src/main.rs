use anyhow::{Context, Result};
use iced::widget::{self, column, row, text, Button, Slider};
use iced::{alignment, Length, Point, Sandbox, Settings};
use image::{ImageBuffer, Rgb};
use mandelbruhst_cli::mandelbrot::Complex;
use mandelbruhst_cli::opts::{
    get_intervals, Cli, Commands, Interval, PlottingAlgorithm, Resolution,
};
use mandelbruhst_gui::mouse_area::MouseArea;

pub fn main() -> iced::Result {
    App::run(Settings::default())
}

#[derive(Debug, Clone)]
pub enum Message {
    PointClicked(f32, f32),
    ZoomIn,
    ZoomOut,
    ZoomMultiplier(f32),
}

pub struct App {
    centre: Complex,
    zoom: f64,
    zoom_multiplier: f32,
    handle: widget::image::Handle,
}

impl Sandbox for App {
    type Message = Message;

    fn new() -> Self {
        let centre = Complex::id();
        let zoom = 8_f64;
        let image = refresh_image(centre, zoom as usize).unwrap();
        Self {
            centre,
            zoom,
            handle: widget::image::Handle::from_pixels(960, 540, image.into_raw()),
            zoom_multiplier: 2.,
        }
    }

    fn title(&self) -> String {
        String::from("Mandelbrot - Iced")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::PointClicked(x_per, y_per) => {
                let (x_bounds, y_bounds) = get_intervals(self.centre, self.zoom as f64);
                self.centre =
                    Complex::new(x_bounds.lerp(x_per as f64), y_bounds.lerp(y_per as f64));
            }
            Message::ZoomIn => self.zoom *= self.zoom_multiplier as f64,
            Message::ZoomOut => self.zoom /= self.zoom_multiplier as f64,
            Message::ZoomMultiplier(new_zoom) => {
                self.zoom_multiplier = new_zoom;
                return;
            }
        }
        let buf = refresh_image(self.centre, self.zoom as usize).unwrap();
        self.handle = widget::image::Handle::from_memory(buf.into_raw())
    }

    fn view(&self) -> iced::Element<Message> {
        let clickable_image = widget::image(self.handle.clone());

        let point_text = format!(
            "Centre: {}, zoom: {}, zoom_multiplier: {}",
            self.centre.to_string(),
            self.zoom as f64,
            self.zoom_multiplier
        );
        let middle_row = row![
            button("+").on_press(Message::ZoomIn),
            button("-").on_press(Message::ZoomOut),
            Slider::new(1.0..=50.0, self.zoom_multiplier, Message::ZoomMultiplier)
        ]
        .spacing(20);
        column![clickable_image, middle_row, text(&point_text).size(20),]
            .spacing(20)
            .into()
    }

    fn theme(&self) -> iced::Theme {
        iced::Theme::Dark
    }
}

fn button<'a, Message: Clone>(label: &str) -> Button<'a, Message> {
    iced::widget::button(text(label).horizontal_alignment(alignment::Horizontal::Center))
        .padding(12)
        .width(100)
}

fn refresh_image(centre: Complex, zoom: usize) -> Result<ImageBuffer<Rgb<u8>, Vec<u8>>> {
    let args = Cli {
        out_file: "".to_string(),
        max_iters: 5000,
        bailout: 1e9,
        resolution: Resolution::Med,
        palette: "electric".to_string(),
        algorithm: PlottingAlgorithm::SmoothHistogram,
        command: Commands::Centre {
            x: centre.re,
            y: centre.im,
            zoom,
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
    Ok(buf)
}
