use common::{
    ParseError,
    Channel,
    Message,
    Command,
    MSG,
    CMD
};
use std::str::FromStr;
use std::fmt;


/// one of either a command or message
#[derive(Debug,Clone)]
pub enum Operation {
    Command(Command),
    Message(Message),
}


impl Operation {

    /// attempt to parse from string buffer
    pub fn from_string(buf: String) -> Result<Self,ParseError> {
        match Command::from_str(&buf) {
            Ok(cmd) => Ok(Self::from(cmd)),
            Err(_) => {
                let msg: Message = Message::from_string(buf)?;
                Ok(Self::from(msg))
            }
        }
    }

    /// get destination channel of operation
    pub fn dest_channel(&self) -> Channel {
        match *self {
            Operation::Command(ref cmd) => cmd.dest_channel(),
            Operation::Message(ref msg) => msg.dest_channel(),
        }
    }


    /// if operation matches specified `CMD` instance, extract
    /// and return inner command.
    pub fn expect_command(self, cmd: CMD) -> Result<Command,Self> {
        match self {
            Operation::Command(command) => {
                if command.flag == cmd {
                    Ok(command)
                } else {
                    Err(Self::from(command))
                }
            },
            other => Err(other)
        }
    }

    /// if operation matches specified `MSG` instance, extract
    /// and return inner message.
    pub fn expect_message(self, msg: MSG) -> Result<Message,Self> {
        match self {
            Operation::Message(message) => {
                if message.msg_variant() == msg {
                    Ok(message)
                } else {
                    Err(Self::from(message))
                }
            },
            other => Err(other)
        }
    }
}


impl Into<String> for Operation {

    fn into(self) -> String {
        match self {
            Operation::Message(msg) => msg.into(),
            Operation::Command(cmd) => cmd.to_string(),
        }
    }
}


impl fmt::Display for Operation {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Operation::Command(ref cmd) => cmd.fmt(f),
            Operation::Message(ref msg) => msg.fmt(f),
        }
    }
}


impl From<Command> for Operation {

    fn from(cmd: Command) -> Self { Operation::Command(cmd) }
}

impl From<Message> for Operation {

    fn from(msg: Message) -> Self { Operation::Message(msg) }
}
