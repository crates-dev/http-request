use crate::*;

pub type BoxWebSocketTrait = Box<dyn WebSocketTrait>;
pub type BoxAsyncWebSocketTrait = Box<dyn AsyncWebSocketTrait>;

pub(crate) type WebSocketConnection = Arc<AsyncMutex<Option<WebSocketConnectionType>>>;
