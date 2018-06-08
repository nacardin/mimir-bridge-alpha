use mimir_common::Error as CommonError;
use tokio_tungstenite::tungstenite::Error as WebSocketError;
use tokio::timer::Error as TimerError;
use serde_json::Error as JsonError;
use std::str::Utf8Error;
use std::io::Error as IoError;
use std::{fmt,error};


/// transport error
///
/// top-level error type of the `mimir-transport` crate
///
#[derive(Debug)]
pub enum Error {
    /// generic error from `mimir-common`
    Common(CommonError),

    /// websocket error
    WebSocket(WebSocketError),

    /// tokio timer error
    Timer(TimerError),

    /// io error
    Io(IoError),

    /// json ser/de error
    Json(JsonError),
}


impl Error {

    pub fn message(msg: &'static str) -> Self { Self::from(msg) }

    pub fn generic<T>(msg: &'static str, data: T) -> Self where T: fmt::Debug + Send + 'static {
        CommonError::new(msg)
            .data(data)
            .into()
    }
}


impl fmt::Display for Error {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Common(err) => err.fmt(f),
            Error::WebSocket(err) => err.fmt(f),
            Error::Timer(err) => err.fmt(f),
            Error::Io(err) => err.fmt(f),
            Error::Json(err) => err.fmt(f),
        }
    }
}


impl error::Error for Error {

    fn description(&self) -> &str {
        match self {
            Error::Common(err) => err.description(),
            Error::WebSocket(err) => err.description(),
            Error::Timer(err) => err.description(),
            Error::Io(err) => err.description(),
            Error::Json(err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match self {
            Error::Common(err) => Some(err),
            Error::WebSocket(err) => Some(err),
            Error::Timer(err) => Some(err),
            Error::Io(err) => Some(err),
            Error::Json(err) => Some(err),
        }
    }
}


impl From<&'static str> for Error {

    fn from(msg: &'static str) -> Self { CommonError::from(msg).into() }
}

impl From<Utf8Error> for Error {

    fn from(_: Utf8Error) -> Self { Self::from("invalid utf-8") }
}

impl From<CommonError> for Error {

    fn from(err: CommonError) -> Self { Error::Common(err) }
}

impl From<WebSocketError> for Error {

    fn from(err: WebSocketError) -> Self { Error::WebSocket(err) }
}

impl From<TimerError> for Error {

    fn from(err: TimerError) -> Self { Error::Timer(err) }
}

impl From<IoError> for Error {

    fn from(err: IoError) -> Self { Error::Io(err) }
}

impl From<JsonError> for Error {

    fn from(err: JsonError) -> Self { Error::Json(err) }
}
