use crate::*;

/// JSON body content represented as a hash map with string keys.
pub(crate) type BodyJson = HashMapXxHash3_64<String, JsonValue>;
/// Text body content represented as a string.
pub(crate) type BodyText = String;
/// Binary body content represented as a byte vector.
pub(crate) type BodyBinary = Vec<u8>;
