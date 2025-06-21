use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Config {
    pub(crate) timeout: u64,
    pub(crate) url_obj: HttpUrlComponents,
    pub(crate) redirect: bool,
    pub(crate) max_redirect_times: usize,
    pub(crate) redirect_times: usize,
    pub(crate) http_version: HttpVersion,
    pub(crate) buffer: usize,
    pub(crate) decode: bool,
    pub(crate) proxy: Option<ProxyConfig>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ProxyConfig {
    pub(crate) proxy_type: ProxyType,
    pub(crate) host: String,
    pub(crate) port: u16,
    pub(crate) username: Option<String>,
    pub(crate) password: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ProxyType {
    Http,
    Https,
    Socks5,
}
