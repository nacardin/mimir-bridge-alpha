use edge::Error;
use common::{
    Operation,
    Abilities,
    Identity,
    Message,
    Command, 
    CMD,
};


#[derive(Debug,Clone,PartialEq,Eq)]
pub struct OperationFilter {
    /// capabilities of connected entity
    abilities: Abilities,

    /// identity of connected entity
    identity: Identity,
}


impl OperationFilter {

    /// initialize new connection filter
    pub fn new(identity: Identity) -> Self {
        let abilities = Abilities::new(identity.role);
        Self { abilities, identity }
    }


    /// filter incoming (client -> redis) operation. 
    pub fn filter_incoming(&self, op: Operation) -> Result<Option<Operation>,Error> {
        match op {
            Operation::Message(msg) => {
                let rslt = self.filter_incoming_msg(msg)?;
                Ok(rslt.map(|msg|msg.into()))
            },
            Operation::Command(cmd) => {
                let rslt = self.filter_incoming_cmd(cmd)?;
                Ok(rslt.map(|cmd|cmd.into()))
            },
        }
    }

    fn filter_incoming_msg(&self, msg: Message) -> Result<Option<Message>,Error> {
        if self.abilities.can_produce(msg.msg_variant()) {
            if &self.identity.address == msg.source_address() {
                Ok(Some(msg))
            } else {
                Err("invalid source address".into())
            }
        } else {
            Err("unauthorized message variant".into())
        }
    }

    fn filter_incoming_cmd(&self, cmd: Command) -> Result<Option<Command>,Error> {
        if self.abilities.can_execute(cmd.cmd_variant()) {
            Ok(Some(cmd))
        } else {
            Err("unauthorized command variant".into())
        }
    }

    /// filter outoing (redis -> client) operation.
    pub fn filter_outgoing(&self, op: Operation) -> Result<Option<Operation>,Error> {
        match op {
            Operation::Message(msg) => {
                let rslt = self.filter_outgoing_msg(msg)?;
                Ok(rslt.map(|msg|msg.into()))
            },
            Operation::Command(cmd) => {
                let rslt = self.filter_outgoing_cmd(cmd)?;
                Ok(rslt.map(|msg|msg.into()))
            },
        }
    }

    fn filter_outgoing_msg(&self, msg: Message) -> Result<Option<Message>,Error> {
        // TODO: verify message destination...
        Ok(Some(msg))
    }

    fn filter_outgoing_cmd(&self, cmd: Command) -> Result<Option<Command>,Error> {
        // TODO: verify command certs, target, & expiry...
        match cmd.flag {
            CMD::IDENTIFY => Err("unexpected `CMD` variant `IDENTIFY`".into()),
            CMD::DEBUG    => {
                info!("command `{}`",cmd);
                Ok(Some(cmd))
            },
            CMD::KICK     => {
                info!("kick command against {}",self.identity);
                Err("connection terminated (KICK)".into())
            },
        }
    }
}

