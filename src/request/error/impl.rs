// TODO:
use super::r#type::Error;
use std::{
    error::Error as StdError,
    fmt::{self, Display},
};

impl StdError for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::InvalidUrl => write!(f, "Invalid URL"),
            Error::TcpStreamConnectError => write!(f, "TCP Stream Connection Error"),
            Error::RequestError => write!(f, "Request Error"),
            Error::MethodsNotSupport => write!(f, "Unsupported HTTP Method"),
            Error::ReadConnectionError => write!(f, "Connection Read Error"),
            Error::TlsConnectorBuildError => write!(f, "TLS Connector Build Error"),
            Error::SetReadTimeoutError => write!(f, "Failed to Set Read Timeout"),
            Error::SetWriteTimeoutError => write!(f, "Failed to Set Write Timeout"),
            Error::TlsStreamConnectError => write!(f, "TLS Stream Connection Error"),
            Error::MaxRedirectTimes => write!(f, "Max Redirect Times"),
            Error::RedirectUrlDeadLoop => write!(f, "Redirect URL Dead Loop"),
            Error::RedirectInvalidUrl => write!(f, "Redirect Invalid Url"),
            Error::NeedOpenRedirect => write!(f, "Need Open Redirect"),
        }
    }
}
