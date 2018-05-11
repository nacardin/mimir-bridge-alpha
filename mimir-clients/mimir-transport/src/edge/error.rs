use redis_async::error::Error as RedisError;
use tokio_timer::Error as TimerError;
use websocket::WebSocketError;
use common::ParseError;
use std::{fmt,error};


/// generic error type for higher-level edge-connection utilties
///
#[derive(Debug)]
pub enum Error {
    /// error originating from `tokio-timer`
    Timer(TimerError),

    /// error originating from `redis-async`
    Redis(RedisError),

    /// error originating from `websocket`
    WebSocket(WebSocketError),

    /// error during message parsing
    Parsing(ParseError),

    /// generic error variant
    Other(&'static str)
}


impl fmt::Display for Error {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Timer(ref err) => err.fmt(f),
            Error::Redis(ref err) => err.fmt(f),
            Error::WebSocket(ref err) => err.fmt(f),
            Error::Parsing(ref err) => err.fmt(f),
            Error::Other(ref msg) => f.write_str(msg),
        }
    }
}


impl error::Error for Error {

    fn description(&self) -> &str {
        match *self {
            Error::Timer(ref err) => err.description(),
            Error::Redis(ref err) => err.description(),
            Error::WebSocket(ref err) => err.description(),
            Error::Parsing(ref err) => err.description(),
            Error::Other(ref msg) => msg
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::Timer(ref err) => Some(err),
            Error::Redis(ref err) => Some(err),
            Error::WebSocket(ref err) => Some(err),
            Error::Parsing(ref err) => Some(err),
            Error::Other(_) => None
        }
    }
}


impl From<TimerError> for Error {

    fn from(err: TimerError) -> Self { Error::Timer(err) }
}

impl From<RedisError> for Error {

    fn from(err: RedisError) -> Self { Error::Redis(err) }
}

impl From<WebSocketError> for Error {

    fn from(err: WebSocketError) -> Self { Error::WebSocket(err) }
}

impl From<ParseError> for Error {

    fn from(err: ParseError) -> Self { Error::Parsing(err) }
}

impl From<&'static str> for Error {

    fn from(msg: &'static str) -> Self { Error::Other(msg) }
}
