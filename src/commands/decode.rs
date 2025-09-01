use crate::qr::decoder::QrDecoder;
use crate::utils::error::Result;
use std::path::PathBuf;

pub fn run(input: PathBuf) -> Result<()> {
    let decoder = QrDecoder::new();

    println!("Decoding QR code from: {}", input.display());

    let results = decoder.decode_from_file(&input)?;

    if results.len() == 1 {
        println!("\nDecoded content:");
        println!("{}", results[0]);
    } else {
        println!("\nFound {} QR codes:", results.len());
        for (i, content) in results.iter().enumerate() {
            println!("\nQR Code #{}:", i + 1);
            println!("{}", content);
        }
    }

    Ok(())
}
