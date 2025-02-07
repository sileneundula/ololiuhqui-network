use libp2p::identify;
use libp2p::ping;
use libp2p::kad;
use libp2p::kad::store::MemoryStore;
use libp2p::swarm::NetworkBehaviour;
use libp2p::rendezvous;
use libp2p::gossipsub;
use libp2p::floodsub;
use libp2p::relay;

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