use super::ImageBuffer;
use image::Rgba;

pub struct NormalHandler {
    has_alpha: bool,
}

impl NormalHandler {
    pub fn new(has_alpha: bool) -> Self {
        Self { has_alpha }
    }

    pub fn handle(&mut self, qoi_buffer: &mut ImageBuffer, pixel: &Rgba<u8>, handled: &mut bool) {
        if !*handled {
            let [r, g, b, a] = pixel.0;
            if self.has_alpha {
                qoi_buffer.add_rgba_pixel(r, g, b, a);
                *handled = true;
            } else {
                qoi_buffer.add_rgb_pixel(r, g, b);
                *handled = true;
            }
        }
    }
}
