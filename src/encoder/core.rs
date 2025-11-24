use crate::{
    QOIError, QOIState,
    encoder::{ImageBuffer, diff_handler, normal_handler, run_handler, seen_handler},
};
use image::{DynamicImage, GenericImageView, ImageReader};

pub fn encode_file(image_path: &str) -> Result<ImageBuffer, QOIError> {
    let reader = ImageReader::open(image_path).map_err(|_| QOIError::FileReadError)?;
    let image = reader
        .decode()
        .map_err(|_| QOIError::ImageDecodeError("Couldn't decode the input image".to_owned()))?;
    encode(&image)
}

pub fn encode(image: &DynamicImage) -> Result<ImageBuffer, QOIError> {
    let mut qoi_buffer = ImageBuffer::new(&image);
    let mut state = QOIState::new();

    for (_, _, pixel) in image.pixels() {
        let mut handled = false;

        if !handled {
            handled = run_handler::handle(&mut qoi_buffer, &mut state, &pixel);
        };
        if !handled {
            handled = seen_handler::handle(&mut qoi_buffer, &mut state, &pixel);
        };
        if !handled {
            handled = diff_handler::handle(&mut qoi_buffer, &mut state, &pixel);
        };
        if !handled {
            normal_handler::handle(&mut qoi_buffer, &mut state, &pixel);
        };

        state.update(&pixel);
    }

    run_handler::cleanup(&mut qoi_buffer, &mut state);
    qoi_buffer.end_byte_stream();

    Ok(qoi_buffer)
}
