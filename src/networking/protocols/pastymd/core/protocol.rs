use async_trait::async_trait;
use std::{io, iter};

use libp2p::request_response::{Codec as RequestResponseCodec, ProtocolSupport};
use libp2p::core::upgrade::{InboundUpgrade, OutboundUpgrade, NegotiationError};
use libp2p::core::UpgradeInfo;
use async_std::io::prelude::{Read,Write};
use futures::prelude::{AsyncRead,AsyncWrite};


#[derive(Clone, Debug)]
pub struct MarkdownProtocol;

impl UpgradeInfo for MarkdownProtocol {
    type Info = Self;
    type InfoIter = iter::Once<Self::Info>;

    fn protocol_info(&self) -> Self::InfoIter {
        iter::once(self.clone())
    }
}

impl AsRef<str> for MarkdownProtocol {
    fn as_ref(&self) -> &str {
        "/yugen/markdown/1.0.0"
    }
}

#[derive(Debug, Clone)]
pub struct MarkdownCodec;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MarkdownRequest(pub String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MarkdownResponse(pub String);

#[async_trait]
impl RequestResponseCodec for MarkdownCodec {
    // The Protocol Name
    type Protocol = MarkdownProtocol;

    // The Request and Response Types
    type Request = MarkdownRequest;
    type Response = MarkdownResponse;

    async fn read_request<T>(&mut self, _: &MarkdownProtocol, io: &mut T) -> io::Result<Self::Request>
    where
        T: async_std::io::AsyncRead + Unpin + Send,
    {
        let mut buf = Vec::new();
        io.read_to_end(&mut buf).await?;
        Ok(MarkdownRequest(String::from_utf8_lossy(&buf).to_string()))
    }

    async fn read_response<T>(&mut self, _: &MarkdownProtocol, io: &mut T) -> io::Result<Self::Response>
    where
        T: async_std::io::AsyncRead + Unpin + Send,
    {
        let mut buf = Vec::new();
        io.read_to_end(&mut buf).await?;
        Ok(MarkdownResponse(String::from_utf8_lossy(&buf).to_string()))
    }

    async fn write_request<T>(&mut self, _: &MarkdownProtocol, io: &mut T, MarkdownRequest(data): MarkdownRequest) -> io::Result<()>
    where
        T: async_std::io::AsyncWrite + Unpin + Send,
    {
        io.write_all(data.as_bytes()).await?;
        io.close().await?;
        Ok(())
    }

    async fn write_response<T>(&mut self, _: &MarkdownProtocol, io: &mut T, MarkdownResponse(data): MarkdownResponse) -> io::Result<()>
    where
        T: async_std::io::AsyncWrite + Unpin + Send,
    {
        io.write_all(data.as_bytes()).await?;
        io.close().await?;
        Ok(())
    }
}