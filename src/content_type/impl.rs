use super::r#type::ContentType;
use serde::Serialize;
use serde_json;
use serde_xml_rs;
use std::fmt::Debug;

impl ContentType {
    /// Handles `application/json` ContentType
    fn get_application_json<T: Serialize>(data: &T) -> String {
        serde_json::to_string(data).unwrap_or_else(|_| String::from("{}"))
    }

    /// Handles `application/xml` ContentType
    fn get_application_xml<T: Serialize>(data: &T) -> String {
        serde_xml_rs::to_string(data).unwrap_or_else(|_| String::from("<root></root>"))
    }

    /// Handles `text/plain` ContentType
    fn get_text_plain<T: Serialize + Debug + Clone + Default>(data: &T) -> String {
        format!("{:?}", data)
    }

    /// Handles `text/html` ContentType
    fn get_text_html<T: Serialize + Debug + Clone + Default>(data: &T) -> String {
        let mut html: String = String::from("<table>");
        html.push_str(&format!("<tr><td>{:?}</td></tr>", data));
        html.push_str("</table>");
        html
    }

    /// Handles `application/x-www-form-urlencoded` ContentType
    fn get_form_url_encoded<T: Serialize>(data: &T) -> String {
        serde_urlencoded::to_string(data).unwrap_or_else(|_| String::from(""))
    }

    /// Handles binary data (when ContentType is unknown)
    fn get_binary<T: Serialize + Debug + Clone + Default>(data: &T) -> String {
        let raw_data = format!("{:?}", data);
        hex::encode(raw_data)
    }

    /// Public interface for getting a formatted body string
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
