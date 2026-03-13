use std::path::Path;

use image::{DynamicImage, open as open_image};
use rqrr::PreparedImage;

use crate::utils::error::{Result, RqrError};

/// QR Code decoder for extracting text from images
///
/// The `QrDecoder` handles the detection and decoding of QR codes
/// from image files. It supports various image formats and can
/// detect multiple QR codes in a single image.
#[derive(Default)]
pub struct QrDecoder;

impl QrDecoder {
    /// Create a new QR decoder
    ///
    /// # Returns
    /// Returns a configured `QrDecoder`
    ///
    /// # Examples
    /// ```rust
    /// use crate::qr::decoder::QrDecoder;
    ///
    /// let decoder = QrDecoder::new();
    /// ```
    pub fn new() -> Self {
        Self
    }

    /// Decode QR codes from an image url
    pub fn decode_from_url(&self, url: &str) -> Result<Vec<String>> {
        let resp = reqwest::blocking::get(url).map_err(|e| {
            RqrError::DecodingError(format!("Failed to fetch image from URL: {}", e))
        })?;
        let bytes = resp.bytes().map_err(|e| {
            RqrError::DecodingError(format!("Failed to read image bytes from response: {}", e))
        })?;
        let img = image::load_from_memory(&bytes)?;
        self.decode_from_image(img)
    }

    /// Decode QR codes from an image file
    ///
    /// # Arguments
    /// * `path` - Path to the image file
    ///
    /// # Returns
    /// Returns a vector of decoded strings, or an error if decoding fails
    ///
    /// # Examples
    /// ```rust,no_run
    /// use crate::qr::decoder::QrDecoder;
    ///
    /// let decoder = QrDecoder::new();
    /// let contents = decoder.decode_from_file("qr_code.png")?;
    /// for content in contents {
    ///     println!("Decoded: {}", content);
    /// }
    /// # Ok::<(), crate::utils::error::RqrError>(())
    /// ```
    pub fn decode_from_file<P: AsRef<Path>>(&self, path: P) -> Result<Vec<String>> {
        let img = open_image(path)?;
        self.decode_from_image(img)
    }

    /// Decode QR codes from an image buffer
    ///
    /// # Arguments
    /// * `img` - The image to decode
    ///
    /// # Returns
    /// Returns a vector of decoded strings, or an error if decoding fails
    pub fn decode_from_image(&self, img: DynamicImage) -> Result<Vec<String>> {
        // Convert to grayscale
        let luma_img = img.to_luma8();

        // Get width and height
        let width = luma_img.width() as usize;
        let height = luma_img.height() as usize;

        // Prepare the image for QR code detection using a closure
        let mut prepared_img = PreparedImage::prepare_from_greyscale(width, height, |x, y| {
            let pixel = luma_img.get_pixel(x as u32, y as u32);
            pixel[0]
        });

        // Find and decode all QR codes in the image
        let grids = prepared_img.detect_grids();

        if grids.is_empty() {
            return Err(RqrError::DecodingError(
                "No QR codes found in image".to_string(),
            ));
        }

        let mut results = Vec::new();

        for grid in grids {
            match grid.decode() {
                Ok((meta, content)) => {
                    let content_str = match std::str::from_utf8(content.as_bytes()) {
                        Ok(s) => s.to_string(),
                        Err(_) => {
                            // If not valid UTF-8, show as hex
                            format!("Binary data: {}", hex::encode(content.as_bytes()))
                        }
                    };
                    results.push(content_str);

                    // Print metadata for debugging if needed
                    eprintln!("QR Code metadata: {:?}", meta);
                }
                Err(e) => {
                    eprintln!("Failed to decode QR code: {:?}", e);
                    continue;
                }
            }
        }

        if results.is_empty() {
            return Err(RqrError::DecodingError(
                "Found QR codes but failed to decode any".to_string(),
            ));
        }

        Ok(results)
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use image::{DynamicImage, ImageBuffer, Luma};
    use tempfile::TempDir;

    use super::*;
    use crate::qr::encoder::QrEncoder;

    fn create_test_qr_image(content: &str, path: &Path) {
        let encoder = QrEncoder::new(200, 10, "M").unwrap();
        let qr_code = encoder.encode(content).unwrap();
        encoder.save_to_file(&qr_code, path).unwrap();
    }

    fn create_test_qr_image_buffer(content: &str) -> DynamicImage {
        let encoder = QrEncoder::new(200, 10, "M").unwrap();
        let qr_code = encoder.encode(content).unwrap();
        encoder.to_image(&qr_code).unwrap()
    }

    #[test]
    fn test_decoder_new() {
        let _decoder = QrDecoder::new();
    }

    #[test]
    fn test_decoder_default() {
        let _decoder: QrDecoder = Default::default();
    }

    #[test]
    fn test_decode_from_file_basic() {
        let temp_dir = TempDir::new().unwrap();
        let image_path = temp_dir.path().join("test.png");

        create_test_qr_image("Hello from QR", &image_path);

        let decoder = QrDecoder::new();
        let results = decoder.decode_from_file(&image_path).unwrap();

        assert_eq!(results.len(), 1);
        assert_eq!(results[0], "Hello from QR");
    }

    #[test]
    fn test_decode_from_file_empty_content() {
        let temp_dir = TempDir::new().unwrap();
        let image_path = temp_dir.path().join("empty.png");

        create_test_qr_image("", &image_path);

        let decoder = QrDecoder::new();
        let results = decoder.decode_from_file(&image_path).unwrap();

        assert_eq!(results.len(), 1);
        assert_eq!(results[0], "");
    }

    #[test]
    fn test_decode_from_file_unicode() {
        let temp_dir = TempDir::new().unwrap();
        let image_path = temp_dir.path().join("unicode.png");

        let content = "你好世界 🌍 Привет мир";
        create_test_qr_image(content, &image_path);

        let decoder = QrDecoder::new();
        let results = decoder.decode_from_file(&image_path).unwrap();

        assert_eq!(results.len(), 1);
        assert_eq!(results[0], content);
    }

    #[test]
    fn test_decode_from_file_url() {
        let temp_dir = TempDir::new().unwrap();
        let image_path = temp_dir.path().join("url.png");

        let url = "https://example.com/path?query=value&foo=bar";
        create_test_qr_image(url, &image_path);

        let decoder = QrDecoder::new();
        let results = decoder.decode_from_file(&image_path).unwrap();

        assert_eq!(results.len(), 1);
        assert_eq!(results[0], url);
    }

    #[test]
    fn test_decode_from_file_long_text() {
        let temp_dir = TempDir::new().unwrap();
        let image_path = temp_dir.path().join("long.png");

        let long_text = "a".repeat(200);
        create_test_qr_image(&long_text, &image_path);

        let decoder = QrDecoder::new();
        let results = decoder.decode_from_file(&image_path).unwrap();

        assert_eq!(results.len(), 1);
        assert_eq!(results[0], long_text);
    }

    #[test]
    fn test_decode_from_file_not_found() {
        let decoder = QrDecoder::new();
        let result = decoder.decode_from_file("/nonexistent/path/qr.png");
        assert!(result.is_err());
    }

    #[test]
    fn test_decode_from_image_basic() {
        let image = create_test_qr_image_buffer("Test from buffer");

        let decoder = QrDecoder::new();
        let results = decoder.decode_from_image(image).unwrap();

        assert_eq!(results.len(), 1);
        assert_eq!(results[0], "Test from buffer");
    }

    #[test]
    fn test_decode_from_image_no_qr() {
        let blank_image =
            DynamicImage::ImageLuma8(ImageBuffer::from_pixel(100, 100, Luma([255u8])));

        let decoder = QrDecoder::new();
        let result = decoder.decode_from_image(blank_image);

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("No QR codes found"));
    }

    #[test]
    fn test_decode_from_file_special_chars() {
        let temp_dir = TempDir::new().unwrap();
        let image_path = temp_dir.path().join("special.png");

        let special = "!@#$%^&*()_+-=[]{}|;':\",./<>?";
        create_test_qr_image(special, &image_path);

        let decoder = QrDecoder::new();
        let results = decoder.decode_from_file(&image_path).unwrap();

        assert_eq!(results.len(), 1);
        assert_eq!(results[0], special);
    }

    #[test]
    fn test_decode_from_image_different_sizes() {
        for size in [100, 200, 400] {
            let encoder = QrEncoder::new(size, 10, "M").unwrap();
            let qr_code = encoder.encode("Size variation").unwrap();
            let image = encoder.to_image(&qr_code).unwrap();

            let decoder = QrDecoder::new();
            let results = decoder.decode_from_image(image).unwrap();

            assert_eq!(results.len(), 1);
            assert_eq!(results[0], "Size variation");
        }
    }

    #[test]
    fn test_decode_from_image_different_error_correction() {
        for level in ["L", "M", "Q", "H"] {
            let encoder = QrEncoder::new(200, 10, level).unwrap();
            let qr_code = encoder.encode("EC test").unwrap();
            let image = encoder.to_image(&qr_code).unwrap();

            let decoder = QrDecoder::new();
            let results = decoder.decode_from_image(image).unwrap();

            assert_eq!(results.len(), 1);
            assert_eq!(results[0], "EC test");
        }
    }
}
