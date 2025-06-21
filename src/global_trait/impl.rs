use crate::*;

impl<T: AsyncRead + AsyncWrite + Unpin + Send> AsyncReadWrite for T {}

impl<T: std::io::Read + std::io::Write> ReadWrite for T {}
