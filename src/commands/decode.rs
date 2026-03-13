use std::path::PathBuf;

use crate::{qr::decoder::QrDecoder, utils::error::Result};

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
/// use crate::commands::decode::run;
///
/// run("qr_code.png".to_string())?;
/// run("https://example.com/qr.png".to_string())?;
/// # Ok::<(), crate::utils::error::RqrError>(())
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

#[cfg(test)]
mod tests {
    use std::path::Path;

    use tempfile::TempDir;

    use super::*;
    use crate::qr::encoder::QrEncoder;

    fn create_test_qr_file(path: &Path, content: &str) {
        let encoder = QrEncoder::new(200, 10, "M").unwrap();
        let qr_code = encoder.encode(content).unwrap();
        encoder.save_to_file(&qr_code, path).unwrap();
    }

    #[test]
    #[allow(clippy::unwrap_used)]
    fn test_decode_command_basic() {
        let temp_dir = TempDir::new().unwrap();
        let image_path = temp_dir.path().join("test.png");

        create_test_qr_file(&image_path, "Hello from decode");

        let result = run(image_path.to_str().unwrap().to_string());
        assert!(result.is_ok());
    }

    #[test]
    #[allow(clippy::unwrap_used)]
    fn test_decode_command_empty_content() {
        let temp_dir = TempDir::new().unwrap();
        let image_path = temp_dir.path().join("empty.png");

        create_test_qr_file(&image_path, "");

        let result = run(image_path.to_str().unwrap().to_string());
        assert!(result.is_ok());
    }

    #[test]
    #[allow(clippy::unwrap_used)]
    fn test_decode_command_unicode() {
        let temp_dir = TempDir::new().unwrap();
        let image_path = temp_dir.path().join("unicode.png");

        let content = "你好世界 🌍 Привет мир";
        create_test_qr_file(&image_path, content);

        let result = run(image_path.to_str().unwrap().to_string());
        assert!(result.is_ok());
    }

    #[test]
    #[allow(clippy::unwrap_used)]
    fn test_decode_command_long_text() {
        let temp_dir = TempDir::new().unwrap();
        let image_path = temp_dir.path().join("long.png");

        let long_text = "a".repeat(200);
        create_test_qr_file(&image_path, &long_text);

        let result = run(image_path.to_str().unwrap().to_string());
        assert!(result.is_ok());
    }

    #[test]
    #[allow(clippy::unwrap_used)]
    fn test_decode_command_special_chars() {
        let temp_dir = TempDir::new().unwrap();
        let image_path = temp_dir.path().join("special.png");

        let special = "!@#$%^&*()_+-=[]{}|;':\",./<>?";
        create_test_qr_file(&image_path, special);

        let result = run(image_path.to_str().unwrap().to_string());
        assert!(result.is_ok());
    }

    #[test]
    #[allow(clippy::unwrap_used)]
    fn test_decode_command_file_not_found() {
        let result = run("/nonexistent/path/qr.png".to_string());
        assert!(result.is_err());
    }

    #[test]
    #[allow(clippy::unwrap_used)]
    fn test_decode_command_invalid_image() {
        let temp_dir = TempDir::new().unwrap();
        let invalid_path = temp_dir.path().join("not_an_image.txt");

        std::fs::write(&invalid_path, "This is not an image").unwrap();

        let result = run(invalid_path.to_str().unwrap().to_string());
        assert!(result.is_err());
    }

    #[test]
    #[allow(clippy::unwrap_used)]
    fn test_decode_command_url_format() {
        let result_http = run("http://example.com/qr.png".to_string());
        assert!(result_http.is_err());

        let result_https = run("https://example.com/qr.png".to_string());
        assert!(result_https.is_err());
    }

    #[test]
    #[allow(clippy::unwrap_used)]
    fn test_decode_command_different_qr_sizes() {
        let temp_dir = TempDir::new().unwrap();

        for size in [100, 200, 400] {
            let image_path = temp_dir.path().join(format!("size_{}.png", size));

            let encoder = QrEncoder::new(size, 10, "M").unwrap();
            let qr_code = encoder.encode("Size test").unwrap();
            encoder.save_to_file(&qr_code, &image_path).unwrap();

            let result = run(image_path.to_str().unwrap().to_string());
            assert!(result.is_ok(), "Failed with size {}", size);
        }
    }

    #[test]
    #[allow(clippy::unwrap_used)]
    fn test_decode_command_different_error_correction() {
        let temp_dir = TempDir::new().unwrap();

        for level in ["L", "M", "Q", "H"] {
            let image_path = temp_dir.path().join(format!("ec_{}.png", level));

            let encoder = QrEncoder::new(200, 10, level).unwrap();
            let qr_code = encoder.encode("EC test").unwrap();
            encoder.save_to_file(&qr_code, &image_path).unwrap();

            let result = run(image_path.to_str().unwrap().to_string());
            assert!(result.is_ok(), "Failed with level {}", level);
        }
    }

    #[test]
    #[allow(clippy::unwrap_used)]
    fn test_decode_command_url_content() {
        let temp_dir = TempDir::new().unwrap();
        let image_path = temp_dir.path().join("url_content.png");

        let url = "https://example.com/path?query=value&foo=bar";
        create_test_qr_file(&image_path, url);

        let result = run(image_path.to_str().unwrap().to_string());
        assert!(result.is_ok());
    }

    #[test]
    #[allow(clippy::unwrap_used)]
    fn test_decode_command_with_spaces_in_content() {
        let temp_dir = TempDir::new().unwrap();
        let image_path = temp_dir.path().join("spaces.png");

        let content = "This is a test with   multiple   spaces";
        create_test_qr_file(&image_path, content);

        let result = run(image_path.to_str().unwrap().to_string());
        assert!(result.is_ok());
    }

    #[test]
    #[allow(clippy::unwrap_used)]
    fn test_decode_command_multiline_content() {
        let temp_dir = TempDir::new().unwrap();
        let image_path = temp_dir.path().join("multiline.png");

        let content = "Line 1\nLine 2\nLine 3";
        create_test_qr_file(&image_path, content);

        let result = run(image_path.to_str().unwrap().to_string());
        assert!(result.is_ok());
    }
}
