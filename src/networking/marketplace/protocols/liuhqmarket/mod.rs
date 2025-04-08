use libp2p::request_response::{RequestResponse, ProtocolSupport, RequestResponseCodec, RequestResponseEvent, RequestResponseMessage};
use libp2p::swarm::{NetworkBehaviour, Swarm};

pub struct LiuhqMarket {
    pub protocol: RequestResponse<LiuhqMarketCodec>,
}
