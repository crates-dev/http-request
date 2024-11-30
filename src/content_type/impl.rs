use super::r#type::ContentType;
use crate::constant::http::{
    APPLICATION_JSON, APPLICATION_XML, FORM_URLENCODED, TEXT_HTML, TEXT_PLAIN,
};
use serde::Serialize;
use serde_json;
use serde_xml_rs;
use std::fmt::Debug;
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
    fn get_application_json<T: Serialize>(data: &T) -> String {
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
    fn get_application_xml<T: Serialize>(data: &T) -> String {
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
    fn get_text_plain<T: Serialize + Debug + Clone + Default>(data: &T) -> String {
        format!("{:?}", data)
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
    fn get_form_url_encoded<T: Serialize>(data: &T) -> String {
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
    fn get_binary<T: Serialize + Debug + Clone + Default>(data: &T) -> String {
        let raw_data = format!("{:?}", data);
        hex::encode(raw_data)
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
    pub fn get_body_string<T: Serialize + Debug + Clone + Default>(&self, data: &T) -> String {
        match self {
            ContentType::ApplicationJson => ContentType::get_application_json(data),
            ContentType::ApplicationXml => ContentType::get_application_xml(data),
            ContentType::TextPlain => ContentType::get_text_plain(data),
            ContentType::TextHtml => ContentType::get_text_html(data),
            ContentType::FormUrlEncoded => ContentType::get_form_url_encoded(data),
            ContentType::Unknown => ContentType::get_binary(data),
        }
    }
}

impl FromStr for ContentType {
    type Err = ();

    /// Parses a string to convert it into a `ContentType` enum variant.
    ///
    /// This implementation compares the input string (case-insensitive) with predefined content
    /// types and returns the corresponding `ContentType` variant.
    ///
    /// # Parameters
    /// - `data`: The string representing the content type to be parsed.
    ///
    /// # Returns
    /// - A `Result` containing the matching `ContentType` variant if the string matches a known
    ///   content type, or `ContentType::Unknown` if the string does not match any predefined content
    ///   type.
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
    /// Returns the default `ContentType`, which is `ContentType::Unknown`.
    ///
    /// This is used when no specific content type is provided or when the content type is not recognized.
    ///
    /// # Returns
    /// - The `ContentType::Unknown` variant.
    fn default() -> Self {
        Self::Unknown
    }
}
