// TODO:
use flate2::read::DeflateDecoder;
use std::io::{BufReader, Read};

/// Decodes deflate compressed data
///
/// # Parameters
/// - `data`: A reference to a `Vec<u8>` containing the compressed data.
/// - `buffer_size`: The buffer size to use for the buffered reader.
///
/// # Returns
/// A `Vec<u8>` containing the decompressed data, or an empty `Vec<u8>` in case of error.
pub fn decode(data: &Vec<u8>, buffer_size: usize) -> Vec<u8> {
    let decoder: DeflateDecoder<&[u8]> = DeflateDecoder::new(data.as_slice());
    let mut buffered_reader: BufReader<DeflateDecoder<&[u8]>> =
        BufReader::with_capacity(buffer_size, decoder);
    let mut decompressed_data: Vec<u8> = Vec::new();
    match buffered_reader.read_to_end(&mut decompressed_data) {
        Ok(_) => decompressed_data,
        _ => Vec::new(),
    }
}
