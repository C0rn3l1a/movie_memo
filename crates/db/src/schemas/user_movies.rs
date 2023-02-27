use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct UserMovie {
    id: i32,
    movie_id: i32,
    user_id: Option<String>,
    title: String,
    seen: bool,
    watch_again: bool,
    rating: i32,
    created_on: DateTime<Utc>,
    updated_on: DateTime<Utc>,
}

impl UserMovie {
    pub fn get_by_user_id(user_id: i32) -> Vec<UserMovie> {
        // do sqlx thing here
        vec![UserMovie {
            id: 123,
            user_id: None,
            created_on: Utc::now(),
            updated_on: Utc::now(),
            movie_id: 123,
            rating: 5,
            title: String::from("value"),
            seen: true,
            watch_again: false,
        }]
    }
}
