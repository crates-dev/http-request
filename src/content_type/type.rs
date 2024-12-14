/// Represents different types of HTTP content types, such as JSON, XML, plain text, HTML,
/// form URL encoded, and an unknown type.
#[derive(Debug, PartialEq, Eq)]
pub enum ContentType {
    /// Represents the `application/json` content type.
    ApplicationJson,

    /// Represents the `application/xml` content type.
    ApplicationXml,

    /// Represents the `text/plain` content type.
    TextPlain,

    /// Represents the `text/html` content type.
    TextHtml,

    /// Represents the `application/x-www-form-urlencoded` content type.
    FormUrlEncoded,

    /// Represents an unknown or unrecognized content type.
    Unknown,
}
