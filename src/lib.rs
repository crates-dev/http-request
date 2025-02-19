pub(crate) mod body;
#[cfg(test)]
mod cfg;
pub(crate) mod common;
pub(crate) mod constant;
pub(crate) mod global_trait;
pub(crate) mod request;
pub(crate) mod response;
pub(crate) mod utils;

pub use http_compress::*;
pub use request::{
    r#trait::RequestTrait, r#type::BoxRequestTrait, request_builder::r#type::RequestBuilder,
};
pub use response::{
    r#trait::ResponseTrait, r#type::BoxResponseTrait, response_binary::r#type::HttpResponseBinary,
    response_text::r#type::HttpResponseText,
};

pub(crate) use body::r#type::*;
pub(crate) use constant::r#type::*;
pub(crate) use global_trait::r#trait::*;
pub(crate) use http_type::{
    ContentType, HttpBodyString, HttpBodyVec, HttpHeaderMap, HttpHeaderSliceMap, HttpUrlComponents,
    HttpVersion, Methods, Protocol, RequestError, StatusCode, StatusCodeUsize, ACCEPT, ACCEPT_ANY,
    BR_BYTES, COLON_SPACE_BYTES, CONTENT_LENGTH, CONTENT_TYPE, DEFAULT_BUFFER_SIZE,
    DEFAULT_HTTP_PATH, DEFAULT_MAX_REDIRECT_TIMES, DEFAULT_TIMEOUT, EMPTY_STR, HOST, HTTP_BR,
    HTTP_BR_BYTES, HTTP_DOUBLE_BR_BYTES, LOCATION, QUERY_SYMBOL, SPACE_U8, TAB_U8, USER_AGENT,
    ZERO_STR,
};
pub(crate) use request::{config::r#type::*, r#type::*, request::r#type::*, tmp::r#type::*};
pub(crate) use rustls::{
    pki_types::ServerName, ClientConfig, ClientConnection, RootCertStore, StreamOwned,
};

pub(crate) use common::r#type::*;
pub(crate) use std::{
    collections::HashMap,
    fmt::{self, Debug, Display},
    net::TcpStream,
    sync::{Arc, RwLock},
    time::Duration,
    vec::IntoIter,
};
pub(crate) use utils::vec::*;
