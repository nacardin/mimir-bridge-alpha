use serde_json::Error as JsonError;
use web3::Error as Web3Error;
use std::{fmt,error};


#[derive(Debug)]
pub enum OracleError {
    Json(JsonError),
    Web3(Web3Error),
}


impl From<JsonError> for OracleError {
    
    fn from(err: JsonError) -> Self { OracleError::Json(err) }
}


impl From<Web3Error> for OracleError {

    fn from(err: Web3Error) -> Self { OracleError::Web3(err) }
}


impl fmt::Display for OracleError {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            OracleError::Json(ref err) => err.fmt(f),
            OracleError::Web3(ref err) => err.fmt(f),
        }
    }
}


impl error::Error for OracleError {

    fn description(&self) -> &str {
        match *self {
            OracleError::Json(ref err) => err.description(),
            OracleError::Web3(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            OracleError::Json(ref err) => Some(err),
            OracleError::Web3(ref err) => Some(err),
        }
    }
}


