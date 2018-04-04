[travis-badge]: https://travis-ci.org/mimirblockchainsolutions/mimir-bridge-alpha.svg?branch=master
[travis-url]: https://travis-ci.org/mimirblockchainsolutions/mimir-bridge-alpha

# mimir-bridge-alpha [![Build Status][travis-badge]][travis-url]

Experimental Rust implementation of Mimir Bridge clients & protocols.


## About

This repository is part of the alpha/PoC iteration of the Mimir Bridge, a decentralized
blockchain API system. The goal of the Mimir Bridge system is to achieve the speed 
and convenience of classic client/server models (for both developers *and* end-users), 
while preserving the decentralized security model.

The code in this repository is roughly divided between generic protocol types/utilities
and concrete client software implementations located in [`mimir-bridge`](./mimir-bridge)
and [`mimir-clients`](./mimir-clients) respectively.  All components are currently under
rapid development/iteration, so expect regular braking changes.


## Getting Started

Make sure you have a recent verion of [Rust](https://www.rust-lang.org) installed (stable OK).

Check out auto-generated API docs with `cargo doc --open`.

High level docs coming soon.

