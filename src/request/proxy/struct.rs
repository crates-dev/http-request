use crate::*;

/// Asynchronous proxy tunnel stream wrapper.
///
/// Provides unified interface for different proxy implementations.
pub struct ProxyTunnelStream {
    /// The underlying asynchronous read/write stream.
    pub(super) inner: BoxAsyncReadWrite,
}

/// Synchronous proxy tunnel stream wrapper.
///
/// Provides unified interface for different proxy implementations.
pub struct SyncProxyTunnelStream {
    /// The underlying synchronous read/write stream.
    pub(super) inner: BoxReadWrite,
}
