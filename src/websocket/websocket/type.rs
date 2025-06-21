use crate::*;

pub(crate) type BoxWebSocketTrait = Box<dyn WebSocketTrait>;
pub(crate) type BoxAsyncWebSocketTrait = Box<dyn AsyncWebSocketTrait>;
