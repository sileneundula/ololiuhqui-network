use libp2p::request_response::{ProtocolSupport, RequestResponse, RequestResponseCodec, RequestResponseConfig};
use libp2p::core::upgrade::{InboundUpgrade, OutboundUpgrade};
use std::io;

pub mod core;

#[derive(Clone)]
struct IMXProtocol();

#[derive(Clone)]
struct MyCodec;

#[derive(Debug, Clone)]
struct MyRequest(Vec<u8>);

#[derive(Debug, Clone)]
struct MyResponse(Vec<u8>);

#[async_trait::async_trait]
impl RequestResponseCodec for MyCodec {
    type Protocol = IMXProtocol;
    type Request = MyRequest;
    type Response = MyResponse;

    async fn read_request<T>(&mut self, _: &MyProtocol, io: &mut T) -> io::Result<Self::Request>
    where T: AsyncRead + Unpin + Send {
        let mut buf = Vec::new();
        io.read_to_end(&mut buf).await?;
        Ok(MyRequest(buf))
    }

    async fn read_response<T>(&mut self, _: &MyProtocol, io: &mut T) -> io::Result<Self::Response>
    where T: AsyncRead + Unpin + Send {
        let mut buf = Vec::new();
        io.read_to_end(&mut buf).await?;
        Ok(MyResponse(buf))
    }

    async fn write_request<T>(&mut self, _: &MyProtocol, io: &mut T, MyRequest(data): MyRequest) -> io::Result<()>
    where T: AsyncWrite + Unpin + Send {
        io.write_all(&data).await
    }

    async fn write_response<T>(&mut self, _: &MyProtocol, io: &mut T, MyResponse(data): MyResponse) -> io::Result<()>
    where T: AsyncWrite + Unpin + Send {
        io.write_all(&data).await
    }
}
