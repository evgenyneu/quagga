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
}
