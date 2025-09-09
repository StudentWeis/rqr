//! # rqr Library Usage Example
//!
//! This example demonstrates how to use the rqr library's public API
//! for encoding and decoding QR codes programmatically.
//!
//! The example shows:
//! - Basic QR code encoding and saving to file
//! - Terminal display of QR codes
//! - Decoding QR codes from image files

use rqr::{encode, decode, Result};
use std::path::PathBuf;

fn main() -> Result<()> {
    println!("rqr Library Usage Example");
    println!("=========================");

    // Generate a QR code and save it to a file
    let content = "Library Example - 库使用示例";
    let output_path = PathBuf::from("examples/lib-example.png");
    encode(content.to_string(), output_path.clone(), 200, "M".to_string(), 10, false)?;
    println!("✅ Encoded QR code: {}", output_path.display());

    // Generate a QR code for terminal display
    println!("\n📟 Terminal output:");
    encode("Terminal Example".to_string(), PathBuf::from("dummy.png"), 200, "M".to_string(), 10, true)?;

    // Decode the QR code from the saved file
    println!("\n🔍 Decoding the encoded QR code:");
    decode(output_path)?;

    Ok(())
}
