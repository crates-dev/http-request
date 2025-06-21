use crate::*;

pub trait AsyncReadWrite: AsyncRead + AsyncWrite + Unpin + Send {}

pub trait ReadWrite: Read + Write {}
