//! utilities for operating on toml files.
//! 
//! This module provides a simple means of loading and saving
//! values as toml files:
//!
//! ```
//! # extern crate mimir_common;
//! # fn main() {
//! use std::collections::HashMap;
//! use mimir_common::util::toml;
//! 
//! let mut mapping = HashMap::new();
//! 
//! mapping.insert("foo".to_string(),123);
//! 
//! mapping.insert("bar".to_string(),456);
//!
//! toml::save(&mapping,"/tmp/mapping.toml").unwrap();
//! 
//! assert_eq!(mapping,toml::load("/tmp/mapping.toml").unwrap());
//! # }
//! ```
//!
use serde::de::DeserializeOwned;
use serde::ser::Serialize;
use std::io::{self,Read,Write};
use std::fs::OpenOptions;
use std::path::Path;
use std::{fmt,error};
use toml::{self,ser,de};



/// attempt to load value from a toml file.
///
pub fn load<T,P>(path: P) -> Result<T,Error> where T: DeserializeOwned, P: AsRef<Path> {
    let path: &Path = path.as_ref();
    let mut file = OpenOptions::new()
        .read(true)
        .open(path)?;
    let mut buff = String::new();
    file.read_to_string(&mut buff)?;
    from_str(&buff)
}


/// attempt to parse `str` as toml
///
pub fn from_str<T>(buff: &str) -> Result<T,Error> where T: DeserializeOwned {
    let deserialized = toml::from_str(buff)?;
    Ok(deserialized)
}


/// attempt to save value to a toml file.
///
pub fn save<T,P>(value: &T, path: P) -> Result<(),Error> where T: Serialize, P: AsRef<Path> {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path.as_ref())?;
    let serialized = toml::to_string(value)?;
    file.write_all(serialized.as_ref())?;
    Ok(())
}


/// error while working with `toml` values
///
#[derive(Debug)]
pub enum Error {
    /// error during serialization
    Serialize(ser::Error),
    /// error during deserialization
    Deserialize(de::Error),
    /// error during file i/o
    FileIo(io::Error),
}


impl fmt::Display for Error {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Serialize(err) => err.fmt(f),
            Error::Deserialize(err) => err.fmt(f),
            Error::FileIo(err) => err.fmt(f),
        }
    }
}


impl error::Error for Error {

    fn description(&self) -> &str {
        match self {
            Error::Serialize(err) => err.description(),
            Error::Deserialize(err) => err.description(),
            Error::FileIo(err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match self {
            Error::Serialize(err) => Some(err),
            Error::Deserialize(err) => Some(err),
            Error::FileIo(err) => Some(err),
        }
    }
}


impl From<ser::Error> for Error {

    fn from(err: ser::Error) -> Self { Error::Serialize(err) }
}


impl From<de::Error> for Error {

    fn from(err: de::Error) -> Self { Error::Deserialize(err) }
}


impl From<io::Error> for Error {

    fn from(err: io::Error) -> Self { Error::FileIo(err) }
}

