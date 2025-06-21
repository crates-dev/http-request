use crate::*;

impl Default for WebSocketConfig {
    fn default() -> Self {
        Self {
            timeout: DEFAULT_TIMEOUT,
            url_obj: HttpUrlComponents::default(),
            buffer: DEFAULT_BUFFER_SIZE,
            protocols: Vec::new(),
            proxy: None,
        }
    }
}
