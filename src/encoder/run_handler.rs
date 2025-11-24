use crate::{QOIState, encoder::ImageBuffer};
use image::Rgba;

pub fn handle(qoi_buffer: &mut ImageBuffer, state: &mut QOIState, pixel: &Rgba<u8>) -> bool {
    let run_increment = if state.run_length + 1 > 62 { 0 } else { 1 };

    if *pixel == state.prev_pixel {
        state.run_length += run_increment;
        return true;
    }

    let run_length_limit_exceeded = *pixel == state.prev_pixel && run_increment == 0;
    let pixel_changed = *pixel != state.prev_pixel && state.run_length != 0;

    if run_length_limit_exceeded || pixel_changed {
        qoi_buffer.add_run_pixels(state.run_length);
        state.run_length = 0;
    }

    false
}

pub fn cleanup(qoi_buffer: &mut ImageBuffer, state: &mut QOIState) {
    if state.run_length != 0 {
        qoi_buffer.add_run_pixels(state.run_length);
        state.run_length = 0;
    }
}
