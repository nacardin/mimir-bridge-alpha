use mimir_util::unix_time;
use mimir_types::Address;
use std::cell::Cell;
use edge::Error;
use common::{
    RawMessage,
    Abilities,
    Channel,
    Role,
    MSG,
};


#[derive(Debug,Clone,PartialEq,Eq)]
pub struct ConnState {
    /// timestamp of last `auth` event
    last_auth: Cell<u64>,
    
    /// capabilities of connected entity
    abilities: Abilities,
    
    /// address of connected entity
    addr: Address,
    
    /// role of connected entity
    role: Role, 
}


impl ConnState {

    /// initialize new connection state
    pub fn new(addr: Address, role: Role) -> Self {
        let last_auth = Cell::new(0);
        let abilities = Abilities::new(role);
        Self { last_auth, abilities, addr, role }
    }


    /// visit raw incoming message string.
    ///
    /// typically returns a `RawMessage` instance to be forwarded to a redis queue.
    /// if `Ok(None)` is returned, message was consumed by internal state update.
    ///
    pub fn visit_incoming(&self, raw: String) -> Result<Option<RawMessage>,Error> {
        let msg = RawMessage::from_string(raw)?;
        if self.can_produce(msg.msg_variant()) {
            if &self.addr == msg.source_address() {
                Ok(Some(msg))
            } else {
                Err("invalid source address".into())
            }
        } else {
            Err("unauthorized message variant".into())
        }
    }

    /// visit raw outgoing message string.
    ///
    /// expects `src` to be the name of the source channel, and `raw` to be the full
    /// message string.  
    pub fn visit_outgoing(&self, src: String, raw: String) -> Result<Option<RawMessage>,Error> {
        match src.parse()? {
            channel @ Channel::Shared { .. } => {
                let msg = RawMessage::from_string(raw)?;
                debug_assert!(msg.dest_channel() == channel,"channel dest should always match");
                Ok(Some(msg))
            },
            channel @ Channel::Direct { .. } => {
                match raw.as_ref() {
                    "AUTHORIZE" => {
                        self.set_last_auth(unix_time());
                        Ok(None)
                    },
                    "KICK"      => {
                        self.set_last_auth(0);
                        Err("connection terminated (KICK)".into())
                    },
                    _ => {
                        let msg = RawMessage::from_string(raw)?;
                        debug_assert!(msg.dest_channel() == channel,"channel dest should always match");
                        Ok(Some(msg))
                    }
                }
            }
        }
    }

    /// set timestamp of last auth
    pub fn set_last_auth(&self, time: u64) { self.last_auth.set(time) }

    /// get timestamp of last auth
    pub fn get_last_auth(&self) -> u64 { self.last_auth.get() }

    /// get shared channel for this connection
    pub fn shared_channel(&self) -> Channel {
        Channel::Shared { role: self.role }
    }

    /// get direct channel for this connection
    pub fn direct_channel(&self) -> Channel {
        Channel::Direct { role: self.role, addr: self.addr }
    }

    /// check if able to produce specified message variant
    pub fn can_produce(&self, msg: MSG) -> bool { self.abilities.can_produce(msg) }
}

