# mimir-clients

mimir-bridge clients implemented in rust.

## dev status

### mimir-node

- [x] custom `web3` utility namespace w/ local node management helpers
- [x] `rpc` types w/ sanitization & block state injection (placeholders)
- [x] external transaction signing & compilation helpers
- [x] transparent jsonrpc error forwarding
- [ ] fail-fast on chain reorgs (may require parity fork)

### mimir-worker
 
- [x] check for funded state on startup
- [x] check for bound state on startup

### mimir-requester

TODO

### mimir-transport

TODO

