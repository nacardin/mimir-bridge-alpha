//! utilities for operating on toml files.
//! 
//! This module provides a simple means of loading and saving
//! values as toml files:
//!
//! ```
//! # extern crate mimir_util;
//! # fn main() {
//! use std::collections::HashMap;
//! use mimir_util::toml;
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
use std::io::{Read,Write};
use std::fs::OpenOptions;
use std::path::Path;
use _toml;


simple_error!(
    TomlError,
    NoSuchFile => "specified file does not exist",
    OpenFile => "unable to open file",
    ReadFile => "unable to read from file",
    WriteFile => "unable to write to file",  
    Serialize => "unable to serialize as toml",
    Deserialize => "unable to deserialize as toml",
);


/// attempt to load value from a toml file.
///
pub fn load<T,P>(path: P) -> Result<T,TomlError> where T: DeserializeOwned, P: AsRef<Path> {
    let path: &Path = path.as_ref();
    if path.is_file() {
        let mut file = OpenOptions::new()
            .read(true)
            .open(path)
            .map_err(|_| TomlError::OpenFile)?;
        let mut buff = String::new();
        file.read_to_string(&mut buff)
            .map_err(|_| TomlError::ReadFile)?;
        let deserialized = _toml::from_str(&buff)
            .map_err(|_| TomlError::Deserialize)?;
        Ok(deserialized)
    } else {
        Err(TomlError::NoSuchFile)
    }
}


/// attempt to save value to a toml file.
///
pub fn save<T,P>(value: &T, path: P) -> Result<(),TomlError> where T: Serialize, P: AsRef<Path> {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path.as_ref())
        .map_err(|_| TomlError::OpenFile)?;
    let serialized = _toml::to_string(value)
        .map_err(|_| TomlError::Serialize)?;
    file.write_all(serialized.as_ref())
        .map_err(|_| TomlError::WriteFile)
}

