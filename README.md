# portgen
Port number generator for substrate node naming conventions.

## install
```sh
curl -L https://github.com/rotkonetworks/portgen/releases/download/v0.6.1/portgen -o portgen && chmod +x portgen && sudo mv portgen /usr/local/bin/
```

## usage example
```sh
$ portgen boot-polkadot-00         # bootnode for polkadot relay chain
192.168.11.10:31000

$ portgen rpc-asset-hub-kusama-01  # RPC node for asset hub on kusama network
192.168.121.11:32011

$ portgen val-people-westend-01    # validator node for people chain on westend
192.168.231.14:33044
```

## port scheme
```
{role}-{chain}-{network}-{instance}

# network (first digit after 3)
31xxx - polkadot chain
32xxx - kusama chain
33xxx - westend chain
34xxx - paseo chain

# parachain id (digits 3-4)
xx00x - relay chain
xx01x - asset hub
xx02x - bridge hub
xx03x - collectives
xx04x - people
xx05x - coretime
xx06x - encointer

# reserved ranges
00-19 - system parachains
20+ - network parachains

# role and instance (last digit)
xxxxx0 - bootnode (instance 01)
xxxxx1-3 - rpc nodes (instances 01-03)
xxxxx4-9 - validator nodes (instances 01-06)

Full port format: 3NCCI
N = network (1-4)
CC = chain id (00-99)
I = instance (0-9)

Example breakdown:
32011 = Kusama (2) Asset Hub (01) RPC instance 01
33044 = Westend (3) People Chain (04) Validator instance 01
34000 = Paseo (4) Relay Chain (01) Bootnode 01
```

## ip scheme
```
IP format: 192.168.{RNI}.{C}
R = role (0=boot, 1=rpc, 2=validator)
N = network (1=polkadot, 2=kusama, 3=westend, 4=paseo)
I = instance number from the node name
C = chain id + 10 (relay=10, asset-hub=11, etc)

Examples:
192.168.011.10 = boot (0) polkadot (1) instance 1 relay chain (10)
192.168.121.11 = rpc (1) kusama (2) instance 1 asset hub (11)
192.168.234.14 = validator (2) westend (3) instance 4 people chain (14)
```
