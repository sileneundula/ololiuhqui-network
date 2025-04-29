pub mod core;

use libp2p::request_response::{Codec as RequestResponseCodec};
use libp2p::core::upgrade::{InboundUpgrade, OutboundUpgrade, NegotiationError};
use libp2p::core::UpgradeInfo;
use async_trait::async_trait;
//use tokio::io::{AsyncRead, AsyncWrite, AsyncReadExt, AsyncWriteExt};
use futures::io::{AsyncRead,AsyncWrite,AsyncWriteExt,AsyncReadExt};
use futures::io;

#[derive(Clone)]
pub struct KeyServiceProtocol;

impl AsRef<str> for KeyServiceProtocol {
    fn as_ref(&self) -> &str {
        "/keyservice/1.0.0"
    }
}

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
        Ok(KeyServiceRequest(String::from_utf8_lossy(&buf).to_string()))
    }

    async fn read_response<T>(&mut self, _: &Self::Protocol, io: &mut T) -> io::Result<KeyServiceResponse>
    where T: AsyncRead + Unpin + Send {
        let mut buf = Vec::new();
        io.read_to_end(&mut buf).await?;
        Ok(KeyServiceResponse(buf))
    }

    async fn write_request<T>(&mut self, _: &Self::Protocol, io: &mut T, KeyServiceRequest(path): KeyServiceRequest) -> io::Result<()>
    where T: AsyncWrite + Unpin + Send {
        io.write_all(path.as_bytes()).await?;
        io.close().await
    }

    async fn write_response<T>(&mut self, _: &Self::Protocol, io: &mut T, KeyServiceResponse(data): KeyServiceResponse) -> io::Result<()>
    where T: AsyncWrite + Unpin + Send {
        io.write_all(&data).await?;
        io.close().await
    }
}

impl UpgradeInfo for KeyServiceProtocol {
    type Info = &'static str;
    type InfoIter = std::iter::Once<Self::Info>;

    fn protocol_info(&self) -> Self::InfoIter {
        std::iter::once("/keyservice/1.0.0")
    }
}
