# mimir-bridge

core mimir-bridge functionality implemented in rust.


## dev status

### mimir-proto

- message protocol
  - [x] simple message and cert types & document placeholders
  - [x] basic cryptographic cert generation & checking
  - [x] visitor traits for message & block state verification
  - [ ] refactor message type to be generic over `record` (requires `Hashable` trait)

- routing tree 
  - [x] routing tree primitives & impls for branch & leaf nodes
  - [x] node hashing & traversal policy helpers
  - [ ] tree construction utilities (requires finalization of tree padding algo)
  - [ ] proof construction utilities
  - [ ] top-level proof & tree types & impls
  - [ ] replace placeholder proofs in existing message utils

- judgement visitor
  - [x] accusation primitives & helpers
  - [x] basic judgement & acccusation tracking
  - [ ] refactor judgement compilation (requires finalization of solidity refactor)


### mimir-crypto

- [x] generic `Hasher` and `Signer` traits.
- [x] `Hasher` implementation for `keccak-256`
- [x] ethereum style ecc primitives including `Address`
- [x] `Signer` implementation for ethereum style `secp256k1`
- [ ] `Hashable` trait for self-describing data


### mimir-types

- [x] bytes-like primitves (`U256`,`H256`,etc...)
- [x] reexport ethereum crypo types


### mimir-util

- [x] aggregate macros for common patterns & boilerplate
- [x] aggregate common helper functions (`hex`,`toml`,`time`,etc...).


