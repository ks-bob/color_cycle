use image::imageops;
use image::{io::Reader, DynamicImage, GenericImageView, ImageBuffer, Pixel, Rgba};
use minifb::{Key, Window, WindowOptions};
use std::f64::consts::PI;
use std::time::Duration;

fn main() {
    let image = Reader::open("./minecraft.jpg").unwrap().decode().unwrap();
    let (width, height) = image.dimensions();

    let width = width as usize;
    let height = height as usize;

    let mut window = Window::new("Color Cycle", width, height, WindowOptions::default()).unwrap();

    // Limit to about 60 FPS.
    window.limit_update_rate(Some(Duration::from_micros(16600)));

    // Create an object to keep track of the overlay state.
    let mut overlay = Overlay::new(50);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let mut image = image.clone();
        overlay.next_color();
        overlay.apply(&mut image);

        let buffer = create_argb_buffer(image);
        window.update_with_buffer(&buffer, width, height).unwrap();
    }
}

/// Handles smooth RGBA overlay state and application.
struct Overlay {
    alpha: u8,
    cycle_position: f64,
}

impl Overlay {
    const MAX_COLOR: f64 = 255.0;
    const CYCLE_INCREMENT: f64 = 0.2;

    fn new(alpha: u8) -> Self {
        Self {
            alpha,
            cycle_position: 0.0,
        }
    }

    fn smooth_rgb(cycle_position: f64) -> (u8, u8, u8) {
        let (r, g, b) = (
            (cycle_position + 2.0 * PI / 3.0).sin() * Self::MAX_COLOR,
            (cycle_position).sin() * Self::MAX_COLOR,
            (cycle_position - 2.0 * PI / 3.0).sin() * Self::MAX_COLOR,
        );

        (
            f64::round(r) as u8,
            f64::round(g) as u8,
            f64::round(b) as u8,
        )
    }

    /// Advance the cycle position to the next color.
    fn next_color(&mut self) {
        self.cycle_position += Self::CYCLE_INCREMENT;

        if self.cycle_position >= 2.0 * PI {
            self.cycle_position -= 2.0 * PI;
        }
    }

    /// Applies smooth RGBA overlay to passed image.
    fn apply(&self, image: &mut DynamicImage) {
        let (width, height) = image.dimensions();
        let (red, green, blue) = Self::smooth_rgb(self.cycle_position);
        let pixel = Rgba([red, green, blue, self.alpha]);
        let overlay = ImageBuffer::from_pixel(width, height, pixel);
        imageops::overlay(image, &overlay, 0, 0);
    }
}

/// Converts RGBA buffer to an ARGB buffer as expected by minifb.
fn create_argb_buffer(image: DynamicImage) -> Vec<u32> {
    let (width, height) = image.dimensions();
    let width = width as usize;
    let height = height as usize;

    let mut buffer: Vec<u32> = vec![0; width * height];

    for y in 0..height {
        for x in 0..width {
            let pixel = image.get_pixel(x as u32, y as u32);
            let channels = pixel.channels();
            let red = channels[0] as u32;
            let green = channels[1] as u32;
            let blue = channels[2] as u32;
            let alpha = channels[3] as u32;
            buffer[y * width + x] = (alpha << 24) | (red << 16) | (green << 8) | blue;
        }
    }

    buffer
}
