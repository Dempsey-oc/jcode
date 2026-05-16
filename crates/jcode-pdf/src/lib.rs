//! Thin PDF text-extraction wrapper used by file-attachment paths.

use std::path::Path;
use thiserror::Error;

/// Errors returned by [`extract_text`].
#[derive(Debug, Error)]
pub enum PdfError {
    #[error("failed to extract PDF text: {0}")]
    Extract(#[from] pdf_extract::OutputError),
}

pub type Result<T> = std::result::Result<T, PdfError>;

pub fn extract_text(path: &Path) -> Result<String> {
    Ok(pdf_extract::extract_text(path)?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn extract_text_on_missing_file_returns_pdf_error() {
        let missing = PathBuf::from("/tmp/jcode-pdf-does-not-exist-zzz.pdf");
        let err = extract_text(&missing).expect_err("should error");
        // Display impl exists via thiserror derive.
        let msg = err.to_string();
        assert!(msg.starts_with("failed to extract PDF text:"), "got: {msg}");
        // Specific variant is reachable for downcasting consumers.
        let PdfError::Extract(_) = err;
    }
}
