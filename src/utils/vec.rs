use http_constant::{SPACE_U8, TAB_U8};

/// Splits a byte slice into parts based on whitespace characters (space or tab).
///
/// This helper function takes a byte slice and splits it into multiple parts at
/// every whitespace character (either space or tab). It returns a `Vec` containing
/// slices of the original byte slice, excluding the whitespace characters.
///
/// # Parameters
/// - `input`: The byte slice to be split.
///
/// # Returns
/// Returns a `Vec<&[u8]>`, where each element is a slice of the original input,
/// representing a part of the input that was separated by whitespace.
pub fn split_whitespace(input: &[u8]) -> Vec<&[u8]> {
    let mut parts: Vec<&[u8]> = Vec::new();
    let mut start: usize = 0;
    for (i, &byte) in input.iter().enumerate() {
        if byte == SPACE_U8 || byte == TAB_U8 {
            if i > start {
                parts.push(&input[start..i]);
            }
            start = i + 1;
        }
    }
    if start < input.len() {
        parts.push(&input[start..]);
    }
    parts
}

/// Splits a byte slice by a multi-byte delimiter.
///
/// # Parameters
/// - `data` - The byte slice to split.
/// - `delimiter` - The delimiter byte sequence.
///
/// # Returns
/// A vector of byte slices split by the delimiter.
pub fn split_multi_byte<'a>(data: &'a [u8], delimiter: &'a [u8]) -> Vec<&'a [u8]> {
    let mut result: Vec<&[u8]> = Vec::new();
    let mut start: usize = 0;
    for i in 0..=data.len() {
        if data[i..].starts_with(delimiter) {
            result.push(&data[start..i]);
            start = i + delimiter.len();
        }
    }
    if start < data.len() {
        result.push(&data[start..]);
    }
    result
}

/// Compares two byte slices case-insensitively.
///
/// This function checks if two byte slices are equal, ignoring case differences.
///
/// # Parameters
/// - `first`: The first byte slice.
/// - `second`: The second byte slice.
///
/// # Returns
/// Returns `true` if the slices match case-insensitively, otherwise `false`.
pub fn case_insensitive_match(first: &[u8], second: &[u8]) -> bool {
    first.len() == second.len()
        && first
            .iter()
            .zip(second)
            .all(|(byte1, byte2)| byte1.to_ascii_lowercase() == byte2.to_ascii_lowercase())
}
