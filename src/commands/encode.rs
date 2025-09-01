use crate::qr::encoder::QrEncoder;
use crate::qr::output::OutputFormat;
use crate::utils::error::Result;
use std::path::PathBuf;

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
