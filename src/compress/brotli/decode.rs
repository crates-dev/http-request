use brotli::Decompressor;
use std::io::Read;

/// Decodes compressed data using a specified decompressor
///
/// This function takes a reference to a `Vec<u8>` containing compressed data and a buffer size.
/// It uses the `Decompressor` to decompress the data and returns the decompressed result as a `Vec<u8>`.
/// If decompression fails, it returns an empty `Vec<u8>`.
///
/// # Parameters
/// - `data`: A reference to a `Vec<u8>` containing the compressed data.
/// - `buffer_size`: The buffer size to use for the decompressor.
///
/// # Returns
/// A `Vec<u8>` containing the decompressed data, or an empty `Vec<u8>` if decompression fails.
pub fn decode(data: &Vec<u8>, buffer_size: usize) -> Vec<u8> {
    let mut decompressor: Decompressor<&[u8]> = Decompressor::new(data.as_slice(), buffer_size);
    let mut decompressed_data: Vec<u8> = Vec::new();
    match decompressor.read_to_end(&mut decompressed_data) {
        Ok(_) => decompressed_data,
        _ => Vec::new(),
    }
}
