mod diff_handler;
mod image_buffer;
mod normal_handler;
mod qoi_encoder;
mod run_handler;
mod seen_handler;

use diff_handler::DiffHandler;
use image_buffer::ImageBuffer;
use normal_handler::NormalHandler;
use run_handler::RunHandler;
use seen_handler::SeenHandler;

pub use qoi_encoder::encode;
