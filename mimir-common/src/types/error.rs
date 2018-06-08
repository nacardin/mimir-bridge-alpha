use util::hex::ParseHexError;
use std::fmt::{self,Debug};
use std::error;


#[derive(Debug)]
/// simple general-purpose error type
///
/// simple error type for use in situations where the program has no interest
/// in the underlying cause of an error, but granular debug information
/// may still be useful.
///
pub struct Error<T=Box<Debug + Send>> {
    /// static description of the error
    pub message: &'static str,
    
    /// optional associated data (e.g.; inputs which triggered the error)
    pub data: Option<T>,
}


impl<T> Error<T> {

    /// build new error with specified message
    pub fn new(message: &'static str) -> Self { Self::from_parts(message,None) }

    /// build error from individual components
    pub fn from_parts(message: &'static str, data: Option<T>) -> Self {
        Self { message, data }
    }
}


impl Error<Box<Debug + Send>> {

    /// 
    pub fn data<T>(mut self, error_data: T) -> Self where T: Debug + Send + 'static {
        self.data = Some(Box::new(error_data));
        self
    }
}


impl<T> fmt::Display for Error<T> where T: Debug {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.message)
    }
}


impl<T> error::Error for Error<T> where T: Debug {

    fn description(&self) -> &'static str { self.message }
}


impl<T> From<ParseHexError> for Error<T> {

    fn from(err: ParseHexError) -> Self { Self::new(err.as_str()) }
}

impl<T> From<&'static str> for Error<T> {

    fn from(message: &'static str) -> Self { Self::new(message) }
}


impl<T> From<(&'static str,T)> for Error where T: Debug + Send + 'static {

    fn from((message,data): (&'static str,T)) -> Self {
        Self::new(message)
            .data(data)
    }
}
