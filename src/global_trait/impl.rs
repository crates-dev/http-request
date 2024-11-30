use super::r#trait::ReadWrite;

/// Provides a blanket implementation for the `ReadWrite` trait.
///
/// This implementation allows any type that implements both `std::io::Read`
/// and `std::io::Write` to automatically be treated as implementing the
/// `ReadWrite` trait. This is a common approach in Rust to extend the
/// functionality of existing types without additional boilerplate.
///
/// # Type Parameters
///
/// - `T`: A generic type that must implement both `std::io::Read` and `std::io::Write`.
///
/// This blanket implementation is useful for working with types like `TcpStream`
/// or custom stream implementations that support both reading and writing.
impl<T: std::io::Read + std::io::Write> ReadWrite for T {}
