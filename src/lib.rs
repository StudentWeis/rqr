//! # rqr - Rust QR Code Library
//!
//! A simple and fast library for encoding and decoding QR codes.
//!
//! This library provides high-level functions for QR code operations
//! while keeping the internal implementation details encapsulated.
//!
//! ## Features
//!
//! - **Encode**: Generate QR codes from text content
//! - **Decode**: Extract text from QR code images
//! - **Flexible Output**: Save to files or display in terminal
//! - **Customizable**: Control size, error correction, and margins
//! - **Error Handling**: Comprehensive error types and handling
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use rqr::{encode, decode};
//!
//! // Encode a QR code and save to file
//! encode(
//!     "Hello, World!".to_string(),
//!     "hello.png".into(),
//!     200,
//!     "M".to_string(),
//!     10,
//!     false
//! )?;
//!
//! // Decode a QR code from file
//! decode("hello.png".into())?;
//! # Ok::<(), rqr::RqrError>(())
//! ```
//!
//! ## Error Handling
//!
//! The library uses `Result<T>` for all operations that can fail.
//! Common error types include encoding failures, file I/O errors,
//! and invalid input parameters.
//!
//! ```rust
//! use rqr::{encode, RqrError};
//!
//! match encode("test".to_string(), "test.png".into(), 200, "M".to_string(), 10, false) {
//!     Ok(()) => println!("Success!"),
//!     Err(RqrError::EncodingError(msg)) => println!("Encoding failed: {}", msg),
//!     Err(e) => println!("Other error: {}", e),
//! }
//! ```

pub mod commands;
pub mod utils;

pub use commands::decode::run as decode;
pub use commands::encode::run as encode;
pub use utils::error::{Result, RqrError};

mod qr;
