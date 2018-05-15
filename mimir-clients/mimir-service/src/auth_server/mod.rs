//! auth server utilities.
//!
pub(crate) mod seeding;
pub(crate) mod options;
pub(crate) mod config;

pub use self::seeding::{
    SeedLoader,
    SeedSources,
    SeedConfig,
    SeedState,
};
pub use self::options::Options;
pub use self::config::Config;


use futures::future::Future;
use mimir_transport::redis::{
    RedisNonBlock,
    Error,
};


/// apply basic seeding with provided redis handle
///
// NOTE: this is placeholder logic; final version must store & clear set state,
// and take care to use proper ordering.
//
pub fn apply_seeding<R: RedisNonBlock>(redis: R, seed_state: SeedState) -> impl Future<Item=(),Error=Error> {
    let SeedState { role, conn, auth } = seed_state;
    let conn_idents = conn.iter().map(|addr| format!("{}::{:#}",role,addr));
    let auth_idents = auth.iter().map(|addr| format!("{}::{:#}",role,addr));
    let conn_key = format!("{}::conn-lease",role);
    let auth_key = format!("{}::auth-lease",role);
    let conn_work = redis.sadd(conn_key,conn_idents);
    let auth_work = redis.sadd(auth_key,auth_idents);
    let seed_work = conn_work.join(auth_work).map(|(c,a)| {
        debug!("seeded {} to `conn` and {} to `auth`...",c,a)
    });
    seed_work
}

