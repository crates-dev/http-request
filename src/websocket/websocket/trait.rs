use crate::*;

pub trait WebSocketTrait: Send + Sync {
    fn send_text(&mut self, text: &str) -> WebSocketResult;
    fn send_binary(&mut self, data: &[u8]) -> WebSocketResult;
    fn send_ping(&mut self, data: &[u8]) -> WebSocketResult;
    fn send_pong(&mut self, data: &[u8]) -> WebSocketResult;
    fn receive(&mut self) -> WebSocketMessageResult;
    fn close(&mut self) -> WebSocketResult;
    fn is_connected(&self) -> bool;
}

pub trait AsyncWebSocketTrait: Send + Sync {
    fn send_text<'a>(
        &'a mut self,
        text: &'a str,
    ) -> Pin<Box<dyn Future<Output = WebSocketResult> + Send + 'a>>;
    fn send_binary<'a>(
        &'a mut self,
        data: &'a [u8],
    ) -> Pin<Box<dyn Future<Output = WebSocketResult> + Send + 'a>>;
    fn send_ping<'a>(
        &'a mut self,
        data: &'a [u8],
    ) -> Pin<Box<dyn Future<Output = WebSocketResult> + Send + 'a>>;
    fn send_pong<'a>(
        &'a mut self,
        data: &'a [u8],
    ) -> Pin<Box<dyn Future<Output = WebSocketResult> + Send + 'a>>;
    fn receive<'a>(
        &'a mut self,
    ) -> Pin<Box<dyn Future<Output = WebSocketMessageResult> + Send + 'a>>;
    fn close<'a>(&'a mut self) -> Pin<Box<dyn Future<Output = WebSocketResult> + Send + 'a>>;
    fn is_connected(&self) -> bool;
}
