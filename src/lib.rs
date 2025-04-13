#[cfg(test)]
mod cfg;

pub(crate) mod body;
pub(crate) mod common;
pub(crate) mod r#const;
pub(crate) mod global_trait;
pub(crate) mod request;
pub(crate) mod response;
pub(crate) mod utils;

pub use http_type::{
    HashMapXxHash3_64, OptionStr, OptionUsize, OptionVecU8, hash_map_xxhash3_64, serde, serde_json,
    serde_urlencoded, serde_xml_rs,
};
pub use request::{
    request_builder::r#type::RequestBuilder, r#trait::RequestTrait, r#type::BoxRequestTrait,
};
pub use response::{
    response_binary::r#type::HttpResponseBinary, response_text::r#type::HttpResponseText,
    r#trait::ResponseTrait, r#type::BoxResponseTrait,
};

pub(crate) use body::r#type::*;
pub(crate) use r#const::r#const::*;
pub(crate) use global_trait::r#trait::*;
pub(crate) use http_type::{
    ACCEPT, ACCEPT_ANY, BR_BYTES, COLON_SPACE_BYTES, CONTENT_LENGTH, CONTENT_TYPE, Compress,
    ContentType, DEFAULT_BUFFER_SIZE, DEFAULT_HTTP_PATH, DEFAULT_MAX_REDIRECT_TIMES,
    DEFAULT_TIMEOUT, EMPTY_STR, HOST, HTTP_BR, HTTP_BR_BYTES, HTTP_DOUBLE_BR_BYTES, HttpStatus,
    HttpUrlComponents, HttpVersion, LOCATION, Methods, Protocol, QUERY_SYMBOL, RequestBody,
    RequestBodyString, RequestError, RequestHeaders, ResponseHeaders, ResponseStatusCode, SPACE_U8,
    TAB_U8, USER_AGENT, ZERO_STR,
};
pub(crate) use request::{config::r#type::*, request::r#type::*, tmp::r#type::*, r#type::*};
pub(crate) use rustls::{
    ClientConfig, ClientConnection, RootCertStore, StreamOwned, pki_types::ServerName,
};

pub(crate) use common::r#type::*;
pub(crate) use std::{
    fmt::{self, Debug, Display},
    net::TcpStream,
    sync::{Arc, RwLock},
    time::Duration,
    vec::IntoIter,
};
pub(crate) use utils::vec::*;
