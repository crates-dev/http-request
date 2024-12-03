use flate2::read::GzDecoder;
use std::io::BufReader;
use std::io::Read;

/// Decodes Gzip compressed data.
///
/// This function takes a Gzip compressed byte vector and decompresses it using the
/// `flate2` crate's `GzDecoder`. It buffers the decompressed data and returns the
/// result as a vector of bytes.
///
/// # Arguments
/// - `data` - A reference to a `Vec<u8>` containing the compressed Gzip data.
/// - `buffer_size` - The buffer size to use for reading the compressed data. A larger
///   buffer size can improve performance for larger datasets.
///
/// # Returns
/// - `Vec<u8>` - The decompressed data as a vector of bytes. If decompression fails,
///   an empty `Vec<u8>` is returned.
pub fn decode(data: &Vec<u8>, buffer_size: usize) -> Vec<u8> {
    let decoder: GzDecoder<&[u8]> = GzDecoder::new(data.as_slice());
    let mut buffered_reader: BufReader<GzDecoder<&[u8]>> =
        BufReader::with_capacity(buffer_size, decoder);
    let mut decompressed_data: Vec<u8> = Vec::new();
    match buffered_reader.read_to_end(&mut decompressed_data) {
        Ok(_) => decompressed_data,
        _ => Vec::new(),
    }
}
