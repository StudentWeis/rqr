use thiserror::Error;

/// Comprehensive error type for the rqr library
///
/// This enum covers all possible error conditions that can occur
/// during QR code encoding, decoding, and file operations.
#[derive(Error, Debug)]
pub enum RqrError {
    /// Failed to encode content into a QR code
    #[error("QR code encoding failed: {0}")]
    EncodingError(String),

    /// Failed to decode a QR code from an image
    #[error("QR code decoding failed: {0}")]
    DecodingError(String),

    /// Error occurred during image processing
    #[error("Image processing error: {0}")]
    ImageError(#[from] image::ImageError),

    /// File input/output operation failed
    #[error("File I/O error: {0}")]
    IoError(#[from] std::io::Error),

    /// Invalid input parameters provided
    #[error("Invalid input: {0}")]
    InvalidInput(String),

    /// Requested output format is not supported
    #[error("Output format not supported: {0}")]
    UnsupportedFormat(String),
}

/// Type alias for Results used throughout the rqr library
///
/// This provides a consistent error type across all library operations.
pub type Result<T> = std::result::Result<T, RqrError>;
