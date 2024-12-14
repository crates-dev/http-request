// TODO:
/// A trait that combines `std::io::Read` and `std::io::Write`.
///
/// The `ReadWrite` trait serves as a composite trait, requiring that any type implementing it
/// must also implement both the `std::io::Read` and `std::io::Write` traits. This abstraction
/// is useful for defining APIs or types that need bidirectional I/O operations.
pub trait ReadWrite: std::io::Read + std::io::Write {}
