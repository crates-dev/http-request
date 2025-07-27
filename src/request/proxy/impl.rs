use crate::*;

/// Implementation of ProxyTunnelStream methods.
impl ProxyTunnelStream {
    /// Creates a new ProxyTunnelStream from an async read/write stream.
    ///
    /// # Arguments
    ///
    /// - `BoxAsyncReadWrite` - The async stream to wrap.
    ///
    /// # Returns
    ///
    /// - `ProxyTunnelStream` - The new proxy tunnel stream.
    pub(crate) fn new(stream: BoxAsyncReadWrite) -> Self {
        Self { inner: stream }
    }
}

/// AsyncRead implementation for ProxyTunnelStream.
///
/// Delegates all operations to the underlying stream.
impl AsyncRead for ProxyTunnelStream {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        Pin::new(&mut self.inner).poll_read(cx, buf)
    }
}

/// AsyncWrite implementation for ProxyTunnelStream.
///
/// Delegates all operations to the underlying stream.
impl AsyncWrite for ProxyTunnelStream {
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<Result<usize, std::io::Error>> {
        Pin::new(&mut self.inner).poll_write(cx, buf)
    }

    fn poll_flush(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<(), std::io::Error>> {
        Pin::new(&mut self.inner).poll_flush(cx)
    }

    fn poll_shutdown(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<(), std::io::Error>> {
        Pin::new(&mut self.inner).poll_shutdown(cx)
    }
}

impl Unpin for ProxyTunnelStream {}

/// Implementation of SyncProxyTunnelStream methods.
impl SyncProxyTunnelStream {
    /// Creates a new SyncProxyTunnelStream from a sync read/write stream.
    ///
    /// # Arguments
    ///
    /// - `BoxReadWrite` - The sync stream to wrap.
    ///
    /// # Returns
    ///
    /// - `SyncProxyTunnelStream` - The new sync proxy tunnel stream.
    pub(crate) fn new(stream: BoxReadWrite) -> Self {
        Self { inner: stream }
    }
}

/// Read implementation for SyncProxyTunnelStream.
///
/// Delegates all operations to the underlying stream.
impl Read for SyncProxyTunnelStream {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.inner.read(buf)
    }
}

/// Write implementation for SyncProxyTunnelStream.
///
/// Delegates all operations to the underlying stream.
impl Write for SyncProxyTunnelStream {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.inner.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.inner.flush()
    }
}
