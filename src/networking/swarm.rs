use libp2p::{identity, swarm::Config, Swarm};
use crate::networking::transport;
use libp2p::swarm;

use super::behaviour::OloliuhquiBehaviour;

pub struct LiuhqSwarm;

impl LiuhqSwarm {
    pub fn new(kp: identity::Keypair) -> Swarm<OloliuhquiBehaviour> {
        let transport = transport::create_secure_transport_tcp(kp.clone());
        let transport_wss = transport::create_secure_transport_wss(kp.clone());
        // Initialize the swarm with the transport and other necessary components
        let swarm: Swarm<OloliuhquiBehaviour> = Swarm::new(
            transport,
            OloliuhquiBehaviour::new(kp.clone()),
            kp.public().to_peer_id(),
            swarm::Config::with_tokio_executor(),
        );
        // Start the swarm
        return swarm
    }
}