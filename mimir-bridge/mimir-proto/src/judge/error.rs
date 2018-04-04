//! errors which indicate a bad judgement.
//!
use mimir_types::Address;
use visit::CertError;
use std::{fmt,error};


/// error encountered by judge
#[derive(Debug,Copy,Clone,PartialEq,Eq)]
pub enum JudgeError {
    /// malformed cert
    Cert {
        /// inner error value
        err: CertError
    },
    
    /// nonexistent entity
    Entity {
        /// address which failed lookup
        addr: Address
    }
}


impl JudgeError {

    /// get simple static error message
    pub fn as_str(&self) -> &'static str {
        match *self {
            JudgeError::Cert { ref err } => err.as_str(),
            JudgeError::Entity { .. } => "nonexistent entity"
        }
    }
}


impl fmt::Display for JudgeError {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            JudgeError::Cert { ref err } => err.fmt(f),
            JudgeError::Entity { ref addr } => {
                write!(f,"entity `{:?}` does not exist",addr)
            }
        }
    }
}

impl error::Error for JudgeError {

    fn description(&self) -> &str {
        self.as_str()
    }
}


impl From<Address> for JudgeError {

    fn from(addr: Address) -> Self {
        JudgeError::Entity { addr }
    }
}


impl From<CertError> for JudgeError {

    fn from(err: CertError) -> Self {
        JudgeError::Cert { err }
    }
}
