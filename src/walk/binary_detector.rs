use std::fs::File;
use std::io::{self, Read};
use std::path::PathBuf;

/// Determines if a file at the given path is likely a valid UTF-8 text code file.
///
/// This function reads the first 1024 bytes of the file and uses `is_valid_text`
/// to check if the content is likely text.
///
/// # Arguments
///
/// * `file_path` - A `PathBuf` representing the path to the file.
///
/// # Returns
///
/// * `Ok(true)` if the file is likely a valid text file.
/// * `Ok(false)` if the file is likely binary.
/// * `Err` if an error occurs while opening or reading the file.
pub fn is_valid_text_file(file_path: PathBuf) -> io::Result<bool> {
    const SAMPLE_SIZE: usize = 1024; // Number of bytes to read for sampling

    // Open the file in read-only mode
    let mut file = File::open(file_path)?;

    // Read up to SAMPLE_SIZE bytes from the file
    let mut buffer = Vec::with_capacity(SAMPLE_SIZE);
    let _ = file
        .by_ref()
        .take(SAMPLE_SIZE as u64)
        .read_to_end(&mut buffer)?;

    // Use is_valid_text to determine if the buffer is likely text
    Ok(is_valid_text(&buffer))
}

/// Counts the number of null bytes (`0x00`) in a buffer.
///
/// # Arguments
///
/// * `buffer` - A slice of bytes to inspect.
///
/// # Returns
///
/// The number of null bytes found in the buffer.
pub fn number_of_null_bytes(buffer: &[u8]) -> usize {
    buffer.iter().filter(|&&byte| byte == 0).count()
}

use std::str;

/// Checks if a buffer contains valid UTF-8 data, handling incomplete multibyte characters at the end.
///
/// This function trims incomplete multibyte sequences from the end of the buffer before validation.
///
/// # Arguments
///
/// * `buffer` - A slice of bytes to check.
///
/// # Returns
///
/// `true` if the buffer contains valid UTF-8 data, `false` otherwise.
pub fn is_valid_utf8(buffer: &[u8]) -> bool {
    let len = buffer.len();

    // Maximum length of a UTF-8 character is 4 bytes
    let max_char_len = 4;

    // We only need to check the last few bytes
    let start = if len > max_char_len {
        len - max_char_len
    } else {
        0
    };

    // Trim off incomplete multibyte characters at the end
    for i in (start..len).rev() {
        let byte = buffer[i];
        if (byte & 0b1100_0000) != 0b1000_0000 {
            // this is not a continuation byte (10xx_xxxx)
            let mut trim_to = i + 1;

            if (byte & 0b1110_0000) == 0b1100_0000 || // first byte of a 2-byte character
               (byte & 0b1111_0000) == 0b1110_0000 || // first byte of a 3-byte character
               (byte & 0b1111_1000) == 0b1111_0000
            // first byte of a 4-byte character
            {
                trim_to -= 1; // trim the byte off
            }

            let trimmed = &buffer[..trim_to];
            return str::from_utf8(trimmed).is_ok();
        }
    }

    str::from_utf8(buffer).is_ok()
}

/// Determines if a buffer is likely a text file in UTF-8 encoding (e.g., source code).
///
/// # Arguments
///
/// * `buffer` - A slice of bytes representing the content to check.
///
/// # Returns
///
/// `true` if the buffer is likely a text file in UTF-8 encoding, `false` otherwise.
pub fn is_valid_text(buffer: &[u8]) -> bool {
    if number_of_null_bytes(buffer) > 0 {
        false // Contains null bytes; likely binary
    } else {
        is_valid_utf8(buffer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::temp_dir::TempDir;

    #[test]
    fn test_number_of_null_bytes_with_no_nulls() {
        let buffer = vec![1, 2, 3, 4, 5];
        let count = number_of_null_bytes(&buffer);
        assert_eq!(count, 0);
    }

    #[test]
    fn test_number_of_null_bytes_with_nulls() {
        let buffer = vec![0, 1, 0, 2, 0];
        let count = number_of_null_bytes(&buffer);
        assert_eq!(count, 3);
    }

    #[test]
    fn test_number_of_null_bytes_with_empty_buffer() {
        let buffer: Vec<u8> = Vec::new();
        let count = number_of_null_bytes(&buffer);
        assert_eq!(count, 0);
    }

    #[test]
    fn test_number_of_null_bytes_with_all_nulls() {
        let buffer = vec![0; 10];
        let count = number_of_null_bytes(&buffer);
        assert_eq!(count, 10);
    }

    #[test]
    fn test_is_valid_utf8_with_valid_utf8() {
        let buffer = "Hello, world!".as_bytes();
        assert!(is_valid_utf8(&buffer));
    }

    #[test]
    fn test_is_valid_utf8_with_valid_utf8_multibyte() {
        let buffer = "こんにちは".as_bytes(); // "Hello" in Japanese
        assert!(is_valid_utf8(&buffer));
    }

    #[test]
    fn test_is_valid_utf8_with_valid_single_multibyte_character() {
        let buffer = "こ".as_bytes();
        assert!(is_valid_utf8(&buffer));
    }

    #[test]
    fn test_is_valid_utf8_with_incomplete_multibyte_at_end() {
        let mut buffer = "こんにちは".as_bytes().to_vec();
        buffer.pop(); // Remove last byte to simulate incomplete character
        assert!(is_valid_utf8(&buffer));
    }

    #[test]
    fn test_is_valid_utf8_with_invalid_utf8() {
        let buffer = vec![0xFF, 0xFE, 0xFD];
        assert!(!is_valid_utf8(&buffer));
    }

    #[test]
    fn test_is_valid_utf8_with_continuation_bytes_only() {
        let buffer = vec![0x80, 0x80, 0x80]; // Continuation bytes without a start byte
        assert!(!is_valid_utf8(&buffer));
    }

    #[test]
    fn test_is_valid_utf8_with_empty_buffer() {
        let buffer: Vec<u8> = Vec::new();
        assert!(is_valid_utf8(&buffer));
    }

    #[test]
    fn test_is_valid_text_with_utf8_text() {
        let buffer = "fn main() {}".as_bytes();
        assert!(is_valid_text(buffer));
    }

    #[test]
    fn test_is_valid_text_with_binary_data() {
        let buffer = [0x00, 0xFF, 0x00, 0xFF]; // Contains null bytes
        assert!(!is_valid_text(&buffer));
    }

    #[test]
    fn test_is_valid_text_with_non_utf8_text() {
        let buffer = vec![0xC0, 0xC1]; // Invalid UTF-8 sequences
        assert!(!is_valid_text(&buffer));
    }

    #[test]
    fn test_is_valid_text_with_empty_buffer() {
        let buffer: &[u8] = &[];
        assert!(is_valid_text(buffer));
    }

    #[test]
    fn test_is_valid_text_file_with_text_file() {
        let td = TempDir::new().unwrap();
        let file_path = td.mkfile_with_contents("test.txt", "fn main() {}");

        let result = is_valid_text_file(file_path).unwrap();

        assert!(result);
    }

    #[test]
    fn test_is_valid_text_file_with_binary_file() {
        let td = TempDir::new().unwrap();
        let file_path = td.mkfile_with_bytes("test.bin", &[0x00, 0xFF, 0x00, 0xFF]); // Contains null bytes

        // Check if the file is valid text
        let result = is_valid_text_file(file_path).unwrap();

        assert!(!result);
    }

    #[test]
    fn test_is_valid_text_file_with_empty_file() {
        let td = TempDir::new().unwrap();
        let file_path = td.path().join("empty.txt");
        File::create(&file_path).unwrap();

        let result = is_valid_text_file(file_path).unwrap();

        assert!(result, "Empty file detected as binary");
    }

    #[test]
    fn test_is_valid_text_file_with_nonexistent_file() {
        let td = TempDir::new().unwrap();
        let file_path = td.path().join("nonexistent.txt");

        let result = is_valid_text_file(file_path);

        assert!(result.is_err());
    }

    #[test]
    fn test_is_valid_text_file_with_directory_path() {
        let td = TempDir::new().unwrap();
        let dir_path = td.path().to_path_buf();

        let result = is_valid_text_file(dir_path);

        assert!(result.is_err());
    }
}
