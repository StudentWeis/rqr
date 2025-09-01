use crate::utils::error::{Result, RqrError};
use image::{DynamicImage, ImageBuffer, Luma};
use qrcode::{EcLevel, QrCode};
use std::path::Path;

pub struct QrEncoder {
    size: u32,
    margin: u32,
    error_correction: EcLevel,
}

impl QrEncoder {
    pub fn new(size: u32, margin: u32, error_correction: &str) -> Result<Self> {
        let ec_level = match error_correction.to_uppercase().as_str() {
            "L" => EcLevel::L,
            "M" => EcLevel::M,
            "Q" => EcLevel::Q,
            "H" => EcLevel::H,
            _ => {
                return Err(RqrError::InvalidInput(
                    "Error correction level must be L, M, Q, or H".to_string(),
                ))
            }
        };

        Ok(Self {
            size,
            margin,
            error_correction: ec_level,
        })
    }

    pub fn encode(&self, content: &str) -> Result<QrCode> {
        QrCode::with_error_correction_level(content, self.error_correction)
            .map_err(|e| RqrError::EncodingError(e.to_string()))
    }

    pub fn to_image(&self, qr_code: &QrCode) -> Result<DynamicImage> {
        // Get the QR code as a vector of colors
        let qr_matrix = qr_code.to_colors();
        let qr_width = qr_code.width();

        // Calculate module size based on desired output size and margin
        let total_modules = qr_width + (self.margin as usize * 2);
        let module_size = if total_modules > 0 {
            (self.size as usize) / total_modules
        } else {
            1
        };

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
