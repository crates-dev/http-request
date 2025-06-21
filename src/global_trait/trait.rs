use crate::*;

pub trait AsyncReadWrite: AsyncRead + AsyncWrite + Unpin + Send {}

pub trait ReadWrite: std::io::Read + std::io::Write {}
