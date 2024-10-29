# portgen
Port number generator for substrate node naming conventions.

## install
```sh
curl -L https://github.com/rotkonetworks/portgen/releases/download/v0.4.0/portgen -o portgen && chmod +x portgen && sudo mv portgen /usr/local/bin/
```

## usage example
```sh
$ portgen boot-polkadot-00         # bootnode for polkadot relay chain
31000

$ portgen rpc-asset-hub-kusama-01  # RPC node for asset hub on kusama network
32011 

$ portgen val-people-westend-01    # validator node for people chain on westend
33044
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
