//! Shared test utilities for rqr
//!
//! This module provides common helper functions and fixtures used across
//! the test suite to reduce code duplication and ensure consistency.

use std::path::Path;

use image::DynamicImage;
use rqr::qr::encoder::QrEncoder;
use tempfile::TempDir;

/// Creates a temporary directory for test files
///
/// # Returns
/// A `TempDir` that will be automatically cleaned up when dropped
pub fn temp_dir() -> TempDir {
    TempDir::new().expect("Failed to create temp directory")
}

/// Creates a QR code image file with the given content
///
/// # Arguments
/// * `path` - Where to save the QR code image
/// * `content` - The text content to encode
///
/// # Panics
/// Panics if encoding or saving fails
pub fn create_qr_image_file(path: &Path, content: &str) {
    let encoder = QrEncoder::new(200, 10, "M").expect("Failed to create encoder");
    let qr_code = encoder.encode(content).expect("Failed to encode content");
    encoder
        .save_to_file(&qr_code, path)
        .expect("Failed to save QR image");
}

/// Creates a QR code image buffer with the given content
///
/// # Arguments
/// * `content` - The text content to encode
///
/// # Returns
/// A `DynamicImage` containing the QR code
///
/// # Panics
/// Panics if encoding fails
pub fn create_qr_image_buffer(content: &str) -> DynamicImage {
    let encoder = QrEncoder::new(200, 10, "M").expect("Failed to create encoder");
    let qr_code = encoder.encode(content).expect("Failed to encode content");
    encoder
        .to_image(&qr_code)
        .expect("Failed to render QR image")
}

/// Creates a QR code image file with custom parameters
///
/// # Arguments
/// * `path` - Where to save the QR code image
/// * `content` - The text content to encode
/// * `size` - Image size in pixels
/// * `margin` - Margin in modules
/// * `error_correction` - Error correction level ("L", "M", "Q", "H")
///
/// # Panics
/// Panics if encoding or saving fails
pub fn create_qr_image_file_with_params(
    path: &Path,
    content: &str,
    size: u32,
    margin: u32,
    error_correction: &str,
) {
    let encoder = QrEncoder::new(size, margin, error_correction).expect("Failed to create encoder");
    let qr_code = encoder.encode(content).expect("Failed to encode content");
    encoder
        .save_to_file(&qr_code, path)
        .expect("Failed to save QR image");
}

/// Returns the binary command for CLI testing
///
/// # Returns
/// A `Command` configured to run the rqr binary
#[cfg(test)]
pub fn cmd() -> assert_cmd::Command {
    assert_cmd::Command::cargo_bin("rqr").expect("Failed to find rqr binary")
}

/// Common test content constants
pub mod fixtures {
    /// Unicode content with multiple scripts
    pub const UNICODE_CONTENT: &str = "你好世界 🌍 Привет мир";

    /// Special characters that need escaping
    pub const SPECIAL_CHARS: &str = "!@#$%^&*()_+-=[]{}|;':\",./<>?";

    /// URL with query parameters
    pub const URL_CONTENT: &str = "https://example.com/path?query=value&foo=bar";

    /// Multiline content
    pub const MULTILINE_CONTENT: &str = "Line 1\nLine 2\nLine 3";

    /// Content with tabs
    pub const TAB_CONTENT: &str = "Column1\tColumn2\tColumn3";

    /// Empty string
    pub const EMPTY: &str = "";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_temp_dir_creation() {
        let temp = temp_dir();
        assert!(temp.path().exists());
    }

    #[test]
    fn test_create_qr_image_file() {
        let temp = temp_dir();
        let path = temp.path().join("test.png");

        create_qr_image_file(&path, "Test content");

        assert!(path.exists());
        let img = image::open(&path).expect("Failed to open image");
        assert_eq!(img.width(), img.height());
    }

    #[test]
    fn test_create_qr_image_buffer() {
        let img = create_qr_image_buffer("Buffer test");

        assert!(img.width() > 0);
        assert!(img.height() > 0);
    }

    #[test]
    fn test_create_qr_with_params() {
        let temp = temp_dir();
        let path = temp.path().join("custom.png");

        create_qr_image_file_with_params(&path, "Custom", 400, 20, "H");

        assert!(path.exists());
    }

    #[test]
    fn test_fixtures_available() {
        assert!(!fixtures::UNICODE_CONTENT.is_empty());
        assert!(!fixtures::SPECIAL_CHARS.is_empty());
        assert!(!fixtures::URL_CONTENT.is_empty());
        assert!(!fixtures::MULTILINE_CONTENT.is_empty());
        assert!(!fixtures::TAB_CONTENT.is_empty());
        assert_eq!(fixtures::EMPTY, "");
    }
}
