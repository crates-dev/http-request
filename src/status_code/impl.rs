use std::{
    fmt::{self, Display},
    str::FromStr,
};

use super::r#type::StatusCode;

impl StatusCode {
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
    fn default() -> Self {
        StatusCode::Ok
    }
}
