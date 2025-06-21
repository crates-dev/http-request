use crate::*;

impl<T: AsyncRead + AsyncWrite + Unpin + Send> AsyncReadWrite for T {}

impl<T: Read + Write> ReadWrite for T {}
