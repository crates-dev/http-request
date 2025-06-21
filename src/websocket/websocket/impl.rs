use crate::*;
use futures::{SinkExt, StreamExt};
use std::sync::atomic::Ordering;
use std::time::Duration;
use tokio::time::timeout;
use tokio_tungstenite::{
    connect_async_with_config, tungstenite::Message, tungstenite::handshake::client::Request,
};

impl WebSocket {
    fn get_url(&self) -> String {
        self.url.as_ref().clone()
    }

    fn get_headers(&self) -> Vec<(String, String)> {
        let mut headers: Vec<(String, String)> = Vec::new();

        for (key, value) in self.header.iter() {
            headers.push((key.clone(), value.clone()));
        }

        headers
    }

    async fn connect_async_internal(&self) -> Result<(), WebSocketError> {
        if self.connected.load(Ordering::Relaxed) {
            return Ok(());
        }

        let url: String = self.get_url();
        if url.is_empty() {
            return Err(WebSocketError::invalid_url("URL is empty"));
        }

        let url_obj: HttpUrlComponents = SharedWebSocketBuilder::parse_url(&url)?;

        if let Ok(mut config) = self.config.write() {
            config.url_obj = url_obj;
        }

        let timeout_duration: Duration = Duration::from_millis(
            self.config
                .read()
                .map(|c| c.timeout)
                .unwrap_or(DEFAULT_TIMEOUT),
        );

        let headers: Vec<(String, String)> = self.get_headers();
        let mut request_builder = Request::builder().uri(&url);

        for (key, value) in headers {
            request_builder = request_builder.header(&key, &value);
        }

        let request: Request = request_builder
            .body(())
            .map_err(|e| WebSocketError::invalid_url(format!("Failed to build request: {}", e)))?;

        let connect_future = connect_async_with_config(request, None, false);
        let (ws_stream, _) = timeout(timeout_duration, connect_future)
            .await
            .map_err(|_| WebSocketError::timeout("Connection timeout"))?
            .map_err(|e| {
                let error_msg: String = e.to_string();
                if error_msg.contains("tls")
                    || error_msg.contains("TLS")
                    || error_msg.contains("ssl")
                    || error_msg.contains("SSL")
                    || error_msg.contains("certificate")
                    || error_msg.contains("handshake")
                {
                    WebSocketError::tls(error_msg)
                } else {
                    WebSocketError::connection(error_msg)
                }
            })?;

        let mut connection = self.connection.lock().await;
        *connection = Some(ws_stream);

        self.connected.store(true, Ordering::Relaxed);
        Ok(())
    }

    async fn send_message_async(&self, message: Message) -> Result<(), WebSocketError> {
        if !self.connected.load(Ordering::Relaxed) {
            self.connect_async_internal().await?;
        }

        let mut connection = self.connection.lock().await;
        if let Some(ref mut ws_stream) = *connection {
            ws_stream
                .send(message)
                .await
                .map_err(|e| WebSocketError::protocol(e.to_string()))?;
        } else {
            return Err(WebSocketError::connection("Not connected"));
        }
        Ok(())
    }

    fn send_message_sync(&self, message: Message) -> Result<(), WebSocketError> {
        let rt: tokio::runtime::Runtime =
            tokio::runtime::Runtime::new().map_err(|e| WebSocketError::io(e.to_string()))?;
        rt.block_on(self.send_message_async(message))
    }

    async fn receive_message_async(&self) -> Result<WebSocketMessage, WebSocketError> {
        if !self.connected.load(Ordering::Relaxed) {
            return Err(WebSocketError::connection("Not connected"));
        }

        let timeout_duration: Duration = Duration::from_millis(
            self.config
                .read()
                .map(|c| c.timeout)
                .unwrap_or(DEFAULT_TIMEOUT),
        );

        let mut connection = self.connection.lock().await;
        if let Some(ref mut ws_stream) = *connection {
            let receive_future = ws_stream.next();
            if let Some(msg_result) = timeout(timeout_duration, receive_future)
                .await
                .map_err(|_| WebSocketError::timeout("Receive timeout"))?
            {
                let message: Message =
                    msg_result.map_err(|e| WebSocketError::protocol(e.to_string()))?;
                return Ok(self.convert_message(message));
            }
        }
        Err(WebSocketError::connection("Connection closed"))
    }

    fn receive_message_sync(&self) -> Result<WebSocketMessage, WebSocketError> {
        let rt: tokio::runtime::Runtime =
            tokio::runtime::Runtime::new().map_err(|e| WebSocketError::io(e.to_string()))?;
        rt.block_on(self.receive_message_async())
    }

    fn convert_message(&self, message: Message) -> WebSocketMessage {
        match message {
            Message::Text(text) => WebSocketMessage::Text(text),
            Message::Binary(data) => WebSocketMessage::Binary(data),
            Message::Ping(data) => WebSocketMessage::Ping(data),
            Message::Pong(data) => WebSocketMessage::Pong(data),
            Message::Close(_) => WebSocketMessage::Close,
            Message::Frame(_) => WebSocketMessage::Close,
        }
    }

    async fn close_async_internal(&self) -> Result<(), WebSocketError> {
        let mut connection = self.connection.lock().await;
        if let Some(ref mut ws_stream) = *connection {
            ws_stream
                .send(Message::Close(None))
                .await
                .map_err(|e| WebSocketError::protocol(e.to_string()))?;
            ws_stream
                .close(None)
                .await
                .map_err(|e| WebSocketError::protocol(e.to_string()))?;
        }
        *connection = None;
        self.connected.store(false, Ordering::Relaxed);
        Ok(())
    }

    fn close_sync(&self) -> Result<(), WebSocketError> {
        let rt: tokio::runtime::Runtime =
            tokio::runtime::Runtime::new().map_err(|e| WebSocketError::io(e.to_string()))?;
        rt.block_on(self.close_async_internal())
    }

    pub fn send_text(&mut self, text: &str) -> WebSocketResult {
        let message: Message = Message::Text(text.to_string());
        self.send_message_sync(message)
    }

    pub fn send_binary(&mut self, data: &[u8]) -> WebSocketResult {
        let message: Message = Message::Binary(data.to_vec());
        self.send_message_sync(message)
    }

    pub fn send_ping(&mut self, data: &[u8]) -> WebSocketResult {
        let message: Message = Message::Ping(data.to_vec());
        self.send_message_sync(message)
    }

    pub fn send_pong(&mut self, data: &[u8]) -> WebSocketResult {
        let message: Message = Message::Pong(data.to_vec());
        self.send_message_sync(message)
    }

    pub fn receive(&mut self) -> WebSocketMessageResult {
        self.receive_message_sync()
    }

    pub fn close(&mut self) -> WebSocketResult {
        self.close_sync()
    }

    pub fn is_connected(&self) -> bool {
        self.connected.load(Ordering::Relaxed)
    }

    pub async fn send_text_async(&mut self, text: &str) -> WebSocketResult {
        let message: Message = Message::Text(text.to_string());
        self.send_message_async(message).await
    }

    pub async fn send_binary_async(&mut self, data: &[u8]) -> WebSocketResult {
        let message: Message = Message::Binary(data.to_vec());
        self.send_message_async(message).await
    }

    pub async fn send_ping_async(&mut self, data: &[u8]) -> WebSocketResult {
        let message: Message = Message::Ping(data.to_vec());
        self.send_message_async(message).await
    }

    pub async fn send_pong_async(&mut self, data: &[u8]) -> WebSocketResult {
        let message: Message = Message::Pong(data.to_vec());
        self.send_message_async(message).await
    }

    pub async fn receive_async(&mut self) -> WebSocketMessageResult {
        self.receive_message_async().await
    }

    pub async fn close_async_method(&mut self) -> WebSocketResult {
        self.close_async_internal().await
    }
}

impl WebSocketTrait for WebSocket {
    fn send_text(&mut self, text: &str) -> WebSocketResult {
        self.send_text(text)
    }

    fn send_binary(&mut self, data: &[u8]) -> WebSocketResult {
        self.send_binary(data)
    }

    fn send_ping(&mut self, data: &[u8]) -> WebSocketResult {
        self.send_ping(data)
    }

    fn send_pong(&mut self, data: &[u8]) -> WebSocketResult {
        self.send_pong(data)
    }

    fn receive(&mut self) -> WebSocketMessageResult {
        self.receive()
    }

    fn close(&mut self) -> WebSocketResult {
        self.close()
    }

    fn is_connected(&self) -> bool {
        self.is_connected()
    }
}

impl AsyncWebSocketTrait for WebSocket {
    fn send_text<'a>(
        &'a mut self,
        text: &'a str,
    ) -> Pin<Box<dyn Future<Output = WebSocketResult> + Send + 'a>> {
        Box::pin(self.send_text_async(text))
    }

    fn send_binary<'a>(
        &'a mut self,
        data: &'a [u8],
    ) -> Pin<Box<dyn Future<Output = WebSocketResult> + Send + 'a>> {
        Box::pin(self.send_binary_async(data))
    }

    fn send_ping<'a>(
        &'a mut self,
        data: &'a [u8],
    ) -> Pin<Box<dyn Future<Output = WebSocketResult> + Send + 'a>> {
        Box::pin(self.send_ping_async(data))
    }

    fn send_pong<'a>(
        &'a mut self,
        data: &'a [u8],
    ) -> Pin<Box<dyn Future<Output = WebSocketResult> + Send + 'a>> {
        Box::pin(self.send_pong_async(data))
    }

    fn receive(&mut self) -> Pin<Box<dyn Future<Output = WebSocketMessageResult> + Send + '_>> {
        Box::pin(self.receive_async())
    }

    fn close(&mut self) -> Pin<Box<dyn Future<Output = WebSocketResult> + Send + '_>> {
        Box::pin(self.close_async_method())
    }

    fn is_connected(&self) -> bool {
        self.is_connected()
    }
}
