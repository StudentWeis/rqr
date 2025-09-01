use thiserror::Error;

#[derive(Error, Debug)]
pub enum RqrError {
    #[error("QR code encoding failed: {0}")]
    EncodingError(String),

    #[error("QR code decoding failed: {0}")]
    DecodingError(String),

    #[error("Image processing error: {0}")]
    ImageError(#[from] image::ImageError),

    #[error("File I/O error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Output format not supported: {0}")]
    UnsupportedFormat(String),
}

pub type Result<T> = std::result::Result<T, RqrError>;
