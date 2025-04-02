use ololiuhqui_network::networking::key::LiuhqiKeypair;
use ololiuhqui_network::networking::*;
use ololiuhqui_network::wallet::*;
use libp2p::Swarm;
use libp2p::PeerId;
use libp2p::Multiaddr;
use libp2p::swarm::SwarmEvent;
use libp2p::futures::StreamExt;

#[tokio::main]
async fn main() {

    // Keypair Generation
    let keypair = generate_keypair();
    let kp = keypair.keypair();

    let peer_id = kp.public().to_peer_id();

    println!("Peer Id: {}", peer_id.clone());

    let mut swarm: libp2p::Swarm<behaviour::OloliuhquiBehaviour> = ololiuhqui_network::networking::swarm::LiuhqSwarm::new(kp.clone());
    println!("Swarm created");

    let addr: Multiaddr = "/ip4/0.0.0.0/tcp/0".parse().unwrap();
    swarm.listen_on(addr).unwrap();

    loop {
        match swarm.select_next_some().await {
            SwarmEvent::NewListenAddr { address, .. } => {
                println!("Listening on {address:?}");
            }
            _ => {

            }
        }
    }
}

fn generate_keypair() -> LiuhqiKeypair {
    let keypair = ololiuhqui_network::networking::key::LiuhqiKeypair::generate_ecdsa();
    return keypair
}