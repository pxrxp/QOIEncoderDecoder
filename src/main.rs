use image::{DynamicImage, GenericImageView, ImageReader, Rgba};
use std::{env, error::Error};

const QOI_OP_RGB_TAG: u8 = 0b11111110;
const QOI_OP_RGBA_TAG: u8 = 0b11111111;
const QOI_OP_INDEX_TAG: u8 = 0b00 << 7;
const QOI_OP_DIFF_TAG: u8 = 0b01 << 7;
const QOI_OP_LUMA_TAG: u8 = 0b10 << 7;
const QOI_OP_RUN_TAG: u8 = 0b11 << 7;

fn add_header(image: &DynamicImage, buffer: &mut Vec<u8>) {
    let magic: [u8; 4] = *b"qoif";
    let width: [u8; 4] = image.width().to_ne_bytes();
    let height: [u8; 4] = image.height().to_ne_bytes();
    let channels: u8 = if image.has_alpha() { 4 } else { 3 };
    let colorspace: u8 = match image {
        DynamicImage::ImageRgb32F(_) | DynamicImage::ImageRgba32F(_) => 1,
        _ => 0,
    };

    buffer.extend_from_slice(&magic);
    buffer.extend_from_slice(&width);
    buffer.extend_from_slice(&height);
    buffer.push(channels);
    buffer.push(colorspace);
}

fn end_byte_stream(buffer: &mut Vec<u8>) {
    buffer.push(0x01);
    buffer.extend_from_slice(&[0x00; 7]);
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    match args.get(1).expect("Invalid no. of arguments").as_str() {
        "--encode" | "-e" => {
            let image =
                ImageReader::open(args.get(2).expect("Image file not provided."))?.decode()?;

            let (w, h): (u32, u32) = image.dimensions();
            let mut buffer: Vec<u8> = Vec::with_capacity((w * h * 4) as usize);

            add_header(&image, &mut buffer);

            let mut prev_pixel: Rgba<u8> = Rgba([0, 0, 0, 255]);
            let mut seen_pixels: Vec<Rgba<u8>> = Vec::with_capacity(64);
            let mut run_length: u8 = 0;

            for (_x, _y, Rgba([r, g, b, a])) in image.pixels() {}

            end_byte_stream(&mut buffer);
        }

        "--decode" | "-d" => {}
        "--help" | "-h" => {}
        _ => panic!("Invalid command. Expected '--encode' or '--decode' or '--help'"),
    }

    Ok(())
}
