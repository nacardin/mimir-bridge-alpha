/// common types & utilities.
///

mod operation;
mod abilities;
mod identity;
mod command;
mod message;
mod channel;
mod auth;


pub use self::operation::Operation;
pub use self::abilities::Abilities;
pub use self::identity::Identity;
pub use self::message::Message;
pub use self::command::Command;
pub use self::channel::Channel;
pub use self::auth::Auth;


simple_error!(
    ParseError, "error during message/command parsing",
    BadMsgVariant => "unknown message variant",
    BadCmdVariant => "unknwon command variant",
    BadRoleVariant => "unknown variant for rolename",
    BadTimestamp => "unable to parse timestamp",
    BadSignature => "unabel to parse signature",
    BadChannel => "unable to parse channel",
    BadAddress => "unable to parse address",
    MissingVal => "missing required value(s)",
);

simple_unit!(
    MSG, "message operation variants",
    QUERY    => "QUERY",
    NOTARIZE => "NOTARIZE",
    YIELD    => "YIELD",
    ROUTE    => "ROUTE",
    VERIFY   => "VERIFY",
);


impl MSG {

    /// get consumer role for message type
    pub fn consumer(&self) -> Role {
        match *self {
            MSG::QUERY    => Role::Oracle,
            MSG::NOTARIZE => Role::Notary,
            MSG::YIELD    => Role::Requester,
            MSG::ROUTE    => Role::Router,
            MSG::VERIFY   => Role::Verifier,
        }
    }

    /// check if message is directed variant
    pub fn directed(&self) -> bool {
        match *self {
            MSG::QUERY    => false,
            MSG::NOTARIZE => false,
            MSG::YIELD    => true,
            MSG::ROUTE    => false,
            MSG::VERIFY   => true,
        }
    }
}


simple_unit!(
    CMD,"command operation variants",
    IDENTIFY => "IDENTIFY",
    DEBUG    => "DEBUG",
    KICK     => "KICK",
);


impl CMD {

    /// check if command is signed variant
    pub fn signed_variant(&self) -> bool {
        match *self {
            CMD::IDENTIFY => true,
            CMD::DEBUG => false,
            CMD::KICK => true,
        }
    }
}


simple_unit!(
    Role, "basic client roles", 
    Oracle    => "oracle",
    Notary    => "notary",
    Requester => "requester",
    Router    => "router", 
    Verifier  => "verifier",
    Admin     => "admin",
);


/*
#[derive(Debug,Clone,PartialEq,Eq,PartialOrd,Ord,Serialize,Deserialize)]
#[serde(tag = "op", content = "msg")]
#[serde(rename_all = "snake_case")]
pub enum Operation<T> {
    OnBlock(T),
    Query(T),
    Notarize(T),
    Yield(T),
    Route(T),
    Verify(T),
    Login(T),
}


impl<T> Operation<T> {

    pub fn into_inner(self) -> T {
        match self {
            Operation::OnBlock(val) => val,
            Operation::Query(val) => val,
            Operation::Notarize(val) => val,
            Operation::Yield(val) => val,
            Operation::Route(val) => val,
            Operation::Verify(val) => val,
            Operation::Login(val) => val,
        }
    }
}
*/
