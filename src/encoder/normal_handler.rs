use crate::encoder::ImageBuffer;
use image::Rgba;

pub struct NormalHandler;

impl NormalHandler {
    pub fn new() -> Self {
        Self
    }

    pub fn handle(&mut self, qoi_buffer: &mut ImageBuffer, pixel: &Rgba<u8>, handled: &mut bool) {
        if *handled {
            return;
        }

        let [r, g, b, a] = pixel.0;
        let [_, _, _, a_prev] = pixel.0;

        if a == a_prev {
            qoi_buffer.add_rgb_pixel(r, g, b);
            *handled = true;
        } else {
            qoi_buffer.add_rgba_pixel(r, g, b, a);
            *handled = true;
        }
    }
}
