use crate::errors::QOIError;
use image::DynamicImage;
use std::fs;

struct QOIHeader {
    width: u32,
    height: u32,
    channels: u8,
    colorspace: u8,
}

impl QOIHeader {
    fn new<'a, I>(iter: &mut I) -> Result<Self, QOIError>
    where
        I: Iterator<Item = &'a u8>,
    {
        let magic_chunks = chunk(iter, 4);
        if magic_chunks != b"qoif" {
            return Err(QOIError::ImageDecodeError);
        }

        let width = u32::from_be_bytes(
            chunk(iter, 4)
                .try_into()
                .map_err(|_| QOIError::ImageDecodeError)?,
        );
        let height = u32::from_be_bytes(
            chunk(iter, 4)
                .try_into()
                .map_err(|_| QOIError::ImageDecodeError)?,
        );
        let channels = *iter.next().ok_or(QOIError::ImageDecodeError)?;
        let colorspace = *iter.next().ok_or(QOIError::ImageDecodeError)?;

        Ok(Self {
            width,
            height,
            channels,
            colorspace,
        })
    }
}

pub struct DecodedImage {
    image: Option<DynamicImage>,
    errors: Vec<String>,
}

pub fn decode_file(image_path: &str) -> Result<DecodedImage, QOIError> {
    let bytes: Vec<u8> = fs::read(image_path).map_err(|_| QOIError::FileReadError)?;
    decode(&bytes)
}

pub fn decode(image_bytes: &Vec<u8>) -> Result<DecodedImage, QOIError> {
    let mut iter = image_bytes.iter();

    let header = QOIHeader::new(&mut iter)?;

    Ok(DecodedImage {
        image: None,
        errors: vec![],
    })
}

fn chunk<'a, I>(iter: &mut I, n: usize) -> Vec<u8>
where
    I: Iterator<Item = &'a u8>,
{
    iter.take(n).copied().collect()
}
