use crate::*;

pub struct ProxyTunnelStream {
    pub(super) inner: BoxAsyncReadWrite,
}

pub struct SyncProxyTunnelStream {
    pub(super) inner: BoxReadWrite,
}
