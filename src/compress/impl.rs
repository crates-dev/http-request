use std::str::FromStr;

use super::{
    brotli,
    constant::{BR, DEFLATE, GZIP},
    deflate, gzip,
    r#type::Compress,
};
use crate::{request::constant::CONTENT_ENCODING, response::response_header::r#type::Header};

impl Default for Compress {
    fn default() -> Self {
        Self::Unknown
    }
}

impl FromStr for Compress {
    type Err = ();

    fn from_str(data: &str) -> Result<Self, Self::Err> {
        match data.to_lowercase().as_str() {
            _data if _data == GZIP => Ok(Self::Gzip),
            _data if _data == DEFLATE => Ok(Self::Deflate),
            _data if _data == BR => Ok(Self::Br),
            _ => Ok(Self::Unknown),
        }
    }
}

impl Compress {
    /// Extracts the compression type from an HTTP header.
    ///
    /// This function looks for the `Content-Encoding` header in the provided `Header` and attempts
    /// to parse it into a `Compress` enum value.
    ///
    /// # Arguments
    /// - `header` - The HTTP header from which the compression type is to be extracted.
    ///
    /// # Returns
    /// - The `Compress` value corresponding to the `Content-Encoding` header, or `Compress::Unknown`
    ///   if the header does not match any known compression types.
    pub fn from(header: &Header) -> Self {
        let content_encoding_key: String = CONTENT_ENCODING.to_lowercase();
        let mut compress: Compress = Self::default();
        for (key, value) in header {
            if key.to_lowercase() == content_encoding_key {
                compress = value.parse::<Compress>().unwrap_or_default();
                break;
            }
        }
        compress
    }

    /// Decodes data based on the compression type.
    ///
    /// This function decodes the compressed data using the corresponding compression algorithm
    /// (Gzip, Deflate, or Brotli) depending on the `Compress` enum value.
    ///
    /// # Arguments
    /// - `data` - A vector of bytes containing the compressed data.
    /// - `buffer_size` - The buffer size to use during decompression.
    ///
    /// # Returns
    /// - A `Vec<u8>` containing the decompressed data.
    pub fn decode(&self, data: &Vec<u8>, buffer_size: usize) -> Vec<u8> {
        match self {
            Self::Gzip => gzip::decode::decode(data, buffer_size),
            Self::Deflate => deflate::decode::decode(data, buffer_size),
            Self::Br => brotli::decode::decode(data, buffer_size),
            Self::Unknown => data.clone(),
        }
    }
}
