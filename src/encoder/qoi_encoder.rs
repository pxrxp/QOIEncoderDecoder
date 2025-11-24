use super::{DiffHandler, ImageBuffer, NormalHandler, RunHandler, SeenHandler};

use image::{DynamicImage, GenericImageView, ImageReader};

pub fn encode_file(image_path: &str) -> ImageBuffer {
    let reader = ImageReader::open(image_path).expect("Couldn't open file.");
    let image = reader.decode().expect("Couldn't decode provided file.");
    encode(&image)
}

pub fn encode(image: &DynamicImage) -> ImageBuffer {
    let mut qoi_buffer = ImageBuffer::new(&image);
    let mut run_handler = RunHandler::new();
    let mut seen_handler = SeenHandler::new();
    let mut diff_handler = DiffHandler::new();
    let mut normal_handler = NormalHandler::new(image.has_alpha());

    for (_, _, pixel) in image.pixels() {
        let mut handled = false;
        run_handler.handle(&mut qoi_buffer, &pixel, &mut handled);
        seen_handler.handle(&mut qoi_buffer, &pixel, &mut handled);
        diff_handler.handle(&mut qoi_buffer, &pixel, &mut handled);
        normal_handler.handle(&mut qoi_buffer, &pixel, &mut handled);
    }

    run_handler.cleanup(&mut qoi_buffer);

    qoi_buffer.end_byte_stream();
    qoi_buffer
}
