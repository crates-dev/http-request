use std::str::FromStr;

use crate::constant::http::{
    APPLICATION_JSON, APPLICATION_XML, FORM_URLENCODED, TEXT_HTML, TEXT_PLAIN,
};

#[derive(Debug, PartialEq)]
pub enum ContentType {
    ApplicationJson,
    ApplicationXml,
    TextPlain,
    TextHtml,
    FormUrlEncoded,
    Unknown,
}

impl FromStr for ContentType {
    type Err = ();

    fn from_str(data: &str) -> Result<Self, Self::Err> {
        match data.to_lowercase().as_str() {
            _data if _data == APPLICATION_JSON => Ok(ContentType::ApplicationJson),
            _data if _data == APPLICATION_XML => Ok(ContentType::ApplicationXml),
            _data if _data == TEXT_PLAIN => Ok(ContentType::TextPlain),
            _data if _data == TEXT_HTML => Ok(ContentType::TextHtml),
            _data if _data == FORM_URLENCODED => Ok(ContentType::FormUrlEncoded),
            _ => Ok(ContentType::Unknown),
        }
    }
}

impl Default for ContentType {
    fn default() -> Self {
        Self::Unknown
    }
}
