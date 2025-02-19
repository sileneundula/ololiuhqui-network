use async_trait::async_trait;
use std::{io, iter, pin::Pin, task::{Context, Poll}};

use futures::prelude::*;
use libp2p::{
    core::upgrade::{read_one, write_one, ProtocolName, UpgradeInfo},
    swarm::{NegotiatedSubstream, NetworkBehaviour, NetworkBehaviourAction, PollParameters, ProtocolsHandler, ProtocolsHandlerEvent, SubstreamProtocol},
    InboundUpgrade, OutboundUpgrade, PeerId,
};

// Define the protocol name
#[derive(Clone)]
struct RegistrarProtocol;

impl ProtocolName for RegistrarProtocol {
    fn protocol_name(&self) -> &[u8] {
        b"/registrar/1.0.0"
    }
}

// Define the request and response types
#[derive(Debug, Clone)]
struct RegistrarRequest {
    pub id: String,
}

#[derive(Debug, Clone)]
struct RegistrarResponse {
    pub success: bool,
}

// Implement the inbound upgrade for the protocol
impl InboundUpgrade<NegotiatedSubstream> for RegistrarProtocol {
    type Output = RegistrarRequest;
    type Error = io::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Output, Self::Error>> + Send>>;

    fn upgrade_inbound(self, mut socket: NegotiatedSubstream, _: Self::Info) -> Self::Future {
        Box::pin(async move {
            let data = read_one(&mut socket, 1024).await?;
            let id = String::from_utf8(data).map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid UTF-8"))?;
            Ok(RegistrarRequest { id })
        })
    }
}

// Implement the outbound upgrade for the protocol
impl OutboundUpgrade<NegotiatedSubstream> for RegistrarProtocol {
    type Output = RegistrarResponse;
    type Error = io::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Output, Self::Error>> + Send>>;

    fn upgrade_outbound(self, mut socket: NegotiatedSubstream, request: RegistrarRequest) -> Self::Future {
        Box::pin(async move {
            write_one(&mut socket, request.id.into_bytes()).await?;
            let data = read_one(&mut socket, 1024).await?;
            let success = data == b"success";
            Ok(RegistrarResponse { success })
        })
    }
}

// Define the behaviour for the protocol
#[derive(NetworkBehaviour)]
struct RegistrarBehaviour {
    #[behaviour(ignore)]
    events: Vec<NetworkBehaviourAction<(), RegistrarEvent>>,
}

impl RegistrarBehaviour {
    fn new() -> Self {
        Self { events: Vec::new() }
    }
}

impl NetworkBehaviour for RegistrarBehaviour {
    type ProtocolsHandler = RegistrarHandler;
    type OutEvent = RegistrarEvent;

    fn new_handler(&mut self) -> Self::ProtocolsHandler {
        RegistrarHandler::new()
    }

    fn inject_event(&mut self, _: PeerId, _: <Self::ProtocolsHandler as ProtocolsHandler>::OutEvent) {}

    fn poll(&mut self, _: &mut Context, _: &mut impl PollParameters) -> Poll<NetworkBehaviourAction<Self::OutEvent, Self::ProtocolsHandler>> {
        if let Some(event) = self.events.pop() {
            Poll::Ready(event)
        } else {
            Poll::Pending
        }
    }
}

// Define the handler for the protocol
struct RegistrarHandler;

impl RegistrarHandler {
    fn new() -> Self {
        Self
    }
}

impl ProtocolsHandler for RegistrarHandler {
    type InEvent = ();
    type OutEvent = RegistrarEvent;
    type Error = io::Error;
    type InboundProtocol = RegistrarProtocol;
    type OutboundProtocol = RegistrarProtocol;
    type OutboundOpenInfo = RegistrarRequest;
    type InboundOpenInfo = ();

    fn listen_protocol(&self) -> SubstreamProtocol<Self::InboundProtocol, Self::InboundOpenInfo> {
        SubstreamProtocol::new(RegistrarProtocol, ())
    }

    fn inject_fully_negotiated_inbound(&mut self, request: <Self::InboundProtocol as InboundUpgrade<NegotiatedSubstream>>::Output, _: Self::InboundOpenInfo) {
        // Handle inbound request
    }

    fn inject_fully_negotiated_outbound(&mut self, response: <Self::OutboundProtocol as OutboundUpgrade<NegotiatedSubstream>>::Output, _: Self::OutboundOpenInfo) {
        // Handle outbound response
    }

    fn inject_event(&mut self, _: Self::InEvent) {}

    fn inject_dial_upgrade_error(&mut self, _: Self::OutboundOpenInfo, _: io::Error) {}

    fn connection_keep_alive(&self) -> libp2p::swarm::KeepAlive {
        libp2p::swarm::KeepAlive::Yes
    }

    fn poll(&mut self, _: &mut Context) -> Poll<ProtocolsHandlerEvent<Self::OutboundProtocol, Self::OutboundOpenInfo, Self::OutEvent, Self::Error>> {
        Poll::Pending
    }
}

// Define the events for the protocol
#[derive(Debug)]
enum RegistrarEvent {
    RequestReceived(RegistrarRequest),
    ResponseReceived(RegistrarResponse),
}