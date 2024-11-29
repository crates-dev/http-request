use std::{
    fmt::{self, Display},
    str::FromStr,
};

use super::r#type::StatusCode;

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
    /// - `StatusCode::Ok` returns 200.
    /// - `StatusCode::BadRequest` returns 400.
    /// - `StatusCode::Unknown` returns 0 (the default for unrecognized status codes).
    ///
    /// # Parameters
    /// - `&self`: A reference to the `StatusCode` enum instance. This represents the specific variant of the `StatusCode` enum that the method is called on.
    ///
    /// # Return Value
    /// - `u16`: The numeric HTTP status code associated with the `StatusCode` variant. For example:
    ///   - `StatusCode::Ok` returns `200`.
    ///   - `StatusCode::BadRequest` returns `400`.
    ///   - `StatusCode::Unknown` returns `0`.
    pub fn code(&self) -> u16 {
        match self {
            StatusCode::Ok => 200,
            StatusCode::Created => 201,
            StatusCode::NoContent => 204,
            StatusCode::BadRequest => 400,
            StatusCode::Unauthorized => 401,
            StatusCode::Forbidden => 403,
            StatusCode::NotFound => 404,
            StatusCode::InternalServerError => 500,
            StatusCode::NotImplemented => 501,
            StatusCode::BadGateway => 502,
            StatusCode::Unknown => 0,
        }
    }
}

impl Display for StatusCode {
    /// Formats the `StatusCode` as a human-readable string, such as "OK" or "Not Found".
    ///
    /// This method formats the `StatusCode` variant into a string that can be easily displayed.
    /// For example:
    /// - `StatusCode::Ok` formats to `"OK"`.
    /// - `StatusCode::BadRequest` formats to `"Bad Request"`.
    ///
    /// # Parameters
    /// - `&self`: A reference to the `StatusCode` enum instance, representing the specific status code variant.
    /// - `f`: A mutable reference to the `fmt::Formatter` that is responsible for formatting the output.
    ///
    /// # Return Value
    /// - `fmt::Result`: A result that indicates whether the formatting was successful. The result is `Ok(())` if the formatting was successful, or an error if it failed.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let res: &str = match self {
            StatusCode::Ok => "OK",
            StatusCode::Created => "Created",
            StatusCode::NoContent => "No Content",
            StatusCode::BadRequest => "Bad Request",
            StatusCode::Unauthorized => "Unauthorized",
            StatusCode::Forbidden => "Forbidden",
            StatusCode::NotFound => "Not Found",
            StatusCode::InternalServerError => "Internal Server Error",
            StatusCode::NotImplemented => "Not Implemented",
            StatusCode::BadGateway => "Bad Gateway",
            StatusCode::Unknown => "Unknown",
        };
        write!(f, "{}", res)
    }
}

impl FromStr for StatusCode {
    type Err = ();

    /// Converts a string representation of an HTTP status code (either numeric or textual)
    /// into the corresponding `StatusCode` enum variant.
    ///
    /// This method allows parsing of both numeric status codes (e.g., "200", "404") and their
    /// corresponding textual representations (e.g., "OK", "Not Found") into the `StatusCode` enum.
    ///
    /// # Parameters
    /// - `code_str`: A string slice (`&str`) representing the status code. This can be either numeric (e.g., "200") or textual (e.g., "OK").
    ///
    /// # Return Value
    /// - `Result<StatusCode, ()>`: Returns a `Result`:
    ///   - `Ok(StatusCode::Ok)` for "200" or "OK".
    ///   - `Ok(StatusCode::BadRequest)` for "400" or "Bad Request".
    ///   - `Ok(StatusCode::NotFound)` for "404" or "Not Found".
    ///   - `Ok(StatusCode::Unknown)` if the string is not a recognized status code.
    fn from_str(code_str: &str) -> Result<Self, Self::Err> {
        match code_str {
            "200" | "OK" => Ok(StatusCode::Ok),
            "201" | "Created" => Ok(StatusCode::Created),
            "204" | "No Content" => Ok(StatusCode::NoContent),
            "400" | "Bad Request" => Ok(StatusCode::BadRequest),
            "401" | "Unauthorized" => Ok(StatusCode::Unauthorized),
            "403" | "Forbidden" => Ok(StatusCode::Forbidden),
            "404" | "Not Found" => Ok(StatusCode::NotFound),
            "500" | "Internal Server Error" => Ok(StatusCode::InternalServerError),
            "501" | "Not Implemented" => Ok(StatusCode::NotImplemented),
            "502" | "Bad Gateway" => Ok(StatusCode::BadGateway),
            _ => Ok(StatusCode::Unknown),
        }
    }
}

impl Default for StatusCode {
    /// Returns the default status code, which is `StatusCode::Ok` (HTTP 200).
    ///
    /// This method provides a default status code, typically used when no specific status code is set.
    /// It returns `StatusCode::Ok`, which corresponds to HTTP status code 200 (OK).
    ///
    /// # Parameters
    /// - None
    ///
    /// # Return Value
    /// - `StatusCode::Ok`: The default HTTP status code, which is 200 (OK).
    fn default() -> Self {
        StatusCode::Ok
    }
}
