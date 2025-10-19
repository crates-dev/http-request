//! http-request
//!
//! http-request is a lightweight, efficient library for building, sending,
//! and handling HTTP/HTTPS requests in Rust applications.
//! It provides a simple and intuitive API, allowing developers to easily
//! interact with web services, whether they use the "HTTP" or "HTTPS" protocol.
//! The library supports various HTTP methods, custom headers, request bodies,
//! timeout, automatic handling of redirects (including detecting redirect loops),
//! and enhanced response body decoding (both automatic and manual), enabling fast
//! and secure communication. Whether working with secure "HTTPS" connections
//! or standard "HTTP" requests, the library is optimized for performance,
//! minimal resource usage, and easy integration into Rust projects.

pub(crate) mod body;
pub(crate) mod cfg;
pub(crate) mod common;
pub(crate) mod r#const;
pub(crate) mod request;
pub(crate) mod response;
pub(crate) mod utils;

pub use request::*;
pub use response::*;

pub use http_type::{
    HashMapXxHash3_64, JsonDeserializer, JsonError, JsonMap, JsonNumber, JsonResult,
    JsonSerializer, JsonStreamDeserializer, JsonValue, hash_map_xx_hash3_64, json_from_reader,
    json_from_slice, json_from_str, json_from_value, json_to_string, json_to_string_pretty,
    json_to_value, json_to_vec, json_to_vec_pretty, json_to_writer, json_to_writer_pretty,
    json_value,
};

pub(crate) use body::*;
pub(crate) use common::*;
pub(crate) use r#const::*;
pub(crate) use utils::*;

pub(crate) use futures::{Future, Sink, SinkExt, Stream, StreamExt};
pub(crate) use http_type::{
    ACCEPT, ACCEPT_ANY, BR_BYTES, CONNECTION, CONTENT_LENGTH, CONTENT_TYPE, Compress, ContentType,
    DEFAULT_BUFFER_SIZE, DEFAULT_HTTP_PATH, DEFAULT_MAX_REDIRECT_TIMES, DEFAULT_TIMEOUT, EMPTY_STR,
    HOST, HTTP_BR_BYTES, HttpStatus, HttpUrlComponents, HttpVersion, LOCATION, Method, Protocol,
    QUERY_SYMBOL, RequestBody, RequestBodyString, RequestError, RequestHeaders, ResponseHeaders,
    ResponseStatusCode, SEC_WEBSOCKET_KEY, SEC_WEBSOCKET_VERSION, SPACE_U8, TAB_U8, UPGRADE,
    USER_AGENT,
    tokio::{
        io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt, ReadBuf},
        net::TcpStream as AsyncTcpStream,
        runtime::Runtime,
        sync::{Mutex as AsyncMutex, MutexGuard as AsyncMutexGuard},
        time::timeout,
    },
};
pub(crate) use rustls::{
    ClientConfig, ClientConnection, RootCertStore, StreamOwned, pki_types::ServerName,
};
pub(crate) use serde::{Serialize, Serializer};
pub(crate) use std::{
    borrow::Cow,
    collections::{HashSet, VecDeque},
    fmt::{self, Debug, Display, Formatter},
    io::{Read, Write},
    net::{Ipv4Addr, Ipv6Addr, TcpStream},
    pin::Pin,
    str::from_utf8,
    sync::{
        Arc, RwLock,
        atomic::{AtomicBool, Ordering},
    },
    task::{Context, Poll},
    time::{Duration, SystemTime, UNIX_EPOCH},
    vec::IntoIter,
};
pub(crate) use tokio_rustls::{TlsConnector, client::TlsStream};
pub(crate) use tokio_tungstenite::{
    MaybeTlsStream, WebSocketStream, client_async_with_config, connect_async_with_config,
    tungstenite::Message, tungstenite::handshake::client::Request,
};
pub(crate) use webpki_roots::TLS_SERVER_ROOTS;

#[cfg(test)]
use std::{
    sync::{Mutex, MutexGuard},
    thread::{JoinHandle, spawn},
    time::Instant,
};

#[cfg(test)]
use http_type::tokio;
