use super::r#type::HttpStatusCode;

impl HttpStatusCode {
    pub fn code(&self) -> u16 {
        match self {
            HttpStatusCode::Ok => 200,
            HttpStatusCode::Created => 201,
            HttpStatusCode::NoContent => 204,
            HttpStatusCode::BadRequest => 400,
            HttpStatusCode::Unauthorized => 401,
            HttpStatusCode::Forbidden => 403,
            HttpStatusCode::NotFound => 404,
            HttpStatusCode::InternalServerError => 500,
            HttpStatusCode::NotImplemented => 501,
            HttpStatusCode::BadGateway => 502,
        }
    }

    pub fn reason_phrase(&self) -> &str {
        match self {
            HttpStatusCode::Ok => "OK",
            HttpStatusCode::Created => "Created",
            HttpStatusCode::NoContent => "No Content",
            HttpStatusCode::BadRequest => "Bad Request",
            HttpStatusCode::Unauthorized => "Unauthorized",
            HttpStatusCode::Forbidden => "Forbidden",
            HttpStatusCode::NotFound => "Not Found",
            HttpStatusCode::InternalServerError => "Internal Server Error",
            HttpStatusCode::NotImplemented => "Not Implemented",
            HttpStatusCode::BadGateway => "Bad Gateway",
        }
    }
}
