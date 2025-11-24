use crate::{encoder::image_buffer::ImageBuffer, state::QOIState};
use image::Rgba;

pub fn handle(qoi_buffer: &mut ImageBuffer, state: &mut QOIState, pixel: &Rgba<u8>) -> bool {
    if state.exists(pixel) {
        qoi_buffer.add_seen_pixel(QOIState::hash(pixel));
        return true;
    }
    false
}
