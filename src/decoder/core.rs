use crate::{
    decoder::{chunks::decode_pixel, header::QOIHeader},
    errors::QOIError,
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

    while let Some(pixel_chunk) = decode_pixel(&mut iter) {}

    Ok(None)
}
