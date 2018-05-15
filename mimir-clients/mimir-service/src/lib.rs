//! Identityless mimir-bridge microservices.
//!
//! This crate implements logic specific to the identityless subset of mimir-bridge microservices.
//! Most microservices have a specific cryptographic identity and role, defined by blockchain state
//! (referred to as "workers").  Those that don't (referred to more generally as "services") are 
//! essentially implementation details (eg; the `edge-server` service, which manages message routing 
//! between active workers, but has no cryptographic authority).  In general, workers perform and
//! are held accountable for all protocol work, services just manage practicalities.  It is a
//! fundamental error for any compromised service to be able to perform any malaicious action
//! greater than censorship.  Take routing for example; the `edge-server` (service) handles actually 
//! moving a message to its destination, but a `router` (worker) must produce and commit to a proof
//! of the validity of the route.
//!
extern crate mimir_transport;
extern crate mimir_types;
extern crate mimir_util;
extern crate mimir_node;
#[macro_use]
extern crate structopt;
extern crate tokio_timer;
extern crate futures;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate web3;
extern crate url_serde;
extern crate url;
#[macro_use]
extern crate log;


pub mod auth_seeder;
pub mod edge_server;

