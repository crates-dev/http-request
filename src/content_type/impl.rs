use super::r#type::ContentType;
use crate::request::constant::{
    APPLICATION_JSON, APPLICATION_XML, FORM_URLENCODED, TEXT_HTML, TEXT_PLAIN,
};
use serde::Serialize;
use serde_json;
use serde_xml_rs;
use std::fmt::{Debug, Display};
use std::str::FromStr;

impl ContentType {
    /// Handles the `application/json` Content-Type by serializing the provided data
    /// into a JSON string.
    ///
    /// # Type Parameters
    /// - `T`: The type of the data to be serialized, which must implement `Serialize`.
    ///
    /// # Parameters
    /// - `data`: The data to be serialized into JSON.
    ///
    /// # Returns
    /// A string containing the serialized JSON representation of the provided data.
    /// If serialization fails, it returns an empty JSON object (`{}`).
    fn get_application_json<T: Serialize + Display>(data: &T) -> String {
        serde_json::to_string(data).unwrap_or_else(|_| String::from("{}"))
    }

    /// Handles the `application/xml` Content-Type by serializing the provided data
    /// into an XML string.
    ///
    /// # Type Parameters
    /// - `T`: The type of the data to be serialized, which must implement `Serialize`.
    ///
    /// # Parameters
    /// - `data`: The data to be serialized into XML.
    ///
    /// # Returns
    /// A string containing the serialized XML representation of the provided data.
    /// If serialization fails, it returns an empty XML root element (`<root></root>`).
    fn get_application_xml<T: Serialize + Display>(data: &T) -> String {
        serde_xml_rs::to_string(data).unwrap_or_else(|_| String::from("<root></root>"))
    }

    /// Handles the `text/plain` Content-Type by formatting the provided data
    /// into a plain text string.
    ///
    /// # Type Parameters
    /// - `T`: The type of the data to be formatted, which must implement `Serialize`, `Debug`, `Clone`, and `Default`.
    ///
    /// # Parameters
    /// - `data`: The data to be formatted into plain text.
    ///
    /// # Returns
    /// A plain text string representing the provided data, formatted with the `Debug` trait.
    fn get_text_plain<T: Serialize + Debug + Clone + Default + Display>(data: &T) -> String {
        data.to_string()
    }

    /// Handles the `text/html` Content-Type by formatting the provided data
    /// into an HTML string, typically inside a simple table.
    ///
    /// # Type Parameters
    /// - `T`: The type of the data to be formatted, which must implement `Serialize`, `Debug`, `Clone`, and `Default`.
    ///
    /// # Parameters
    /// - `data`: The data to be formatted into HTML.
    ///
    /// # Returns
    /// A string containing the HTML representation of the provided data, inside a table row.
    fn get_text_html<T: Serialize + Debug + Clone + Default>(data: &T) -> String {
        let mut html: String = String::from("<table>");
        html.push_str(&format!("<tr><td>{:?}</td></tr>", data));
        html.push_str("</table>");
        html
    }

    /// Handles the `application/x-www-form-urlencoded` Content-Type by serializing
    /// the provided data into a URL-encoded string.
    ///
    /// # Type Parameters
    /// - `T`: The type of the data to be serialized, which must implement `Serialize`.
    ///
    /// # Parameters
    /// - `data`: The data to be serialized into URL-encoded format.
    ///
    /// # Returns
    /// A string containing the URL-encoded representation of the provided data.
    /// If serialization fails, it returns an empty string.
    fn get_form_url_encoded<T: Serialize + Display>(data: &T) -> String {
        serde_urlencoded::to_string(data).unwrap_or_else(|_| String::from(""))
    }

    /// Handles binary data when the `Content-Type` is unknown by formatting the
    /// provided data as a hexadecimal string.
    ///
    /// # Type Parameters
    /// - `T`: The type of the data to be formatted, which must implement `Serialize`, `Debug`, `Clone`, and `Default`.
    ///
    /// # Parameters
    /// - `data`: The data to be formatted into binary representation.
    ///
    /// # Returns
    /// A string containing the hexadecimal encoding of the provided data.
    fn get_binary<T: Serialize + Debug + Clone + Default + Display>(data: &T) -> String {
        hex::encode(data.to_string())
    }

    /// Public interface for getting a formatted body string based on the `ContentType`.
    ///
    /// This method routes the data to the appropriate handler method based on the
    /// `ContentType`, formatting the body accordingly.
    ///
    /// # Type Parameters
    /// - `T`: The type of the data to be formatted, which must implement `Serialize`, `Debug`, `Clone`, and `Default`.
    ///
    /// # Parameters
    /// - `data`: The data to be formatted into the body string.
    ///
    /// # Returns
    /// A string containing the formatted body based on the content type, such as JSON, XML, plain text, HTML, etc.
    pub fn get_body_string<T: Serialize + Debug + Clone + Default + Display>(
        &self,
        data: &T,
    ) -> String {
        match self {
            Self::ApplicationJson => Self::get_application_json(data),
            Self::ApplicationXml => Self::get_application_xml(data),
            Self::TextPlain => Self::get_text_plain(data),
            Self::TextHtml => Self::get_text_html(data),
            Self::FormUrlEncoded => Self::get_form_url_encoded(data),
            Self::Unknown => Self::get_binary(data),
        }
    }
}

impl FromStr for ContentType {
    type Err = ();

    fn from_str(data: &str) -> Result<Self, Self::Err> {
        match data.to_lowercase().as_str() {
            _data if _data == APPLICATION_JSON => Ok(Self::ApplicationJson),
            _data if _data == APPLICATION_XML => Ok(Self::ApplicationXml),
            _data if _data == TEXT_PLAIN => Ok(Self::TextPlain),
            _data if _data == TEXT_HTML => Ok(Self::TextHtml),
            _data if _data == FORM_URLENCODED => Ok(Self::FormUrlEncoded),
            _ => Ok(Self::Unknown),
        }
    }
}

impl Default for ContentType {
    fn default() -> Self {
        Self::Unknown
    }
}
