use crate::{
    decoder::{
        chunks::{PixelChunk, decode_pixel},
        header::QOIHeader,
    },
    errors::QOIError,
    state::QOIState,
};
use image::DynamicImage;
use std::fs;

pub fn decode_file(image_path: &str) -> Result<Option<DynamicImage>, QOIError> {
    let bytes: Vec<u8> = fs::read(image_path).map_err(|_| QOIError::FileReadError)?;
    decode(&bytes)
}

pub fn decode(image_bytes: &Vec<u8>) -> Result<Option<DynamicImage>, QOIError> {
    let mut iter = image_bytes.iter();

    let header = QOIHeader::new(&mut iter)?;
    let state = QOIState::new();

    while let Some(pixel_chunk) = decode_pixel(&mut iter) {
        match pixel_chunk {
            PixelChunk::RGB(r, g, b) => {}
            PixelChunk::RGBA(r, g, b, a) => {}
            PixelChunk::Index(i) => {}
            PixelChunk::Diff(dr, dg, db) => {}
            PixelChunk::Luma(dg, dr_dg, db_dg) => {}
            PixelChunk::Run(run) => {}
        }
    }

    Ok(None)
}
