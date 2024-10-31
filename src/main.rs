use clap::Parser;
use std::{fmt, net::Ipv4Addr, str::FromStr};

const PORT_BASE: u16 = 30000;

#[derive(Parser)]
#[command(name = "portgen", about = "Generate port numbers and IP addresses for substrate nodes")]
#[command(after_help = "\
Examples:
  # Relay chain nodes
  portgen boot-polkadot-00           # Bootnode (31000, 192.168.11.10)
  portgen rpc-kusama-01              # RPC node (32001, 192.168.121.10)
  portgen val-westend-04             # Validator (33004, 192.168.234.10)

  # System parachain nodes
  portgen rpc-asset-hub-polkadot-01  # Asset Hub RPC (31011, 192.168.111.11)
  portgen boot-bridge-hub-kusama-00  # Bridge Hub boot (32020, 192.168.20.12)
  portgen val-people-westend-04      # People chain validator (33044, 192.168.234.14)

Supported roles:
  - boot: bootnode (instance 00)
  - rpc:  RPC node (instances 01-03)
  - val:  validator node (instances 04-09)

Format: {role}-{chain}-{network}-{instance}
Port:   3NCCI (N=network, CC=chain, I=instance)
IP:     192.168.{RNI}.{chain_id+10}
        R: role (0=boot, 1=rpc, 2=validator)
        N: network (1=polkadot, 2=kusama, 3=westend, 4=paseo)
        I: instance number")]
struct Args {
    /// Node name (e.g., rpc-asset-hub-polkadot-01)
    node_name: String,
}

#[derive(Debug, Clone, Copy)]
struct Port(u16);

impl fmt::Display for Port {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Copy)]
struct NodeAddress {
    port: Port,
    ip: Ipv4Addr,
}

impl fmt::Display for NodeAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.ip, self.port)
    }
}

#[derive(Debug, Clone, Copy)]
enum Network {
    Polkadot = 1,
    Kusama = 2,
    Westend = 3,
    Paseo = 4,
}

#[derive(Debug, Clone, Copy)]
struct ChainId(u16);

#[derive(Debug, Clone, Copy)]
enum Role {
    Boot,
    Rpc(u8),
    Validator(u8),
}

impl Role {
    fn from_str(role: &str, instance_str: &str) -> Result<Self, &'static str> {
        if instance_str.len() != 2 {
            return Err("instance must be two digits (00-09)");
        }

        let num: u8 = instance_str
            .parse()
            .map_err(|_| "invalid instance number")?;

        match (role, num) {
            ("boot", 0..=9) => Ok(Self::Boot),
            ("rpc", 1..=3) => Ok(Self::Rpc(num)),
            ("val", 1..=6) => Ok(Self::Validator(num)),
            _ => Err("invalid role/instance combination"),
        }
    }

    fn to_digit(self) -> u16 {
        match self {
            Self::Boot => 0,
            Self::Rpc(n) => n as u16,
            Self::Validator(n) => (n + 3) as u16,
        }
    }

    fn to_ip_digit(self) -> u8 {
        match self {
            Self::Boot => 0,
            Self::Rpc(_) => 1,
            Self::Validator(_) => 2,
        }
    }

    fn get_instance_number(self) -> u8 {
        match self {
            Self::Boot => 0,
            Self::Rpc(n) => n,
            Self::Validator(n) => n,
        }
    }
}

impl FromStr for Network {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "polkadot" => Ok(Self::Polkadot),
            "kusama" => Ok(Self::Kusama),
            "westend" => Ok(Self::Westend),
            "paseo" => Ok(Self::Paseo),
            _ => Err("invalid network name"),
        }
    }
}

impl ChainId {
    fn from_str(chain: Option<&str>) -> Result<Self, &'static str> {
        let id = match chain {
            None => 0,
            Some(name) => match name {
                // system
                "asset-hub" | "statemine" | "statemint" => 1,
                "bridge-hub" | "bridgehub" => 2,
                "collectives" => 3,
                "people" => 4,
                "coretime" => 5,
                "encointer" => 6,
                // custom
                "moonbeam" | "moonriver" => 20,
                "hyperbridge" | "nexus" => 21,
                "interlay" | "kintsugi" => 22,
                "acala" | "karura" => 23,
                "kilt" | "spiritnet" => 24,
                "hyperbridge" | "gargantua" => 25,
                "hydration" | "hydradx" => 26,
                "bifrost-polkadot" | "bifrost-kusama" => 27,
                "bajun" | "ajuna" => 28,
                "polimec" => 29,
                "unique" | "quartz" => 30,
                _ => return Err("unknown chain name"),
            },
        };
        Ok(ChainId(id))
    }

    fn to_ip_host(&self) -> u8 {
        self.0 as u8 + 10 // Start from .10 for relay chain
    }
}

#[derive(Debug)]
struct NodeName<'a> {
    role: &'a str,
    chain: Option<String>,
    network: &'a str,
    instance: &'a str,
}

impl<'a> NodeName<'a> {
    fn parse(s: &'a str) -> Result<Self, &'static str> {
        let parts: Vec<&str> = s.trim_end_matches(".yaml").split('-').collect();
        if parts.len() < 3 {
            return Err("invalid node name format");
        }

        let role = parts.first().ok_or("missing role")?;
        let instance = parts.last().ok_or("missing instance")?;
        let network = parts[parts.len() - 2];

        let chain = if parts.len() > 3 {
            Some(parts[1..parts.len() - 2].join("-"))
        } else {
            None
        };

        Ok(Self {
            role,
            chain,
            network,
            instance,
        })
    }
}

fn calculate_port(node_str: &str) -> Result<Port, &'static str> {
    let node = NodeName::parse(node_str)?;
    
    let network = node.network.parse::<Network>()?;
    let chain_id = ChainId::from_str(node.chain.as_deref())?;
    let role = Role::from_str(node.role, node.instance)?;

    let port = PORT_BASE +
        (network as u16 * 1000) +
        (chain_id.0 * 10) +
        role.to_digit();

    Ok(Port(port))
}

fn calculate_address(node_str: &str) -> Result<NodeAddress, &'static str> {
    let node = NodeName::parse(node_str)?;
    
    let network = node.network.parse::<Network>()?;
    let chain_id = ChainId::from_str(node.chain.as_deref())?;
    let role = Role::from_str(node.role, node.instance)?;

    let port = calculate_port(node_str)?;

    // Calculate third octet: {role}{network}{instance}
    let third_octet = 
        role.to_ip_digit() * 100 +    // First digit (0/1/2) * 100
        (network as u8) * 10 +        // Second digit (1-4) * 10
        role.get_instance_number();    // Third digit (instance number)

    let fourth_octet = chain_id.to_ip_host();
    
    // 192.168.xyz.abc
    let ip = Ipv4Addr::new(192, 168, third_octet, fourth_octet);

    Ok(NodeAddress { port, ip })
}

fn main() {
    let args = Args::parse();
    match calculate_address(&args.node_name) {
        Ok(addr) => println!("{addr}"),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}
