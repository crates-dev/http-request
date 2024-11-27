use super::r#type::Protocol;
use std::fmt::{self, Display};

impl Default for Protocol {
    fn default() -> Self {
        Protocol::HTTP
    }
}

impl Protocol {
    pub fn new() -> Self {
        Protocol::default()
    }

    pub fn value(&self) -> &str {
        match self {
            Protocol::HTTP => "http",
            Protocol::HTTPS => "https",
        }
    }

    pub fn is_http(&self) -> bool {
        self.value() == Protocol::HTTP.value()
    }

    pub fn is_https(&self) -> bool {
        self.value() == Protocol::HTTPS.value()
    }

    pub fn get_port(&self) -> u16 {
        match self {
            Protocol::HTTP => 80,
            Protocol::HTTPS => 443,
        }
    }
}

impl Display for Protocol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}
