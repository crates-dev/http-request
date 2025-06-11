#[cfg(test)]
mod cfg;

pub(crate) mod body;
pub(crate) mod common;
pub(crate) mod r#const;
pub(crate) mod global_trait;
pub(crate) mod header;
pub(crate) mod request;
pub(crate) mod response;
pub(crate) mod utils;

pub use request::*;
pub use response::*;

pub use serde_json::{
    Deserializer as JsonDeserializer, Error as JsonError, Map as JsonMap, Number as JsonNumber,
    Result as JsonResult, Serializer as JsonSerializer,
    StreamDeserializer as JsonStreamDeserializer, Value as JsonValue,
    from_reader as json_from_reader, from_slice as json_from_slice, from_str as json_from_str,
    from_value as json_from_value, json as json_value, to_string as json_to_string,
    to_string_pretty as json_to_string_pretty, to_value as json_to_value, to_vec as json_to_vec,
    to_vec_pretty as json_to_vec_pretty, to_writer as json_to_writer,
    to_writer_pretty as json_to_writer_pretty,
};

pub(crate) use body::*;
pub(crate) use common::*;
pub(crate) use r#const::*;
pub(crate) use global_trait::*;
pub(crate) use header::*;
pub(crate) use http_type::{HashMapXxHash3_64, hash_map_xx_hash3_64};
pub(crate) use utils::vec::*;

pub(crate) use http_type::{
    ACCEPT, ACCEPT_ANY, BR_BYTES, COLON_SPACE_BYTES, CONTENT_LENGTH, CONTENT_TYPE, Compress,
    ContentType, DEFAULT_BUFFER_SIZE, DEFAULT_HTTP_PATH, DEFAULT_MAX_REDIRECT_TIMES,
    DEFAULT_TIMEOUT, EMPTY_STR, HOST, HTTP_BR, HTTP_BR_BYTES, HTTP_DOUBLE_BR_BYTES, HttpStatus,
    HttpUrlComponents, HttpVersion, LOCATION, Method, Protocol, QUERY_SYMBOL, RequestBody,
    RequestBodyString, RequestError, ResponseHeaders, ResponseStatusCode, SPACE_U8, TAB_U8,
    USER_AGENT, ZERO_STR,
};
pub(crate) use rustls::{
    ClientConfig, ClientConnection, RootCertStore, StreamOwned, pki_types::ServerName,
};
pub(crate) use serde::{Serialize, Serializer};
pub(crate) use std::{
    collections::HashSet,
    fmt::{self, Debug, Display},
    net::TcpStream,
    sync::{Arc, RwLock},
    time::Duration,
    vec::IntoIter,
};
