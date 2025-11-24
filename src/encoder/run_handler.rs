use crate::encoder::ImageBuffer;
use image::Rgba;

pub struct RunHandler {
    run_length: u8,
}

impl RunHandler {
    pub fn new() -> Self {
        Self { run_length: 0 }
    }

    pub fn handle(
        &mut self,
        qoi_buffer: &mut ImageBuffer,
        pixel: &Rgba<u8>,
        prev_pixel: &Rgba<u8>,
        handled: &mut bool,
    ) {
        if *handled {
            return;
        }

        let run_increment = if self.run_length + 1 > 62 { 0 } else { 1 };

        if pixel == prev_pixel {
            self.run_length += run_increment;
            *handled = true;
        }

        let run_length_limit_exceeded = pixel == prev_pixel && run_increment == 0;
        let pixel_changed = pixel != prev_pixel && self.run_length != 0;

        if run_length_limit_exceeded || pixel_changed {
            qoi_buffer.add_run_pixels(self.run_length);
            self.run_length = 0;
        }
    }

    pub fn cleanup(&mut self, qoi_buffer: &mut ImageBuffer) {
        if self.run_length != 0 {
            qoi_buffer.add_run_pixels(self.run_length);
            self.run_length = 0;
        }
    }
}
