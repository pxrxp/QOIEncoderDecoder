use image::{GenericImageView, ImageReader, Rgba};
use std::{env, error::Error};

mod encoder;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let input_image = args.get(2).expect("Image file not provided.");
    let output_image = args.get(3).expect("Image file not provided.");

    match args.get(1).expect("Invalid no. of arguments").as_str() {
        "--encode" | "-e" => encoder::encode(&input_image).write(&output_image),

        "--decode" | "-d" => {}
        "--help" | "-h" => {}
        _ => panic!("Invalid command. Expected '--encode' or '--decode' or '--help'"),
    }

    Ok(())
}
