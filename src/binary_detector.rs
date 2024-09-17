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
}
