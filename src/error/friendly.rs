use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct FriendlyError {
    description: String
}

impl FriendlyError {
    fn new(msg: &str) -> FriendlyError {
        FriendlyError{description: msg.to_string()}
    }
}

impl fmt::Display for FriendlyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.description)
    }
}

impl Error for FriendlyError {
    fn description(&self) -> &str {
        &self.description
    }
}