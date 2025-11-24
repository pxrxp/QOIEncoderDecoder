use crate::{encoder::ImageBuffer, state::QOIState};
use image::Rgba;

fn diff_tag_eligible(dr: u8, dg: u8, db: u8, da: u8) -> bool {
    (da == 0) && (dr.wrapping_add(2) <= 3) && (dg.wrapping_add(2) <= 3) && (db.wrapping_add(2) <= 3)
}

fn luma_tag_eligible(dr: u8, dg: u8, db: u8, da: u8) -> bool {
    (da == 0)
        && (dr.wrapping_add(32) <= 63)
        && (dg.wrapping_add(8) <= 15)
        && (db.wrapping_add(8) <= 15)
}

pub fn handle(qoi_buffer: &mut ImageBuffer, state: &mut QOIState, pixel: &Rgba<u8>) -> bool {
    let [r, g, b, a] = pixel.0;
    let [r_prev, g_prev, b_prev, a_prev] = state.prev_pixel.0;

    let dr = r.wrapping_sub(r_prev);
    let dg = g.wrapping_sub(g_prev);
    let db = b.wrapping_sub(b_prev);
    let da = a.wrapping_sub(a_prev);

    if diff_tag_eligible(dr, dg, db, da) {
        qoi_buffer.add_diff_pixel(dr, dg, db);
        return true;
    }

    if luma_tag_eligible(dr, dg, db, da) {
        qoi_buffer.add_luma_pixel(dr, dg, db);
        return true;
    }

    false
}
