use crate::*;

pub(crate) type BodyJson = HashMapXxHash3_64<String, JsonValue>;
pub(crate) type BodyText = String;
pub(crate) type BodyBinary = Vec<u8>;
