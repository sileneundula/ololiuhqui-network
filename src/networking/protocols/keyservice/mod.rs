pub mod core;

use libp2p::request_response::{RequestResponseCodec, RequestResponse, UpgradeInfo};

#[derive(Clone)]
pub struct KeyServiceProtocol();

#[derive(Debug, Clone)]
pub struct KeyServiceRequest(pub String); // e.g., "/index.html"

#[derive(Debug, Clone)]
pub struct KeyServiceResponse(pub Vec<u8>);

#[derive(Clone)]
pub struct KeyServiceCodec;

#[async_trait::async_trait]
impl RequestResponseCodec for KeyServiceCodec {
    type Protocol = KeyServiceProtocol;
    type Request = KeyServiceRequest;
    type Response = KeyServiceResponse;

    async fn read_request<T>(&mut self, _: &Self::Protocol, io: &mut T) -> io::Result<KeyServiceRequest>
    where T: AsyncRead + Unpin + Send {
        let mut buf = Vec::new();
        io.read_to_end(&mut buf).await?;
        Ok(StaticRequest(String::from_utf8_lossy(&buf).to_string()))
    }

    async fn read_response<T>(&mut self, _: &Self::Protocol, io: &mut T) -> io::Result<KeyServiceResponse>
    where T: AsyncRead + Unpin + Send {
        let mut buf = Vec::new();
        io.read_to_end(&mut buf).await?;
        Ok(StaticResponse(buf))
    }

    async fn write_request<T>(&mut self, _: &Self::Protocol, io: &mut T, StaticRequest(path): KeyServiceRequest) -> io::Result<()>
    where T: AsyncWrite + Unpin + Send {
        io.write_all(path.as_bytes()).await?;
        io.close().await
    }

    async fn write_response<T>(&mut self, _: &Self::Protocol, io: &mut T, StaticResponse(data): KeyServiceResponse) -> io::Result<()>
    where T: AsyncWrite + Unpin + Send {
        io.write_all(&data).await?;
        io.close().await
    }
}

impl UpgradeInfo for StaticProtocol {
    type Info = &'static [u8];
    type InfoIter = std::iter::Once<Self::Info>;

    fn protocol_info(&self) -> Self::InfoIter {
        std::iter::once(b"/keyservice/1.0.0")
    }
}
