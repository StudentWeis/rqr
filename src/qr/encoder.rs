use std::path::Path;

use image::{DynamicImage, ImageBuffer, Luma};
use qrcode::{EcLevel, QrCode};

use crate::utils::error::{Result, RqrError};

/// QR Code encoder with configurable parameters
///
/// The `QrEncoder` handles the creation and rendering of QR codes.
/// It supports various output formats and customization options.
#[derive(Debug)]
pub struct QrEncoder {
    size: u32,
    margin: u32,
    error_correction: EcLevel,
}

impl QrEncoder {
    /// Create a new QR encoder with specified parameters
    ///
    /// # Arguments
    /// * `size` - The size of the output image in pixels
    /// * `margin` - The margin around the QR code in modules
    /// * `error_correction` - Error correction level ("L", "M", "Q", "H")
    ///
    /// # Returns
    /// Returns a configured `QrEncoder` or an error for invalid parameters
    ///
    /// # Examples
    /// ```rust
    /// use rqr::QrEncoder;
    ///
    /// let encoder = QrEncoder::new(200, 10, "M")?;
    /// # Ok::<(), rqr::RqrError>(())
    /// ```
    pub fn new(size: u32, margin: u32, error_correction: &str) -> Result<Self> {
        let ec_level = match error_correction.to_uppercase().as_str() {
            "L" => EcLevel::L,
            "M" => EcLevel::M,
            "Q" => EcLevel::Q,
            "H" => EcLevel::H,
            _ => {
                return Err(RqrError::InvalidInput(
                    "Error correction level must be L, M, Q, or H".to_string(),
                ));
            }
        };

        Ok(Self {
            size,
            margin,
            error_correction: ec_level,
        })
    }

    /// Encode text content into a QR code
    ///
    /// # Arguments
    /// * `content` - The text to encode
    ///
    /// # Returns
    /// Returns a `QrCode` structure or an error if encoding fails
    ///
    /// # Examples
    /// ```rust
    /// use crate::qr::encoder::QrEncoder;
    ///
    /// let encoder = QrEncoder::new(200, 10, "M")?;
    /// let qr_code = encoder.encode("Hello, World!")?;
    /// # Ok::<(), crate::utils::error::RqrError>(())
    /// ```
    pub fn encode(&self, content: &str) -> Result<QrCode> {
        QrCode::with_error_correction_level(content, self.error_correction)
            .map_err(|e| RqrError::EncodingError(e.to_string()))
    }

    /// Convert a QR code to an image
    ///
    /// # Arguments
    /// * `qr_code` - The QR code to render
    ///
    /// # Returns
    /// Returns a `DynamicImage` or an error if rendering fails
    pub fn to_image(&self, qr_code: &QrCode) -> Result<DynamicImage> {
        // Get the QR code as a vector of colors
        let qr_matrix = qr_code.to_colors();
        let qr_width = qr_code.width();

        // Calculate module size based on desired output size and margin
        let total_modules = qr_width + (self.margin as usize * 2);
        let module_size = (self.size as usize).checked_div(total_modules).unwrap_or(1);

        if module_size == 0 {
            return Err(RqrError::InvalidInput(
                "Size too small for the specified margin".to_string(),
            ));
        }

        // Create output image with margin
        let output_size = total_modules * module_size;
        let mut output_image: ImageBuffer<Luma<u8>, Vec<u8>> =
            ImageBuffer::new(output_size as u32, output_size as u32);

        // Fill with white background
        for pixel in output_image.pixels_mut() {
            *pixel = Luma([255u8]);
        }

        // Draw the QR code
        for (y, row) in qr_matrix.chunks(qr_width).enumerate() {
            for (x, &module) in row.iter().enumerate() {
                let color = match module {
                    qrcode::Color::Light => Luma([255u8]), // White
                    qrcode::Color::Dark => Luma([0u8]),    // Black
                };

                // Draw the module as a square block
                for dy in 0..module_size {
                    for dx in 0..module_size {
                        let out_x = (x + self.margin as usize) * module_size + dx;
                        let out_y = (y + self.margin as usize) * module_size + dy;

                        if out_x < output_size && out_y < output_size {
                            output_image.put_pixel(out_x as u32, out_y as u32, color);
                        }
                    }
                }
            }
        }

        Ok(DynamicImage::ImageLuma8(output_image))
    }

    pub fn save_to_file(&self, qr_code: &QrCode, path: &Path) -> Result<()> {
        let image = self.to_image(qr_code)?;
        image.save(path)?;
        Ok(())
    }

    pub fn to_terminal_string(&self, qr_code: &QrCode) -> String {
        qr_code
            .render::<char>()
            .quiet_zone(false)
            .module_dimensions(2, 1)
            .build()
    }
}

#[cfg(test)]
mod tests {
    use tempfile::TempDir;

    use super::*;

    #[test]
    fn test_encoder_new_with_valid_levels() {
        assert!(QrEncoder::new(200, 10, "L").is_ok());
        assert!(QrEncoder::new(200, 10, "M").is_ok());
        assert!(QrEncoder::new(200, 10, "Q").is_ok());
        assert!(QrEncoder::new(200, 10, "H").is_ok());
    }

    #[test]
    fn test_encoder_new_with_lowercase() {
        assert!(QrEncoder::new(200, 10, "l").is_ok());
        assert!(QrEncoder::new(200, 10, "m").is_ok());
        assert!(QrEncoder::new(200, 10, "q").is_ok());
        assert!(QrEncoder::new(200, 10, "h").is_ok());
    }

    #[test]
    fn test_encoder_new_with_invalid_level() {
        let result = QrEncoder::new(200, 10, "X");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("Error correction level must be"));
    }

    #[test]
    fn test_encoder_new_with_empty_level() {
        let result = QrEncoder::new(200, 10, "");
        assert!(result.is_err());
    }

    #[test]
    fn test_encoder_new_with_multi_char_level() {
        let result = QrEncoder::new(200, 10, "LM");
        assert!(result.is_err());
    }

    #[test]
    fn test_encode_simple_text() {
        let encoder = QrEncoder::new(200, 10, "M").unwrap();
        let result = encoder.encode("Hello, World!");
        assert!(result.is_ok());
    }

    #[test]
    fn test_encode_empty_string() {
        let encoder = QrEncoder::new(200, 10, "M").unwrap();
        let result = encoder.encode("");
        assert!(result.is_ok());
    }

    #[test]
    fn test_encode_long_text() {
        let encoder = QrEncoder::new(400, 10, "M").unwrap();
        let long_text = "a".repeat(100);
        let result = encoder.encode(&long_text);
        assert!(result.is_ok());
    }

    #[test]
    fn test_encode_unicode() {
        let encoder = QrEncoder::new(200, 10, "M").unwrap();
        let result = encoder.encode("你好世界 🌍 Привет мир");
        assert!(result.is_ok());
    }

    #[test]
    fn test_encode_special_characters() {
        let encoder = QrEncoder::new(200, 10, "M").unwrap();
        let result = encoder.encode("!@#$%^&*()_+-=[]{}|;':\",./<>?");
        assert!(result.is_ok());
    }

    #[test]
    fn test_encode_url() {
        let encoder = QrEncoder::new(200, 10, "M").unwrap();
        let url = "https://example.com/path?query=value&foo=bar";
        let result = encoder.encode(url);
        assert!(result.is_ok());
    }

    #[test]
    fn test_to_image_basic() {
        let encoder = QrEncoder::new(200, 10, "M").unwrap();
        let qr_code = encoder.encode("Test").unwrap();
        let image = encoder.to_image(&qr_code);
        assert!(image.is_ok());

        let img = image.unwrap();
        assert_eq!(img.width(), img.height());
    }

    #[test]
    fn test_to_image_different_sizes() {
        for size in [100, 200, 400, 800] {
            let encoder = QrEncoder::new(size, 10, "M").unwrap();
            let qr_code = encoder.encode("Size test").unwrap();
            let image = encoder.to_image(&qr_code).unwrap();
            assert!(image.width() > 0);
            assert!(image.height() > 0);
        }
    }

    #[test]
    fn test_to_image_different_margins() {
        for margin in [0, 5, 10, 20] {
            let encoder = QrEncoder::new(200, margin, "M").unwrap();
            let qr_code = encoder.encode("Margin test").unwrap();
            let image = encoder.to_image(&qr_code);
            assert!(image.is_ok());
        }
    }

    #[test]
    fn test_to_image_size_too_small() {
        let encoder = QrEncoder::new(10, 10, "M").unwrap();
        let qr_code = encoder.encode("Test").unwrap();
        let result = encoder.to_image(&qr_code);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("Size too small"));
    }

    #[test]
    fn test_save_to_file() {
        let temp_dir = TempDir::new().unwrap();
        let output_path = temp_dir.path().join("test_qr.png");

        let encoder = QrEncoder::new(200, 10, "M").unwrap();
        let qr_code = encoder.encode("Save test").unwrap();

        let result = encoder.save_to_file(&qr_code, &output_path);
        assert!(result.is_ok());
        assert!(output_path.exists());

        let loaded = image::open(&output_path);
        assert!(loaded.is_ok());
    }

    #[test]
    fn test_save_to_file_nested_directory() {
        let temp_dir = TempDir::new().unwrap();
        let nested_path = temp_dir.path().join("sub").join("dir").join("qr.png");

        let encoder = QrEncoder::new(200, 10, "M").unwrap();
        let qr_code = encoder.encode("Nested test").unwrap();

        let result = encoder.save_to_file(&qr_code, &nested_path);
        assert!(result.is_err());
    }

    #[test]
    fn test_to_terminal_string() {
        let encoder = QrEncoder::new(200, 10, "M").unwrap();
        let qr_code = encoder.encode("Terminal test").unwrap();

        let output = encoder.to_terminal_string(&qr_code);
        assert!(!output.is_empty());
        assert!(
            output.contains('█')
                || output.contains('▀')
                || output.contains('▄')
                || output.contains(' ')
        );
    }

    #[test]
    fn test_to_terminal_string_empty_content() {
        let encoder = QrEncoder::new(200, 10, "M").unwrap();
        let qr_code = encoder.encode("").unwrap();

        let output = encoder.to_terminal_string(&qr_code);
        assert!(!output.is_empty());
    }

    #[test]
    fn test_encode_with_different_error_correction() {
        let content = "Error correction test";

        for level in ["L", "M", "Q", "H"] {
            let encoder = QrEncoder::new(200, 10, level).unwrap();
            let result = encoder.encode(content);
            assert!(result.is_ok());
        }
    }

    #[test]
    fn test_encode_binary_data() {
        let encoder = QrEncoder::new(200, 10, "M").unwrap();
        let binary_content = vec![0u8, 1, 2, 255, 128, 64];
        let result = encoder.encode(&String::from_utf8_lossy(&binary_content));
        assert!(result.is_ok());
    }

    #[test]
    fn test_save_to_file_overwrite() {
        let temp_dir = TempDir::new().unwrap();
        let output_path = temp_dir.path().join("overwrite.png");

        let encoder = QrEncoder::new(200, 10, "M").unwrap();

        let qr_code1 = encoder.encode("First").unwrap();
        encoder.save_to_file(&qr_code1, &output_path).unwrap();

        let qr_code2 = encoder.encode("Second content that is longer").unwrap();
        encoder.save_to_file(&qr_code2, &output_path).unwrap();

        assert!(output_path.exists());
    }
}
