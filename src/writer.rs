//! Render it's files
use std::fs::File;
use std::io::Write;

use anyhow::Result;

use crate::errors::ChabloError;

/// Write down content into file
pub fn write(template: &str, path: &str) -> Result<(), ChabloError> {
    let mut file = File::create(path)?;
    file.write_all(template.as_bytes())?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use std::fs;

    #[test]
    fn test_write_content_to_file_ok() {
        let content = "test test test".to_string();
        let path = "tests/fixtures/test.html";

        write(&content, path).unwrap();

        let result = fs::read_to_string(path).unwrap();
        let expected_result = "test test test";

        assert_eq!(result, expected_result);

        fs::remove_file(&path).unwrap();
    }

    #[test]
    fn test_write_content_to_file_metadata_ok() {
        let content = "testing file type".to_string();
        let path = "tests/fixtures/test_metadata.html";

        write(&content, path).unwrap();

        let metadata = fs::metadata(path).unwrap();

        assert!(metadata.is_file());

        fs::remove_file(&path).unwrap();
    }
}
