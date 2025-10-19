use crate::*;

#[derive(Debug, Clone, Default)]
pub struct WebSocketBuilder {
    pub(crate) websocket: WebSocket,
    pub(crate) builder: WebSocket,
}
