use std::error::Error;

#[derive(Debug, Clone, Copy)]
enum Network {
    Polkadot = 31000,
    Kusama = 32000,
    Westend = 33000,
    Paseo = 34000,
    PolkadotCustom = 35000,
    KusamaCustom = 36000,
    PaseoCustom = 38000,
}

#[derive(Debug, Clone, Copy)]
enum Role {
    Boot = 10,
    Val = 20,
    Rpc = 30,
}

// System chains and their offsets
const SYSTEM_CHAINS: [(&str, u16); 7] = [
    ("relay", 0),
    ("asset-hub", 100),
    ("bridge-hub", 200),
    ("collectives", 300),
    ("people", 400),
    ("coretime", 500),
    ("encointer", 600),
];

// Custom chains mapped by network
const POLKADOT_CHAINS: [(&str, u16); 9] = [
    ("moonbeam", 0),
    ("hyperbridge", 100),
    ("interlay", 200),
    ("acala", 300),
    ("kilt", 400),
    ("ajuna", 500),
    ("bifrost", 500),
    ("hydration", 500),
    ("unique", 500),
];

const KUSAMA_CHAINS: [(&str, u16); 2] = [
    ("karura", 300),
    ("kintsugi", 200),
];

const PASEO_CHAINS: [(&str, u16); 1] = [
    ("gargantua", 100),
];

fn get_role(s: &str) -> Option<Role> {
    match s {
        "boot" => Some(Role::Boot),
        "val" => Some(Role::Val),
        "rpc" => Some(Role::Rpc),
        _ => None,
    }
}

fn get_network(s: &str) -> Option<Network> {
    match s.to_lowercase().as_str() {
        "polkadot" => Some(Network::Polkadot),
        "kusama" => Some(Network::Kusama),
        "westend" => Some(Network::Westend),
        "paseo" => Some(Network::Paseo),
        _ => None,
    }
}

fn normalize_chain_name(name: &str) -> String {
    name.to_lowercase()
        .replace("nexus", "hyperbridge")
        .replace("hydradx", "hydration")
        .replace("spiritnet", "kilt")
}

fn is_custom_chain(chain: &str, network: Network) -> Option<u16> {
    let chain = normalize_chain_name(chain);
    match network {
        Network::Polkadot => POLKADOT_CHAINS.iter()
            .find(|(name, _)| *name == chain)
            .map(|(_, offset)| *offset),
        Network::Kusama => KUSAMA_CHAINS.iter()
            .find(|(name, _)| *name == chain)
            .map(|(_, offset)| *offset),
        Network::Paseo => PASEO_CHAINS.iter()
            .find(|(name, _)| *name == chain)
            .map(|(_, offset)| *offset),
        _ => None,
    }
}

fn get_system_offset(chain: &str) -> Option<u16> {
    SYSTEM_CHAINS.iter()
        .find(|(name, _)| *name == chain.to_lowercase())
        .map(|(_, offset)| *offset)
}

fn calculate_port(name: &str) -> Option<u16> {
    let parts: Vec<&str> = name.trim_end_matches(".yaml").split('-').collect();
    if parts.len() < 3 {
        return None;
    }

    let role = get_role(parts[0])? as u16;
    let instance: u16 = parts.last()?.parse().ok()?;
    if instance == 0 || instance > 9 {
        return None;
    }

    let middle: Vec<&str> = parts[1..parts.len()-1].to_vec();
    
    // Handle relay chains
    if middle.len() == 1 {
        let network = get_network(middle[0])? as u16;
        return Some(network + role + instance);
    }

    let network_name = middle.last()?;
    let network = get_network(network_name)?;
    let chain_name = middle[..middle.len()-1].join("-");

    // First check if it's a custom chain
    if let Some(offset) = is_custom_chain(&chain_name, network) {
        let base = match network {
            Network::Polkadot => Network::PolkadotCustom as u16,
            Network::Kusama => Network::KusamaCustom as u16,
            Network::Paseo => Network::PaseoCustom as u16,
            _ => network as u16,
        };
        return Some(base + offset + role + instance);
    }

    // Then check system chains
    if let Some(offset) = get_system_offset(&chain_name) {
        return Some(network as u16 + offset + role + instance);
    }

    None
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <node-name>", args[0]);
        std::process::exit(1);
    }

    match calculate_port(&args[1]) {
        Some(port) => println!("{}", port),
        None => eprintln!("Invalid node name format"),
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_chains() {
        assert_eq!(calculate_port("rpc-asset-hub-polkadot-01"), Some(31131));
        assert_eq!(calculate_port("val-bridge-hub-kusama-01"), Some(32221));
        assert_eq!(calculate_port("rpc-people-westend-02"), Some(33432));
        assert_eq!(calculate_port("rpc-encointer-kusama-01"), Some(32631));
    }

    #[test]
    fn test_custom_chains() {
        // Polkadot ecosystem
        assert_eq!(calculate_port("rpc-moonbeam-polkadot-01"), Some(35031));
        assert_eq!(calculate_port("rpc-kilt-polkadot-01"), Some(35431));
        assert_eq!(calculate_port("rpc-hyperbridge-polkadot-01"), Some(35131));
        assert_eq!(calculate_port("rpc-hydration-polkadot-01"), Some(35531));
        assert_eq!(calculate_port("rpc-unique-polkadot-01"), Some(35531));
        assert_eq!(calculate_port("rpc-ajuna-polkadot-01"), Some(35531));
        
        // Kusama ecosystem
        assert_eq!(calculate_port("rpc-karura-kusama-01"), Some(36331));
        assert_eq!(calculate_port("rpc-kintsugi-kusama-01"), Some(36231));
        
        // Paseo ecosystem
        assert_eq!(calculate_port("rpc-gargantua-paseo-01"), Some(38131));
    }

    #[test]
    fn test_aliases() {
        assert_eq!(calculate_port("rpc-nexus-polkadot-01"), Some(35131)); // hyperbridge
        assert_eq!(calculate_port("rpc-hydradx-polkadot-01"), Some(35531)); // hydration
    }

    #[test]
    fn test_invalid_inputs() {
        assert_eq!(calculate_port("invalid"), None);
        assert_eq!(calculate_port("rpc-invalid-01"), None);
        assert_eq!(calculate_port("rpc-polkadot-00"), None);
        assert_eq!(calculate_port("rpc-polkadot-10"), None);
    }
}
