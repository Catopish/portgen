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

// Contains only the essential data needed for port calculation
struct _PortConfig {
    base: u16,     // Network base (31000-38000)
    offset: u16,   // Chain offset (000-600)
    role: u16,     // Role offset (10-30)
    instance: u16, // Instance number (1-9)
}

// Custom chains and their offsets
const CUSTOM_CHAINS: [(&str, u16); 8] = [
    ("moonbeam", 0),
    ("hyperbridge", 100),
    ("interlay", 200),
    ("acala", 300),
    ("kilt", 400),
    ("karura", 300),
    ("kintsugi", 200),
    ("gargantua", 100),
];

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

fn get_role(s: &str) -> Option<Role> {
    match s {
        "boot" => Some(Role::Boot),
        "val" => Some(Role::Val),
        "rpc" => Some(Role::Rpc),
        _ => None,
    }
}

fn get_network(s: &str) -> Option<Network> {
    match s {
        "polkadot" => Some(Network::Polkadot),
        "kusama" => Some(Network::Kusama),
        "westend" => Some(Network::Westend),
        "paseo" => Some(Network::Paseo),
        _ => None,
    }
}

fn calculate_port(name: &str) -> Option<u16> {
    // Split by '-' and remove .yaml extension
    let parts: Vec<&str> = name.trim_end_matches(".yaml").split('-').collect();
    
    // We need at least role-something-number
    if parts.len() < 3 {
        return None;
    }

    // First part must be a valid role
    let role = get_role(parts[0])? as u16;
    
    // Last part must be a valid instance number
    let instance: u16 = parts.last()?.parse().ok()?;
    if instance == 0 || instance > 9 {
        return None;
    }

    // Everything between first and last is our chain/network identification
    let middle: Vec<&str> = parts[1..parts.len()-1].to_vec();
    
    // Handle relay chains (only network name)
    if middle.len() == 1 {
        let network = get_network(middle[0])? as u16;
        return Some(network + role + instance);
    }

    // Extract the last part as network
    let network_name = middle.last()?;
    let network = get_network(network_name)?;
    
    // Join the remaining parts as potential chain name
    let chain_name = middle[..middle.len()-1].join("-");
    
    // Check if it's a custom chain
    let custom_offset = CUSTOM_CHAINS.iter()
        .find(|(name, _)| *name == chain_name)
        .map(|(_, offset)| (*offset, true));
        
    // Check if it's a system chain
    let system_offset = SYSTEM_CHAINS.iter()
        .find(|(name, _)| *name == chain_name)
        .map(|(_, offset)| (*offset, false));
        
    // Get final offset and determine if we need to use custom network base
    let (offset, is_custom) = custom_offset.or(system_offset)?;
    
    let base = if is_custom {
        match network {
            Network::Polkadot => Network::PolkadotCustom as u16,
            Network::Kusama => Network::KusamaCustom as u16,
            Network::Paseo => Network::PaseoCustom as u16,
            _ => network as u16,
        }
    } else {
        network as u16
    };

    Some(base + offset + role + instance)
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
    fn test_relay_chains() {
        assert_eq!(calculate_port("val-polkadot-01"), Some(31021));
        assert_eq!(calculate_port("rpc-kusama-02"), Some(32032));
    }

    #[test]
    fn test_system_chains() {
        assert_eq!(calculate_port("rpc-asset-hub-polkadot-01"), Some(31131));
        assert_eq!(calculate_port("val-bridge-hub-kusama-01"), Some(32221));
        assert_eq!(calculate_port("rpc-people-westend-02"), Some(33432));
    }

    #[test]
    fn test_custom_chains() {
        assert_eq!(calculate_port("rpc-kilt-polkadot-01"), Some(35431));
        assert_eq!(calculate_port("val-karura-kusama-01"), Some(36321));
    }

    #[test]
    fn test_invalid_inputs() {
        assert_eq!(calculate_port("invalid"), None);
        assert_eq!(calculate_port("rpc-invalid-01"), None);
        assert_eq!(calculate_port("rpc-polkadot-00"), None);
        assert_eq!(calculate_port("rpc-polkadot-10"), None);
    }
}
