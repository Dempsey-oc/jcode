//! Thin PDF text-extraction wrapper used by file-attachment paths.

use anyhow::Result;
use std::path::Path;

pub fn extract_text(path: &Path) -> Result<String> {
    Ok(pdf_extract::extract_text(path)?)
}
