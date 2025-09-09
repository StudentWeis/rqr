use crate::qr::encoder::QrEncoder;
use crate::qr::output::OutputFormat;
use crate::utils::error::Result;
use std::path::PathBuf;

/// Run the encode command to generate a QR code
///
/// This function handles the complete QR code encoding workflow:
/// 1. Creates a QR encoder with specified parameters
/// 2. Encodes the content into a QR code
/// 3. Outputs the result to file or terminal
///
/// # Arguments
/// * `content` - The text content to encode
/// * `output` - Path where to save the QR code image
/// * `size` - Size of the output image in pixels
/// * `error_correction` - Error correction level ("L", "M", "Q", "H")
/// * `margin` - Margin around the QR code in modules
/// * `terminal` - If true, display in terminal instead of saving to file
///
/// # Returns
/// Returns `Ok(())` on success, or an error if encoding fails
///
/// # Examples
/// ```rust,no_run
/// use std::path::PathBuf;
///
/// run(
///     "Hello World".to_string(),
///     PathBuf::from("hello.png"),
///     200,
///     "M".to_string(),
///     10,
///     false
/// )?;
/// # Ok::<(), rqr::RqrError>(())
/// ```
pub fn run(
    content: String,
    output: PathBuf,
    size: u32,
    error_correction: String,
    margin: u32,
    terminal: bool,
) -> Result<()> {
    // Create encoder
    let encoder = QrEncoder::new(size, margin, &error_correction)?;

    // Encode QR code
    let qr_code = encoder.encode(&content)?;

    // Determine output format
    let output_format = if terminal {
        OutputFormat::Terminal
    } else {
        OutputFormat::from_path(&output)?
    };

    match output_format {
        OutputFormat::Terminal => {
            let qr_string = encoder.to_terminal_string(&qr_code);
            println!("{}", qr_string);
            println!("\nContent: {}", content);
        }
        OutputFormat::Png => {
            encoder.save_to_file(&qr_code, &output)?;
            println!("QR code saved to: {}", output.display());
            println!("Content: {}", content);
            println!("Size: {}x{} pixels", size, size);
            println!("Error correction: {}", error_correction);
            println!("Margin: {} modules", margin);
        }
    }

    Ok(())
}
