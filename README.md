# portgen

Port number generator for substrate node naming conventions.

## install

```sh
curl -L https://github.com/rotkonetworks/portgen/releases/download/v0.3.0/portgen -o portgen && chmod +x portgen && sudo mv portgen /usr/local/bin/
```

## usage example

```sh
$ portgen rpc-polkadot-01        # p2p port for polkadot relay chain rpc node 1
31031

$ portgen rpc-asset-hub-kusama-01  # p2p port for asset parachain rpc node 1 on kusama network
32131 

$ portgen val-kilt-polkadot-01     # p2p port for kilt (custom) parachain validator node 1 on polkadot network
35421
```

## port scheme

```sh
{role}-{chain?}-{network}-{instance}

# network base (first two digits)
31xxx - polkadot relay/system paras
32xxx - kusama relay/system paras
33xxx - westend relay/system paras  
34xxx - paseo relay/system paras
35xxx - polkadot custom paras
36xxx - kusama custom paras
38xxx - paseo custom paras

# chain offset (3rd and 4th digits) 
xx0xx - relay chain
xx1xx - asset hub
xx2xx - bridge hub
xx3xx - collectives
xx4xx - people
xx5xx - coretime
xx6xx - encointer
350xx - moonbeam (polkadot)
351xx - hyperbridge (polkadot) 
352xx - interlay (polkadot)
353xx - acala (polkadot)
354xx - kilt (polkadot)
362xx - kintsugi (kusama)
363xx - karura (kusama) 
381xx - gargantua (paseo)

# role and instance (last digit)
xxx1x - bootnode
xxx2x - validator/collator  
xxx3x - rpc node
xxxxN - instance number (1-9)
```
