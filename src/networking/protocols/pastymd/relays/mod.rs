use libp2p::{
    core::upgrade::{InboundUpgrade, Negotiated as NegotiatedSubstream},
    swarm::NegotiatedSubstream,
    core::Protocol,
};

pub struct PastyMDRelay;

impl ProtocolName for PastyMDRelay {
    fn protocol_name(&self) -> &[u8] {
        b"/pasty-md-relay/1.0.0"
    }
}

#[derive(Debug, Clone)]
pub struct PastyMDRelayRequest {
    pub id: String,
}

#[derive(Debug, Clone)]
pub struct PastyMDRelayResponse {
    pub success: bool,
}

impl InboundUpgrade<NegotiatedSubstream> for PastyMDRelay {
    type Output = PastyMDRelayRequest;
    type Error = io::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Output, Self::Error>> + Send>>;

    fn upgrade_inbound(self, mut socket: NegotiatedSubstream, _: Self::Info) -> Self::Future {
        Box::pin(async move {
            let data = read_one(&mut socket, 1024).await?;
            let id = String::from_utf8(data).map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid UTF-8"))?;
            Ok(PastyMDRelayRequest { id })
        })
    }
}