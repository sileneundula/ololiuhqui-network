use ololiuhqui_network::networking::key::LiuhqiKeypair;
use ololiuhqui_network::networking::*;
use ololiuhqui_network::wallet::*;

#[tokio::main]
async fn main() {

    // Keypair Generation
    let keypair = generate_keypair();
    let kp = keypair.keypair();

    let peer_id = kp.public().to_peer_id();

    println!("Peer Id: {}", peer_id.clone());

    //let (response_sender, mut response_rcv) = mpsc::unbounded_channel();
}

fn generate_keypair() -> LiuhqiKeypair {
    let keypair = ololiuhqui_network::networking::key::LiuhqiKeypair::generate_ed25519();
}