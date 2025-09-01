use crate::utils::error::{Result, RqrError};
use std::path::Path;

#[derive(Debug, Clone)]
pub enum OutputFormat {
    Png,
    Terminal,
    // TODO: Future formats: SVG, JPEG, etc.
}

impl OutputFormat {
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
