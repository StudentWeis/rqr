//! # rqr - QR Code CLI Tool
//!
//! Command-line interface for the rqr QR code library.
//! Provides easy-to-use commands for encoding and decoding QR codes.

use clap::{Parser, Subcommand};
use std::path::PathBuf;

use rqr::{Result, decode, encode};

/// Main CLI structure for the rqr tool
#[derive(Parser)]
#[command(name = "rqr", version, about, long_about = None)]
#[command(disable_help_subcommand = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

/// Available subcommands
#[derive(Subcommand)]
enum Commands {
    /// Encode a QR code from text
    Encode {
        /// Text content to encode
        content: String,
        /// Output file path
        #[arg(short, long, default_value = "rqr.png")]
        output: PathBuf,
        /// QR code size in pixels
        #[arg(short, long, default_value = "200")]
        size: u32,
        /// Error correction level (L, M, Q, H)
        #[arg(short, long, default_value = "M")]
        error_correction: String,
        /// Margin size
        #[arg(short, long, default_value = "10")]
        margin: u32,
        /// Output to terminal instead of file
        #[arg(short, long)]
        terminal: bool,
    },
    /// Decode a QR code from an image file or URL
    Decode {
        /// Path to the image file or URL
        input: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Encode {
            content,
            output,
            size,
            error_correction,
            margin,
            terminal,
        } => {
            encode(content, output, size, error_correction, margin, terminal)?;
        }
        Commands::Decode { input } => {
            decode(input)?;
        }
    }

    Ok(())
}
