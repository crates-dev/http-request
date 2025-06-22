use crate::*;

#[derive(Debug)]
pub enum WebSocketConnectionType {
    Direct(WebSocketStream<MaybeTlsStream<AsyncTcpStream>>),
    Proxy(WebSocketStream<WebSocketProxyTunnelStream>),
}

#[derive(Debug)]
pub struct WebSocket {
    pub(crate) url: Arc<String>,
    pub(crate) header: Arc<RequestHeaders>,
    pub(crate) config: ArcRwLock<WebSocketConfig>,
    pub(crate) connected: Arc<AtomicBool>,
    pub(crate) connection: WebSocketConnection,
}

impl Clone for WebSocket {
    fn clone(&self) -> Self {
        Self {
            url: self.url.clone(),
            header: self.header.clone(),
            config: self.config.clone(),
            connected: Arc::new(AtomicBool::new(false)),
            connection: Arc::new(AsyncMutex::new(None)),
        }
    }
}

impl Default for WebSocket {
    fn default() -> Self {
        Self {
            url: Arc::new(String::new()),
            header: Arc::new(hash_map_xx_hash3_64()),
            config: Arc::new(RwLock::new(WebSocketConfig::default())),
            connected: Arc::new(AtomicBool::new(false)),
            connection: Arc::new(AsyncMutex::new(None)),
        }
    }
}

impl Stream for WebSocketConnectionType {
    type Item =
        Result<tokio_tungstenite::tungstenite::Message, tokio_tungstenite::tungstenite::Error>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match &mut *self {
            WebSocketConnectionType::Direct(stream) => Pin::new(stream).poll_next(cx),
            WebSocketConnectionType::Proxy(stream) => Pin::new(stream).poll_next(cx),
        }
    }
}

impl Sink<tokio_tungstenite::tungstenite::Message> for WebSocketConnectionType {
    type Error = tokio_tungstenite::tungstenite::Error;

    fn poll_ready(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        match &mut *self {
            WebSocketConnectionType::Direct(stream) => Pin::new(stream).poll_ready(cx),
            WebSocketConnectionType::Proxy(stream) => Pin::new(stream).poll_ready(cx),
        }
    }

    fn start_send(
        mut self: Pin<&mut Self>,
        item: tokio_tungstenite::tungstenite::Message,
    ) -> Result<(), Self::Error> {
        match &mut *self {
            WebSocketConnectionType::Direct(stream) => Pin::new(stream).start_send(item),
            WebSocketConnectionType::Proxy(stream) => Pin::new(stream).start_send(item),
        }
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        match &mut *self {
            WebSocketConnectionType::Direct(stream) => Pin::new(stream).poll_flush(cx),
            WebSocketConnectionType::Proxy(stream) => Pin::new(stream).poll_flush(cx),
        }
    }

    fn poll_close(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        match &mut *self {
            WebSocketConnectionType::Direct(stream) => Pin::new(stream).poll_close(cx),
            WebSocketConnectionType::Proxy(stream) => Pin::new(stream).poll_close(cx),
        }
    }
}
