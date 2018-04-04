/// common types & utilities.
///

mod command;
mod message;
mod channel;
mod auth;

pub use self::command::Command;
pub use self::message::Message;
pub use self::channel::Channel;
pub use self::auth::Auth;


simple_error!(
    ParseOpError, "error encountered during operation parsing",
    BadMsgFlag => "unknown message variant",
    BadCmdFlag => "unknown command variant",
    BadChannel => "invalid channel string",
    MissingParam => "missing required param(s)",
);


simple_unit!(
    MSG, "message operation variants",
    QUERY    => "QUERY",
    YIELD    => "YIELD",
    ROUTE    => "ROUTE",
    VERIFY   => "VERIFY",
);


impl MSG {

    pub fn consumer(&self) -> Role {
        match *self {
            MSG::QUERY  => Role::Oracle,
            MSG::YIELD  => Role::Notary,
            MSG::ROUTE  => Role::Router,
            MSG::VERIFY => Role::Verifier,
        }
    }
}

simple_unit!(
    CMD,"command operation variants",
    AUTH => "AUTH",
    CONN => "CONN",
    WORK => "WORK",
    KICK => "KICK",
);


simple_unit!(
    Role, "basic client roles",
    Oracle    => "oracle",
    Notary    => "notary",
    Router    => "router",
    Requester => "requester",
    Verifier  => "verifier",
    Validator => "validator",
);




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

