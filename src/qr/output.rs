use crate::utils::error::{Result, RqrError};
use std::path::Path;

/// Output format options for QR code rendering
///
/// Specifies how the generated QR code should be presented
/// to the user - either as an image file or terminal display.
#[derive(Debug, Clone)]
pub enum OutputFormat {
    /// Save QR code as PNG image file
    Png,
    /// Display QR code in terminal using ASCII art
    Terminal,
    // TODO: Future formats: SVG, JPEG, etc.
}

impl OutputFormat {
    /// Determine output format from file extension
    ///
    /// # Arguments
    /// * `path` - The file path to analyze
    ///
    /// # Returns
    /// Returns the appropriate `OutputFormat` or an error for unsupported formats
    ///
    /// # Examples
    /// ```rust
    /// use std::path::Path;
    ///
    /// let format = OutputFormat::from_path("image.png")?;
    /// assert!(matches!(format, OutputFormat::Png));
    /// # Ok::<(), rqr::RqrError>(())
    /// ```
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();

        match path.extension().and_then(|ext| ext.to_str()) {
            Some("png") => Ok(OutputFormat::Png),
            Some(ext) => Err(RqrError::UnsupportedFormat(format!(
                "Unsupported format: {}",
                ext
            ))),
            None => Err(RqrError::UnsupportedFormat(
                "Cannot determine format from file path".to_string(),
            )),
        }
    }
}
