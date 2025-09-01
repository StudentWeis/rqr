use crate::utils::error::{Result, RqrError};
use image::{open as open_image, DynamicImage};
use rqrr::PreparedImage;
use std::path::Path;

#[derive(Default)]
pub struct QrDecoder;

impl QrDecoder {
    pub fn new() -> Self {
        Self
    }

    pub fn decode_from_file<P: AsRef<Path>>(&self, path: P) -> Result<Vec<String>> {
        let img = open_image(path)?;
        self.decode_from_image(img)
    }

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
