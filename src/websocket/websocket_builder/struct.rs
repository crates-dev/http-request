use crate::*;

#[derive(Debug, Clone)]
pub struct WebSocketBuilder {
    pub(crate) websocket: WebSocket,
    pub(crate) builder: WebSocket,
}

impl Default for WebSocketBuilder {
    fn default() -> Self {
        Self {
            websocket: WebSocket::default(),
            builder: WebSocket::default(),
        }
    }
}
