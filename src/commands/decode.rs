use crate::qr::decoder::QrDecoder;
use crate::utils::error::Result;
use std::path::PathBuf;

/// Run the decode command to extract text from a QR code image
///
/// This function handles the complete QR code decoding workflow:
/// 1. Creates a QR decoder
/// 2. Loads and processes the image file or URL
/// 3. Extracts and displays the decoded content
///
/// # Arguments
/// * `input` - Path to the image file or URL containing the QR code
///
/// # Returns
/// Returns `Ok(())` on success, or an error if decoding fails
///
/// # Examples
/// ```rust,no_run
/// run("qr_code.png".to_string())?;
/// run("https://example.com/qr.png".to_string())?;
/// # Ok::<(), rqr::RqrError>(())
/// ```
pub fn run(input: String) -> Result<()> {
    let decoder = QrDecoder::new();

    println!("Decoding QR code from: {}", input);

    // 判断是否是 URL
    let results = if input.starts_with("http://") || input.starts_with("https://") {
        println!("Detected URL input, fetching from web...");
        decoder.decode_from_url(&input)?
    } else {
        println!("Detected file input, reading from disk...");
        let path = PathBuf::from(&input);
        decoder.decode_from_file(&path)?
    };

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
