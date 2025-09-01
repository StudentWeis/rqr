use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod commands;
mod qr;
mod utils;

use commands::{decode, encode};
use utils::error::Result;


#[derive(Parser)]
#[command(name = "rqr")]
#[command(about = "A CLI tool for encoding/decoding QR Code")]
#[command(version = "0.1.0")]
#[command(disable_help_subcommand = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

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
    /// Decode a QR code from an image file
    Decode {
        /// Path to the image file
        input: PathBuf,
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
            encode::run(content, output, size, error_correction, margin, terminal)?;
        }
        Commands::Decode { input } => {
            decode::run(input)?;
        }
    }

    Ok(())
}
