use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct AlreadyExistsError<Resource> {
    description: String,
    record: Option<Resource>
}

impl<Record> AlreadyExistsError<Record> {
    pub fn new_none(msg: &str) -> AlreadyExistsError<Record> {
        AlreadyExistsError{description: msg.to_string(), record: None}
    }
    
    pub fn new_record(msg: &str, record: Record) -> AlreadyExistsError<Record> {
        AlreadyExistsError{description: msg.to_string(), record: Some(record) }
    }
}

impl<Record> fmt::Display for AlreadyExistsError<Record> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.description)
    }
}

impl<Record: std::fmt::Debug> Error for AlreadyExistsError<Record> {
    fn description(&self) -> &str {
        &self.description
    }
}