//! # Utils Module
//!
//! Utility modules including error handling and common types.

pub mod error;

/// Test utilities module
///
/// This module provides helper functions for tests.
/// Only available when running tests.
#[cfg(test)]
pub mod test_utils {
    use tempfile::TempDir;

    /// Creates a temporary directory for test files
    ///
    /// # Returns
    /// A `TempDir` that will be automatically cleaned up when dropped
    ///
    /// # Panics
    /// Panics if the temporary directory cannot be created
    pub fn temp_dir() -> TempDir {
        TempDir::new().expect("Failed to create temp directory")
    }
}
