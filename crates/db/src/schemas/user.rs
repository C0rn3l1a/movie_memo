use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow};

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct User {
    id: String,
    username: String,
    created_on: DateTime<Utc>,
    updated_on: DateTime<Utc>,
}

impl User {
    pub fn new(username: String) -> Vec<User> {
        // do sqlx thing here
        todo!()
        
    }
}
