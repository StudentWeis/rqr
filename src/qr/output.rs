use std::path::Path;

use crate::utils::error::{Result, RqrError};

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
    /// use rqr::qr::output::OutputFormat;
    ///
    /// let format = OutputFormat::from_path("image.png")?;
    /// assert!(matches!(format, OutputFormat::Png));
    /// # Ok::<(), rqr::utils::error::RqrError>(())
    /// ```
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();

        match path.extension().and_then(|ext| ext.to_str()) {
            Some(ext) => {
                let ext_lower = ext.to_lowercase();
                match ext_lower.as_str() {
                    "png" => Ok(OutputFormat::Png),
                    _ => Err(RqrError::UnsupportedFormat(format!(
                        "Unsupported format: {}",
                        ext
                    ))),
                }
            }
            None => Err(RqrError::UnsupportedFormat(
                "Cannot determine format from file path".to_string(),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn test_output_format_from_path_png() {
        let format = OutputFormat::from_path("test.png").unwrap();
        assert!(matches!(format, OutputFormat::Png));
    }

    #[test]
    fn test_output_format_from_path_png_uppercase() {
        let format = OutputFormat::from_path("test.PNG").unwrap();
        assert!(matches!(format, OutputFormat::Png));
    }

    #[test]
    fn test_output_format_from_path_png_mixed_case() {
        let format = OutputFormat::from_path("test.Png").unwrap();
        assert!(matches!(format, OutputFormat::Png));
    }

    #[test]
    fn test_output_format_unsupported_jpg() {
        let result = OutputFormat::from_path("test.jpg");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("Unsupported format: jpg"));
    }

    #[test]
    fn test_output_format_unsupported_jpeg() {
        let result = OutputFormat::from_path("image.jpeg");
        assert!(result.is_err());
    }

    #[test]
    fn test_output_format_unsupported_gif() {
        let result = OutputFormat::from_path("animation.gif");
        assert!(result.is_err());
    }

    #[test]
    fn test_output_format_no_extension() {
        let result = OutputFormat::from_path("filename");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("Cannot determine format"));
    }

    #[test]
    fn test_output_format_empty_extension() {
        let result = OutputFormat::from_path("filename.");
        assert!(result.is_err());
    }

    #[test]
    fn test_output_format_from_pathbuf() {
        let path = PathBuf::from("output.png");
        let format = OutputFormat::from_path(&path).unwrap();
        assert!(matches!(format, OutputFormat::Png));
    }

    #[test]
    fn test_output_format_clone() {
        let format = OutputFormat::Png;
        let cloned = format.clone();
        assert!(matches!(cloned, OutputFormat::Png));
    }

    #[test]
    fn test_output_format_debug() {
        let format = OutputFormat::Terminal;
        let debug_str = format!("{:?}", format);
        assert!(debug_str.contains("Terminal"));
    }

    #[test]
    fn test_output_format_from_nested_path() {
        let format = OutputFormat::from_path("path/to/file.png").unwrap();
        assert!(matches!(format, OutputFormat::Png));
    }

    #[test]
    fn test_output_format_from_absolute_path() {
        let format = OutputFormat::from_path("/absolute/path/to/file.png").unwrap();
        assert!(matches!(format, OutputFormat::Png));
    }
}
