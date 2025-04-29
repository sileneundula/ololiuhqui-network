use libp2p::core::upgrade::{InboundUpgrade, OutboundUpgrade, NegotiationError};
use libp2p::core::UpgradeInfo;
use std::{io, iter};

#[derive(Debug, Clone)]
pub struct LiuhqMarketProto;

impl UpgradeInfo for LiuhqMarketProto {
    type Info = &'static str;
    type InfoIter = iter::Once<Self::Info>;

    fn protocol_info(&self) -> Self::InfoIter {
        iter::once("/liuhqmarket/1.0.0")
    }
}