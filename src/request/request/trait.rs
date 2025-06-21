use crate::*;

pub(crate) trait AsyncReadWrite: AsyncRead + AsyncWrite + Unpin + Send {}
pub(crate) trait ReadWrite: Read + Write {}

pub trait AsyncRequestTrait: Send + Debug {
    type RequestResult: Sized;

    fn send(&mut self) -> Pin<Box<dyn Future<Output = Self::RequestResult> + Send + '_>>;
}

pub trait RequestTrait: Send + Debug {
    type RequestResult: Sized;

    fn send(&mut self) -> Self::RequestResult;
}
