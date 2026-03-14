use std::path::PathBuf;

use crate::{
    qr::{encoder::QrEncoder, output::OutputFormat},
    utils::error::Result,
};

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
/// use rqr::commands::encode::run;
///
/// run(
///     "Hello World".to_string(),
///     PathBuf::from("hello.png"),
///     200,
///     "M".to_string(),
///     10,
///     false
/// )?;
/// # Ok::<(), rqr::utils::error::RqrError>(())
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::test_utils::temp_dir;

    #[test]
    #[allow(clippy::unwrap_used)]
    fn test_encode_command_basic() {
        let temp = temp_dir();
        let output_path = temp.path().join("output.png");

        let result = run(
            "Test content".to_string(),
            output_path.clone(),
            200,
            "M".to_string(),
            10,
            false,
        );

        assert!(result.is_ok());
        assert!(output_path.exists());
    }

    #[test]
    #[allow(clippy::unwrap_used)]
    fn test_encode_command_different_sizes() {
        let temp = temp_dir();

        for size in [100, 200, 400] {
            let output_path = temp.path().join(format!("size_{}.png", size));

            let result = run(
                "Size test".to_string(),
                output_path.clone(),
                size,
                "M".to_string(),
                10,
                false,
            );

            assert!(result.is_ok(), "Failed with size {}", size);
            assert!(output_path.exists());
        }
    }

    #[test]
    #[allow(clippy::unwrap_used)]
    fn test_encode_command_different_error_correction() {
        let temp = temp_dir();

        for level in ["L", "M", "Q", "H"] {
            let output_path = temp.path().join(format!("ec_{}.png", level));

            let result = run(
                "EC test".to_string(),
                output_path.clone(),
                200,
                level.to_string(),
                10,
                false,
            );

            assert!(result.is_ok(), "Failed with level {}", level);
            assert!(output_path.exists());
        }
    }

    #[test]
    #[allow(clippy::unwrap_used)]
    fn test_encode_command_different_margins() {
        let temp = temp_dir();

        for margin in [0, 5, 10, 20] {
            let output_path = temp.path().join(format!("margin_{}.png", margin));

            let result = run(
                "Margin test".to_string(),
                output_path.clone(),
                200,
                "M".to_string(),
                margin,
                false,
            );

            assert!(result.is_ok(), "Failed with margin {}", margin);
        }
    }

    #[test]
    #[allow(clippy::unwrap_used)]
    fn test_encode_command_empty_content() {
        let temp = temp_dir();
        let output_path = temp.path().join("empty.png");

        let result = run(
            "".to_string(),
            output_path.clone(),
            200,
            "M".to_string(),
            10,
            false,
        );

        assert!(result.is_ok());
        assert!(output_path.exists());
    }

    #[test]
    #[allow(clippy::unwrap_used)]
    fn test_encode_command_unicode_content() {
        let temp = temp_dir();
        let output_path = temp.path().join("unicode.png");

        let result = run(
            "你好世界 🌍 Привет мир".to_string(),
            output_path.clone(),
            200,
            "M".to_string(),
            10,
            false,
        );

        assert!(result.is_ok());
        assert!(output_path.exists());
    }

    #[test]
    #[allow(clippy::unwrap_used)]
    fn test_encode_command_long_content() {
        let temp = temp_dir();
        let output_path = temp.path().join("long.png");

        let long_content = "a".repeat(500);

        let result = run(
            long_content,
            output_path.clone(),
            400,
            "M".to_string(),
            10,
            false,
        );

        assert!(result.is_ok());
        assert!(output_path.exists());
    }

    #[test]
    #[allow(clippy::unwrap_used)]
    fn test_encode_command_invalid_error_correction() {
        let temp = temp_dir();
        let output_path = temp.path().join("invalid.png");

        let result = run(
            "Test".to_string(),
            output_path.clone(),
            200,
            "X".to_string(),
            10,
            false,
        );

        assert!(result.is_err());
    }

    #[test]
    #[allow(clippy::unwrap_used)]
    fn test_encode_command_size_too_small() {
        let temp = temp_dir();
        let output_path = temp.path().join("small.png");

        let result = run(
            "Test".to_string(),
            output_path.clone(),
            10,
            "M".to_string(),
            10,
            false,
        );

        assert!(result.is_err());
    }

    #[test]
    #[allow(clippy::unwrap_used)]
    fn test_encode_command_terminal_mode() {
        let temp = temp_dir();
        let output_path = temp.path().join("terminal.png");

        let result = run(
            "Terminal test".to_string(),
            output_path.clone(),
            200,
            "M".to_string(),
            10,
            true,
        );

        assert!(result.is_ok());
        assert!(!output_path.exists());
    }

    #[test]
    #[allow(clippy::unwrap_used)]
    fn test_encode_command_nested_output_path() {
        let temp = temp_dir();
        let nested_path = temp.path().join("sub").join("dir").join("output.png");

        let result = run(
            "Test".to_string(),
            nested_path.clone(),
            200,
            "M".to_string(),
            10,
            false,
        );

        assert!(result.is_err());
    }

    #[test]
    #[allow(clippy::unwrap_used)]
    fn test_encode_command_special_chars_in_content() {
        let temp = temp_dir();
        let output_path = temp.path().join("special.png");

        let special_content = "!@#$%^&*()_+-=[]{}|;':\",./<>?";

        let result = run(
            special_content.to_string(),
            output_path.clone(),
            200,
            "M".to_string(),
            10,
            false,
        );

        assert!(result.is_ok());
        assert!(output_path.exists());
    }

    #[test]
    #[allow(clippy::unwrap_used)]
    fn test_encode_command_url_content() {
        let temp = temp_dir();
        let output_path = temp.path().join("url.png");

        let url = "https://example.com/path?query=value&foo=bar";

        let result = run(
            url.to_string(),
            output_path.clone(),
            200,
            "M".to_string(),
            10,
            false,
        );

        assert!(result.is_ok());
        assert!(output_path.exists());
    }
}
