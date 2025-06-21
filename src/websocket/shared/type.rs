use crate::*;

pub type WebSocketResult = Result<(), WebSocketError>;
pub type WebSocketMessageResult = Result<WebSocketMessage, WebSocketError>;
