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
}
