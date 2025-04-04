/// Transport (TCP or WSS)
pub mod transport;

/// Network Key
pub mod key;

/// Network Behaviour
pub mod behaviour;

pub mod swarm;

/// Custom Protocols
pub mod protocols;

/// Marketplace
pub mod marketplace;

pub mod ehdred_lattice;


/// Addressing (Ehdred) 
/// PROTO: [Ehdred]::[Request]
/// PROTO: [Ehdred]::[FEFEFEFE]
/// PROTO: [Ehdred]::[FEFEFEFE]::[Request]
/// ADDRESS: addr_3FED429A510BA83B7E472E5D960F233DFCF28EB031BFD3405CBEA5FC
/// ADDRESS: addr::ehdred_3FED429A510BA83B7E472E5D960F233DFCF28EB031BFD3405CBEA5FC
/// SUFFIX: ?
pub mod addressing;