//! # rqr - Rust QR Code CLI Tool
//!
//! A simple and fast command-line tool for encoding and decoding QR codes.
//!
//! ## Features
//!
//! - Encode QR codes from text
//! - Save QR codes as PNG images
//! - Display QR codes in terminal
//! - Decode QR codes from image files
//! - Customizable size, error correction, and margin
//!
//! ## Usage
//!
//! ```bash
//! # Encode a QR code
//! rqr encode "Hello, World!" --output hello.png --size 300
//!
//! # Display QR code in terminal
//! rqr encode "Terminal QR" --terminal
//!
//! # Decode a QR code from image
//! rqr decode qr-image.png
//! ```

pub mod commands;
pub mod qr;
pub mod utils;

pub use utils::error::{Result, RqrError};

// Re-export the main functionality
pub use qr::decoder::QrDecoder;
pub use qr::encoder::QrEncoder;
pub use qr::output::OutputFormat;
