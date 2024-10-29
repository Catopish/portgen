use clap::Parser;
use std::{fmt, str::FromStr};

const PORT_BASE: u16 = 30000;

#[derive(Parser)]
#[command(name = "portgen", about = "Generate port numbers for substrate nodes")]
#[command(after_help = "\
Examples:
  # Relay chain nodes
  portgen boot-polkadot-00           # Bootnode (31000)
  portgen rpc-kusama-01              # RPC node (32001)
  portgen val-westend-04             # Validator (33004)

  # System parachain nodes
  portgen rpc-asset-hub-polkadot-01  # Asset Hub RPC (31011)
  portgen boot-bridge-hub-kusama-00  # Bridge Hub boot (32020)
  portgen val-people-westend-04      # People chain validator (33044)

Supported roles:
  - boot: bootnode (instance 00)
  - rpc:  RPC node (instances 01-03)
  - val:  validator node (instances 04-09)

Format: {role}-{chain}-{network}-{instance}
Port:   3NCCI (N=network, CC=chain, I=instance)")]
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
        // Ensure instance is exactly two digits
        if instance_str.len() != 2 {
            return Err("instance must be two digits (00-09)");
        }

        // Parse instance number
        let num: u8 = instance_str
            .parse()
            .map_err(|_| "invalid instance number")?;

        match (role, num) {
            // Boot node is always port X000 regardless of input number
            ("boot", 0..=9) => Ok(Self::Boot),
            // RPC nodes must be 01-03
            ("rpc", 1..=3) => Ok(Self::Rpc(num)),
            // Validator nodes must be 01-06 (will map to 4-9)
            ("val", 1..=6) => Ok(Self::Validator(num)),
            _ => Err("invalid role/instance combination"),
        }
    }

    fn to_digit(self) -> u16 {
        match self {
            Self::Boot => 0,
            Self::Rpc(n) => n as u16,
            Self::Validator(n) => (n + 3) as u16,  // 01->4, 02->5, 03->6...
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
            None => 0, // Relay chain
            Some(name) => match name {
                // System chains (0-19)
                "asset-hub" | "statemine" | "statemint" => 1,
                "bridge-hub" => 2,
                "collectives" => 3,
                "people" => 4,
                "coretime" => 5,
                "encointer" => 6,
                // Custom chains (20+)
                "moonbeam" => 20,
                "hyperbridge" | "nexus" => 21,
                "interlay" => 22,
                "acala" => 23,
                "kilt" | "spiritnet" => 24,
                "karura" => 25,
                _ => return Err("unknown chain name"),
            },
        };
        Ok(ChainId(id))
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

        // Only include chain if we have more parts than role-network-instance
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

fn main() {
    let args = Args::parse();
    match calculate_port(&args.node_name) {
        Ok(port) => println!("{port}"),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}
