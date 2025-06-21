use crate::*;
use std::sync::atomic::AtomicBool;
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};

type WebSocketConnection = Arc<Mutex<Option<WebSocketStream<MaybeTlsStream<TcpStream>>>>>;

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
            connection: Arc::new(Mutex::new(None)),
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
            connection: Arc::new(Mutex::new(None)),
        }
    }
}
