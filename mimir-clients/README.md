# mimir-clients

mimir-bridge clients implemented in rust.

## dev status

### mimir-node

- [x] custom `web3` utility namespace w/ local node management helpers
- [x] `rpc` types w/ sanitization & block state injection (placeholders)
- [x] external transaction signing & compilation helpers
- [x] transparent jsonrpc error forwarding
- [ ] retry/reconnect on transport errors (may be worth adding to `web3` directly)
- [ ] fail-fast on chain reorgs (may require parity fork)

### mimir-worker
 
- [x] check for funded state on startup
- [x] check for bound state on startup
- [ ] populate new `BlockState` on each new block (with lag)
- [ ] minimal router client (redis-based OK for debug)
- [ ] minimal verifier client (simple `Cmp` based checks OK for debug)

### mimir-requester

TODO

### mimir-transport

- [x] separate blocking-only and nonblocking-only redis traits & handles
- [x] `Channel` type for routing info by role and (optionally) identity
- [x] `Message` type for operations passing through bridge (client to client)
- [x] Command` type for operations against bridge (client to server)
- [ ] implement ping/pong logic for all websocket servers
- [ ] move `mimir-node` crate into top-level `node` module
- [ ] move `Role` and `Identity` types to `mimir-types`

### mimir-service

- [x] minimal `auth-seeder` (bound oracles & presets only)
- [x] minimal `edge-server` (basic leasing & msg forwarding)
- [ ] simple "clock" based auth seeding & expiry (block number OK for mvp)
- [ ] connected set calculation & rate-limited marking of disconnects

