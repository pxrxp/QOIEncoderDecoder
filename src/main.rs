mod decoder;
mod encoder;
mod errors;

fn main() -> Result<(), crate::errors::QOIError> {
    use crate::errors::QOIError;

    let args: Vec<String> = std::env::args().collect();
    let input_image_path = args.get(2).ok_or(QOIError::InvalidArgs(
        "Invalid arguments: Input image path not specified".to_owned(),
    ))?;
    let output_image_path = args.get(3).ok_or(QOIError::InvalidArgs(
        "Invalid arguments: Output image path not specified".to_owned(),
    ))?;

    match args.get(1).expect("Invalid no. of arguments").as_str() {
        "--encode" | "-e" => encoder::encode_file(&input_image_path)?.write(&output_image_path)?,

        "--decode" | "-d" => decoder::decode_file(&input_image_path)?
            .unwrap()
            .save(output_image_path)
            .map_err(|_| QOIError::FileWriteError)?,

        "--help" | "-h" => {}
        _ => panic!("Invalid command. Expected '--encode' or '--decode' or '--help'"),
    }

    Ok(())
}
