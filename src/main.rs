use clap::Parser;
use std::error::Error;

#[derive(Debug, Clone, Copy)]
enum Network {
    Polkadot,      // 31xxx
    PolkadotCustom, // 35xxx
    Kusama,        // 32xxx
    KusamaCustom,  // 36xxx
    Westend,       // 33xxx
    Paseo,         // 34xxx
    PaseoCustom,   // 38xxx
}

#[derive(Debug, Clone, Copy)]
enum Chain {
    Relay,         // 3x0xx
    AssetHub,      // 3x1xx
    BridgeHub,     // 3x2xx
    Collectives,   // 3x3xx
    People,        // 3x4xx
    Coretime,      // 3x5xx
    Moonbeam,      // 350xx
    Hyperbridge,   // 351xx
    Interlay,      // 352xx
    Acala,         // 353xx
    Kilt,          // 354xx
    Karura,        // 363xx  (Kusama custom chain)
    Kintsugi,      // 362xx  (Kusama custom chain)
    Gargantua,     // 381xx  (Paseo custom chain)
    Encointer,     // 3x6xx
}

#[derive(Debug, Clone, Copy)]
enum Role {
    Boot,          // 3xx1x
    Val,           // 3xx2x
    Rpc,           // 3xx3x
}

#[derive(Debug)]
struct NodeConfig {
    network: Network,
    chain: Option<Chain>,
    role: Role,
    instance: u8,
}

impl Network {
    const fn base_port(&self) -> u16 {
        match self {
            Network::Polkadot => 31000,
            Network::PolkadotCustom => 35000,
            Network::Kusama => 32000,
            Network::KusamaCustom => 36000,
            Network::Westend => 33000,
            Network::Paseo => 34000,
            Network::PaseoCustom => 38000,
        }
    }

    fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "polkadot" => Some(Self::Polkadot),
            "kusama" => Some(Self::Kusama),
            "westend" => Some(Self::Westend),
            "paseo" => Some(Self::Paseo),
            _ => None,
        }
    }

    fn is_custom_chain(&self, chain: &Chain) -> bool {
        matches!(
            (self, chain),
            (Network::Polkadot, Chain::Moonbeam) |
            (Network::Polkadot, Chain::Hyperbridge) |
            (Network::Polkadot, Chain::Interlay) |
            (Network::Polkadot, Chain::Acala) |
            (Network::Polkadot, Chain::Kilt) |
            (Network::Kusama, Chain::Karura) |
            (Network::Kusama, Chain::Kintsugi) |
            (Network::Paseo, Chain::Gargantua)
        )
    }
}

impl Chain {
    fn get_offset(&self, network: &Network) -> u16 {
        if network.is_custom_chain(self) {
            // Custom chain offsets
            match self {
                Chain::Moonbeam => 0,
                Chain::Hyperbridge => 100,
                Chain::Interlay => 200,
                Chain::Acala => 300,
                Chain::Kilt => 400,
                Chain::Karura => 300,  // Kusama custom
                Chain::Kintsugi => 200, // Kusama custom
                Chain::Gargantua => 100, // Paseo custom
                _ => 0,
            }
        } else {
            // System chain offsets
            match self {
                Chain::Relay => 0,
                Chain::AssetHub => 100,
                Chain::BridgeHub => 200,
                Chain::Collectives => 300,
                Chain::People => 400,
                Chain::Coretime => 500,
                Chain::Encointer => 600,
                _ => 0,
            }
        }
    }

    fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().replace('-', "").as_str() {
            "relay" => Some(Self::Relay),
            "assethub" => Some(Self::AssetHub),
            "bridgehub" => Some(Self::BridgeHub),
            "collectives" => Some(Self::Collectives),
            "people" => Some(Self::People),
            "coretime" => Some(Self::Coretime),
            "moonbeam" => Some(Self::Moonbeam),
            "hyperbridge" => Some(Self::Hyperbridge),
            "interlay" => Some(Self::Interlay),
            "acala" => Some(Self::Acala),
            "kilt" => Some(Self::Kilt),
            "karura" => Some(Self::Karura),
            "kintsugi" => Some(Self::Kintsugi),
            "gargantua" => Some(Self::Gargantua),
            "encointer" => Some(Self::Encointer),
            _ => None,
        }
    }
}

impl Role {
    const fn offset(&self) -> u16 {
        match self {
            Role::Boot => 10,
            Role::Val => 20,
            Role::Rpc => 30,
        }
    }

    fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "boot" => Some(Self::Boot),
            "val" => Some(Self::Val),
            "rpc" => Some(Self::Rpc),
            _ => None,
        }
    }
}

impl NodeConfig {
    fn generate_port(&self) -> u16 {
        let base = match (&self.network, &self.chain) {
            (network, Some(chain)) if network.is_custom_chain(chain) => {
                match self.network {
                    Network::Polkadot => Network::PolkadotCustom,
                    Network::Kusama => Network::KusamaCustom,
                    Network::Paseo => Network::PaseoCustom,
                    _ => self.network,
                }.base_port()
            },
            _ => self.network.base_port()
        };

        base + 
            self.chain.map_or(0, |c| c.get_offset(&self.network)) + 
            self.role.offset() + 
            u16::from(self.instance)  // Convert u8 to u16
    }

    fn from_name(name: &str) -> Option<Self> {
        let parts: Vec<&str> = name.split(&['-', '.'][..]).collect();
        if parts.len() < 2 {
            return None;
        }

        let role = Role::from_str(parts[0])?;

        // Handle relay chain cases like "rpc-polkadot-01"
        if parts.len() == 3 {
            let network = Network::from_str(parts[1])?;
            let instance: u8 = parts[2].replace(".yaml", "").parse().ok()?;
            return Some(Self {
                network,
                chain: None, // Relay chain
                role,
                instance,
            });
        }

        // Handle parachain cases like "rpc-asset-hub-polkadot-01"
        if parts.len() >= 4 {
            let chain_name = format!("{}-{}", parts[1], parts[2]);
            let chain = Chain::from_str(&chain_name)?;
            let network = Network::from_str(parts[3])?;
            let instance: u8 = parts[4].replace(".yaml", "").parse().ok()?;
            return Some(Self {
                network,
                chain: Some(chain),
                role,
                instance,
            });
        }

        None
    }
}

#[derive(Parser)]
#[command(about = "Generate ports for blockchain network nodes")]
struct Cli {
    /// Node name (e.g., rpc-polkadot-01 or rpc-asset-hub-polkadot-01)
    name: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();

    match NodeConfig::from_name(&args.name) {
        Some(config) => println!("{}", config.generate_port()),
        None => eprintln!("Invalid node name format"),
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_relay_chain_nodes() {
        let config = NodeConfig::from_name("rpc-polkadot-01.yaml").unwrap();
        assert_eq!(config.generate_port(), 31031);

        let config = NodeConfig::from_name("val-kusama-01.yaml").unwrap();
        assert_eq!(config.generate_port(), 32021);
    }

    #[test]
    fn test_system_parachain_nodes() {
        let config = NodeConfig::from_name("rpc-asset-hub-polkadot-01.yaml").unwrap();
        assert_eq!(config.generate_port(), 31131);

        let config = NodeConfig::from_name("rpc-bridge-hub-kusama-01.yaml").unwrap();
        assert_eq!(config.generate_port(), 32231);
    }

    #[test]
    fn test_custom_chain_nodes() {
        let config = NodeConfig::from_name("rpc-kilt-polkadot-01.yaml").unwrap();
        assert_eq!(config.generate_port(), 35431);

        let config = NodeConfig::from_name("rpc-karura-kusama-01.yaml").unwrap();
        assert_eq!(config.generate_port(), 36331);
    }
}
