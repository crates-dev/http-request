use super::r#type::StatusCode;
use std::{
    fmt::{self, Display},
    str::FromStr,
};

/// The `StatusCode` enum represents the HTTP status codes.
///
/// It maps common HTTP status codes to their respective meanings. It provides methods to retrieve
/// the corresponding numeric code as well as the associated status text. Additionally, it implements
/// conversion from a string representation of the status code.
///
/// # Variants
/// - `Ok`: HTTP status 200, indicating a successful request.
/// - `Created`: HTTP status 201, indicating that the request was successful and resulted in a resource creation.
/// - `NoContent`: HTTP status 204, indicating that the request was successful, but there is no content to return.
/// - `BadRequest`: HTTP status 400, indicating a bad request, often due to incorrect syntax or invalid data.
/// - `Unauthorized`: HTTP status 401, indicating that authentication is required and has failed or not been provided.
/// - `Forbidden`: HTTP status 403, indicating that the server understands the request but refuses to authorize it.
/// - `NotFound`: HTTP status 404, indicating that the requested resource could not be found.
/// - `InternalServerError`: HTTP status 500, indicating that the server encountered an internal error.
/// - `NotImplemented`: HTTP status 501, indicating that the server does not support the functionality required to fulfill the request.
/// - `BadGateway`: HTTP status 502, indicating that the server, while acting as a gateway or proxy, received an invalid response from an upstream server.
/// - `Unknown`: A default variant for unrecognized or undefined status codes.
impl StatusCode {
    /// Returns the numeric HTTP status code associated with this status code variant.
    ///
    /// This method returns the corresponding HTTP numeric status code based on the `StatusCode` variant.
    /// For example:
    /// - `Self::Ok` returns 200.
    /// - `Self::BadRequest` returns 400.
    /// - `Self::Unknown` returns 0 (the default for unrecognized status codes).
    ///
    /// # Parameters
    /// - `&self`: A reference to the `StatusCode` enum instance. This represents the specific variant of the `StatusCode` enum that the method is called on.
    ///
    /// # Return Value
    /// - `u16`: The numeric HTTP status code associated with the `StatusCode` variant. For example:
    ///   - `Self::Ok` returns `200`.
    ///   - `Self::BadRequest` returns `400`.
    ///   - `Self::Unknown` returns `0`.
    pub fn code(&self) -> u16 {
        match self {
            Self::Ok => 200,
            Self::Created => 201,
            Self::NoContent => 204,
            Self::BadRequest => 400,
            Self::Unauthorized => 401,
            Self::Forbidden => 403,
            Self::NotFound => 404,
            Self::InternalServerError => 500,
            Self::NotImplemented => 501,
            Self::BadGateway => 502,
            Self::Unknown => 0,
        }
    }

    pub fn same(&self, code_str: &str) -> bool {
        self.code().to_string() == code_str || self.to_string() == code_str
    }
}

impl Display for StatusCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let res: &str = match self {
            Self::Ok => "OK",
            Self::Created => "Created",
            Self::NoContent => "No Content",
            Self::BadRequest => "Bad Request",
            Self::Unauthorized => "Unauthorized",
            Self::Forbidden => "Forbidden",
            Self::NotFound => "Not Found",
            Self::InternalServerError => "Internal Server Error",
            Self::NotImplemented => "Not Implemented",
            Self::BadGateway => "Bad Gateway",
            Self::Unknown => "Unknown",
        };
        write!(f, "{}", res)
    }
}

impl FromStr for StatusCode {
    type Err = ();

    fn from_str(code_str: &str) -> Result<Self, Self::Err> {
        match code_str {
            _code_str if Self::Ok.same(_code_str) => Ok(Self::Ok),
            _code_str if Self::Created.same(_code_str) => Ok(Self::Created),
            _code_str if Self::NoContent.same(_code_str) => Ok(Self::NoContent),
            _code_str if Self::BadRequest.same(_code_str) => Ok(Self::BadRequest),
            _code_str if Self::Unauthorized.same(_code_str) => Ok(Self::Unauthorized),
            _code_str if Self::Forbidden.same(_code_str) => Ok(Self::Forbidden),
            _code_str if Self::NotFound.same(_code_str) => Ok(Self::NotFound),
            _code_str if Self::InternalServerError.same(_code_str) => Ok(Self::InternalServerError),
            _code_str if Self::NotImplemented.same(_code_str) => Ok(Self::NotImplemented),
            _code_str if Self::BadGateway.same(_code_str) => Ok(Self::BadGateway),
            _ => Ok(Self::Unknown),
        }
    }
}

impl Default for StatusCode {
    fn default() -> Self {
        Self::Ok
    }
}
