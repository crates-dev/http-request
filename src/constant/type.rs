/// The name of the application.
///
/// This constant represents the name of the application used for
/// identifying the current application context.
pub static APP_NAME: &str = "http-request";

/// The default timeout value for HTTP requests, represented in milliseconds.
/// This value is used when no specific timeout is provided, ensuring that requests
/// have a reasonable maximum duration before timing out.
pub const DEFAULT_TIMEOUT: u64 = 100_000;
