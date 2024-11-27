#[derive(Debug, Clone, PartialEq)]
pub struct Url {
    pub scheme: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub host: Option<String>,
    pub port: Option<u16>,
    pub path: Option<String>,
    pub query: Option<String>,
    pub fragment: Option<String>,
}
