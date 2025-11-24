use crate::{encoder::ImageBuffer, state::QOIState};
use image::Rgba;

pub fn handle(qoi_buffer: &mut ImageBuffer, state: &mut QOIState, pixel: &Rgba<u8>) -> bool {
    let [r, g, b, a] = pixel.0;
    let [_, _, _, a_prev] = state.prev_pixel.0;

    if a == a_prev {
        qoi_buffer.add_rgb_pixel(r, g, b);
        return true;
    } else {
        qoi_buffer.add_rgba_pixel(r, g, b, a);
        return true;
    }

    false
}
