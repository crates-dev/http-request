use crate::*;

#[derive(Debug, Clone)]
pub(crate) struct SharedWebSocketBuilder;

#[derive(Debug, Clone)]
pub struct WebSocketError {
    pub(crate) kind: WebSocketErrorKind,
    pub(crate) message: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum WebSocketErrorKind {
    Connection,
    Protocol,
    Timeout,
    InvalidUrl,
    Io,
    Tls,
}
