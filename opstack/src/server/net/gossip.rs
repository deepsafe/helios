use std::{
    net::{IpAddr, SocketAddr},
    time::Duration,
};

use eyre::Result;
use libp2p::{
    futures::StreamExt,
    gossipsub::{self, IdentTopic, Message, MessageId},
    mplex::MplexConfig,
    multiaddr::Protocol,
    noise, ping,
    swarm::{NetworkBehaviour, SwarmBuilder, SwarmEvent},
    tcp, Multiaddr, PeerId, Swarm, Transport,
    identify
};
use libp2p_identity::Keypair;
use sha2::{Digest, Sha256};
use tokio::select;

use super::{block_handler::BlockHandler, discovery};

/// OP Stack gossip service
pub struct GossipService {
    /// The socket address that the service is listening on.
    addr: SocketAddr,
    /// The chain ID of the network
    chain_id: u64,
    /// A unique keypair to validate the node's identity
    keypair: Option<Keypair>,
    /// Handler for the block
    block_handler: BlockHandler,
    fixed_peerid: bool,
}

impl GossipService {
    /// Creates a new [Service]
    pub fn new(addr: SocketAddr, chain_id: u64, handler: BlockHandler, fixed_peerid: bool) -> Self {
        Self {
            addr,
            chain_id,
            keypair: None,
            block_handler: handler,
            fixed_peerid,
        }
    }

    /// Sets the keypair for [Service]
    pub fn set_keypair(mut self, keypair: Keypair) -> Self {
        self.keypair = Some(keypair);
        self
    }

    pub fn fixed_keypair() -> Keypair {
        let sk_bytes = vec![6, 139, 1, 93, 95, 125, 87, 37, 156, 208, 22, 120, 79, 244, 118, 152, 153, 90, 89, 49, 137, 57, 62, 10, 3, 26, 38, 70, 166, 198, 188, 240];
        //let sk_bytes = vec![6, 139, 3, 93, 95, 125, 87, 37, 156, 208, 22, 120, 79, 244, 118, 152, 153, 90, 89, 49, 137, 57, 62, 10, 3, 26, 38, 70, 166, 198, 188, 240];

        let sk = libp2p_identity::secp256k1::SecretKey::try_from_bytes(sk_bytes).unwrap();
        let keypair = libp2p_identity::secp256k1::Keypair::from(sk);
        let kp = libp2p_identity::Keypair::from(keypair);
        kp
    }

    /// Starts the Discv5 peer discovery & libp2p services
    /// and continually listens for new peers and messages to handle
    pub fn start(self) -> Result<()> {
        
        let keypair = if self.fixed_peerid {
            Self::fixed_keypair()
        } else {
            self.keypair.unwrap_or_else(Keypair::generate_secp256k1)
        };

        let mut swarm = create_swarm(keypair, &self.block_handler)?;
        let mut peer_recv = discovery::start(self.addr, self.chain_id)?;
        let multiaddr = socket_to_multiaddr(self.addr);

        tracing::info!("peer id {}", swarm.local_peer_id());
        tracing::info!("multiaddr {}", multiaddr);


        //swarm.dial("/ip4/172.20.128.155/tcp/9877/p2p/16Uiu2HAmGBpLj5ecnrGCCAiwGqVdKavNdtxtUq92q43rcumn28q9".parse::<Multiaddr>().unwrap()).unwrap();


        swarm
            .listen_on(multiaddr)
            .map_err(|_| eyre::eyre!("swarm listen failed"))?;

        tokio::spawn(async move {
            loop {
                select! {
                    peer = peer_recv.recv() => {
                        if let Some(peer) = peer {
                            tracing::info!("adding peer");
                            let peer = socket_to_multiaddr(peer);
                            _ = swarm.dial(peer);
                        }
                    },
                    event = swarm.select_next_some() => {
                        if let SwarmEvent::Behaviour(event) = event {
                            event.handle(&mut swarm, &self.block_handler);
                        }
                    },
                }
            }
        });

        Ok(())
    }
}

fn socket_to_multiaddr(socket: SocketAddr) -> Multiaddr {
    let mut multiaddr = Multiaddr::empty();
    match socket.ip() {
        IpAddr::V4(ip) => multiaddr.push(Protocol::Ip4(ip)),
        IpAddr::V6(ip) => multiaddr.push(Protocol::Ip6(ip)),
    }
    multiaddr.push(Protocol::Tcp(socket.port()));
    multiaddr
}

/// Computes the message ID of a `gossipsub` message
fn compute_message_id(msg: &Message) -> MessageId {
    let mut decoder = snap::raw::Decoder::new();
    let id = match decoder.decompress_vec(&msg.data) {
        Ok(data) => {
            let domain_valid_snappy: Vec<u8> = vec![0x1, 0x0, 0x0, 0x0];
            let mut hasher = Sha256::new();
            hasher.update(
                [domain_valid_snappy.as_slice(), data.as_slice()]
                    .concat()
                    .as_slice(),
            );
            hasher.finalize()[..20].to_vec()
        }
        Err(_) => {
            let domain_invalid_snappy: Vec<u8> = vec![0x0, 0x0, 0x0, 0x0];
            let mut hasher = Sha256::new();
            hasher.update(
                [domain_invalid_snappy.as_slice(), msg.data.as_slice()]
                    .concat()
                    .as_slice(),
            );
            hasher.finalize()[..20].to_vec()
        }
    };

    MessageId(id)
}

/// Creates the libp2p [Swarm]
fn create_swarm(keypair: Keypair, handler: &BlockHandler) -> Result<Swarm<Behaviour>> {
    let transport = tcp::tokio::Transport::new(tcp::Config::default())
        .upgrade(libp2p::core::upgrade::Version::V1Lazy)
        .authenticate(noise::Config::new(&keypair)?)
        .multiplex(MplexConfig::default())
        .boxed();

    let behaviour = Behaviour::new(handler, keypair.clone())?;

    Ok(
        SwarmBuilder::with_tokio_executor(transport, behaviour, PeerId::from(keypair.public()))
            .build(),
    )
}

/// Specifies the [NetworkBehaviour] of the node
#[derive(NetworkBehaviour)]
#[behaviour(out_event = "Event")]
struct Behaviour {
    /// Adds [libp2p::ping] to respond to inbound pings, and send periodic outbound pings
    ping: ping::Behaviour,
    /// Adds [libp2p::gossipsub] to enable gossipsub as the routing layer
    gossipsub: gossipsub::Behaviour,
    /// Adds [libp2p::identify] to enable identify
    id: identify::Behaviour,
}

impl Behaviour {
    /// Configures the swarm behaviors, subscribes to the gossip topics, and returns a new [Behaviour]
    fn new(handler: &BlockHandler, keypair: Keypair) -> Result<Self> {
        let ping = ping::Behaviour::default();

        let gossipsub_config = gossipsub::ConfigBuilder::default()
            .mesh_n(8)
            .mesh_n_low(6)
            .mesh_n_high(30)
            .gossip_lazy(6)
            .heartbeat_interval(Duration::from_millis(500))
            .fanout_ttl(Duration::from_secs(24))
            .history_length(12)
            .history_gossip(3)
            .duplicate_cache_time(Duration::from_secs(65))
            .validation_mode(gossipsub::ValidationMode::None)
            .validate_messages()
            .message_id_fn(compute_message_id)
            .build()
            .map_err(|_| eyre::eyre!("gossipsub config creation failed"))?;

        let mut gossipsub =
            gossipsub::Behaviour::new(gossipsub::MessageAuthenticity::Anonymous, gossipsub_config)
                .map_err(|_| eyre::eyre!("gossipsub behaviour creation failed"))?;

        handler
            .topics()
            .iter()
            .map(|topic| {
                let topic = IdentTopic::new(topic.to_string());
                gossipsub
                    .subscribe(&topic)
                    .map_err(|_| eyre::eyre!("subscription failed"))
            })
            .collect::<Result<Vec<_>>>()?;

        let id = identify::Behaviour::new(
            identify::Config::new("".to_string(), keypair.public()));

        Ok(Self { ping, gossipsub, id })
    }
}

/// The type of message received
enum Event {
    /// Represents a [ping::Event]
    #[allow(dead_code)]
    Ping(ping::Event),
    /// Represents a [gossipsub::Event]
    Gossipsub(gossipsub::Event),
    /// Represents a [identify::Event]
    ID(identify::Event),
}

impl Event {
    /// Handles received gossipsub messages. Ping messages are ignored.
    /// Reports back to [libp2p::gossipsub] to apply peer scoring and forward the message to other peers if accepted.
    fn handle(self, swarm: &mut Swarm<Behaviour>, handler: &BlockHandler) {
        match  self {
            Event::Ping(event) => tracing::debug!("[Ping] event {event:?} "),
            Event::Gossipsub(event) => {
                match event {
                    gossipsub::Event::Message { propagation_source, message_id, message } => {
                        let status = handler.handle(message);
                        _ = swarm
                        .behaviour_mut()
                        .gossipsub
                        .report_message_validation_result(&message_id, &propagation_source, status);
                    },
                    gossipsub::Event::Subscribed { peer_id, topic } => tracing::info!("[Subscribed] peer_id {peer_id:?} topic {topic:?}"),
                    gossipsub::Event::Unsubscribed { peer_id, topic } => tracing::info!("[Unsubscribed] peer_id {peer_id:?} topic {topic:?}"),
                    gossipsub::Event::GossipsubNotSupported { peer_id } => tracing::info!("[GossipsubNotSupported] peer_id {peer_id:?}"),
                }
            },
            Event::ID(event) => {
                tracing::info!("[ID] event {:?}", event);
            },
        }
    }
}

impl From<ping::Event> for Event {
    /// Converts [ping::Event] to [Event]
    fn from(value: ping::Event) -> Self {
        Event::Ping(value)
    }
}

impl From<gossipsub::Event> for Event {
    /// Converts [gossipsub::Event] to [Event]
    fn from(value: gossipsub::Event) -> Self {
        Event::Gossipsub(value)
    }
}

impl From<identify::Event> for Event {
    /// Converts [identify::Event] to [Event]
    fn from(value: identify::Event) -> Self {
        Event::ID(value)
    }
}
