# Tx-Client

A typescript client to interact with ticket-tx smart rollup.

## L1 -> L2 Bridge

- `<network>`: `ghostnet` or `mainnet`

### For FA12 token
```shell
PRIVATE_KEY=<...> npm run bridge <network> <bridge-address> fa12 <token-address> <amount>
```

### For FA2 token
```shell
PRIVATE_KEY=<...> npm run bridge <network> <bridge-address> fa2 <token-address> <token-id> <amount>
```

## L2 Transfers

- `<network>`: `ghostnet` or `mainnet`
- `<across>`: `sequencer` or `inbox`

### For FA12 token
```shell
PRIVATE_KEY=<...> npm run transfer <network> <across> <nonce> fa12 <token-address> <amount> <destination>
```

### For FA2 token
```shell
PRIVATE_KEY=<...> npm run transfer <network> <across> <nonce> fa2 <token-address> <token-id> <amount> <destination>
```