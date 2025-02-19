use libp2p::identify;
use libp2p::identity::Keypair;
use libp2p::ping;
use libp2p::kad;
use libp2p::kad::store::MemoryStore;
use libp2p::swarm::NetworkBehaviour;
use libp2p::rendezvous;
use libp2p::gossipsub;
use libp2p::floodsub;
use libp2p::relay;
use libp2p::PeerId;

#[derive(NetworkBehaviour)]
pub struct OloliuhquiBehaviour {
    // Relay
    relay: relay::Behaviour,
    // Identify Protocol
    identify: identify::Behaviour,
    // Ping Protocol
    ping: ping::Behaviour,
    // KAD
    kad: kad::Behaviour<MemoryStore>,
    // Rendezous
    rendezous_server: rendezvous::server::Behaviour,
    rendezous_client: rendezvous::client::Behaviour,
    // GossipSub
    gossip: gossipsub::Behaviour,
    // Floodsub
    floodsub: floodsub::Floodsub,
}

impl OloliuhquiBehaviour {
    pub fn new(key: Keypair) -> Self {
        let pk = key.public();
        let peer_id = key.public().to_peer_id();

        Self {
            // Standard Behaviours
            relay: relay::Behaviour::new(key.public().to_peer_id(), relay::Config::default()),
            
            identify: identify::Behaviour::new(identify::Config::new(String::from("/ololiuhqui-identify/0.1.0/"),pk.clone())),

            ping: ping::Behaviour::new(ping::Config::default()),
            kad: kad::Behaviour::new(peer_id.clone(),kad::store::MemoryStore::new(peer_id.clone())),
            rendezous_server: rendezvous::server::Behaviour::new(rendezvous::server::Config::default()),
            rendezous_client: rendezvous::client::Behaviour::new(key.clone()),
            gossip: gossipsub::Behaviour::new(gossipsub::MessageAuthenticity::Signed(key.clone()), gossipsub::Config::default()).unwrap(),
            floodsub: floodsub::Floodsub::new(peer_id.clone()),
        }
    }
}