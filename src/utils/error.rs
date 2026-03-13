use thiserror::Error;

/// Comprehensive error type for the rqr tool
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

/// Type alias for Results used throughout the rqr tool
///
/// This provides a consistent error type across all operations.
pub type Result<T> = std::result::Result<T, RqrError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encoding_error_display() {
        let error = RqrError::EncodingError("test error".to_string());
        assert_eq!(error.to_string(), "QR code encoding failed: test error");
    }

    #[test]
    fn test_decoding_error_display() {
        let error = RqrError::DecodingError("decode failed".to_string());
        assert_eq!(error.to_string(), "QR code decoding failed: decode failed");
    }

    #[test]
    fn test_invalid_input_error_display() {
        let error = RqrError::InvalidInput("bad input".to_string());
        assert_eq!(error.to_string(), "Invalid input: bad input");
    }

    #[test]
    fn test_unsupported_format_error_display() {
        let error = RqrError::UnsupportedFormat("gif".to_string());
        assert_eq!(error.to_string(), "Output format not supported: gif");
    }

    #[test]
    fn test_io_error_conversion() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let rqr_err: RqrError = io_err.into();
        assert!(matches!(rqr_err, RqrError::IoError(_)));
        assert!(rqr_err.to_string().contains("File I/O error"));
    }

    #[test]
    fn test_result_type_alias() {
        fn returns_ok() -> Result<i32> {
            Ok(42)
        }

        fn returns_err() -> Result<i32> {
            Err(RqrError::InvalidInput("test".to_string()))
        }

        assert_eq!(returns_ok().unwrap(), 42);
        assert!(returns_err().is_err());
    }

    #[test]
    fn test_error_debug_format() {
        let error = RqrError::EncodingError("debug test".to_string());
        let debug_str = format!("{:?}", error);
        assert!(debug_str.contains("EncodingError"));
        assert!(debug_str.contains("debug test"));
    }

    #[test]
    fn test_error_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<RqrError>();
    }
}
