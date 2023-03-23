use chrono::{NaiveDateTime};
use error_stack::{Report, Result};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow};
use std::{fmt, error::Error};
use uuid::Uuid;
use crate::connection::get_connection_pool;

#[derive(Debug)]
pub enum UserMovieError {
    SqlxError,
    AlreadyExists(String),
    InvalidArguments(String),
    NotFound(String),
}

// Define [UserMovieError] struct
impl fmt::Display for UserMovieError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UserMovieError::SqlxError => fmt.write_str("Database error"),
            UserMovieError::AlreadyExists(_) => fmt.write_str("UserMovie already exists"),
            UserMovieError::InvalidArguments(_) => fmt.write_str("Invalid arguments"),
            UserMovieError::NotFound(_) => fmt.write_str("UserMovie not found"),
        }
    }
}

impl Error for UserMovieError {}

// Define [UserMovie] struct
#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct UserMovie {
    id: i32,
    movie_id: i32,
    user_id: Option<Uuid>,
    title: String,
    seen: bool,
    watch_again: bool,
    rating: Option<i32>,
    created_on: NaiveDateTime,
    updated_on: NaiveDateTime,
}

impl UserMovie {
    pub async fn get_by_user_id(user_id: &Uuid) -> Result<Vec<UserMovie>, UserMovieError>{
        let pool = get_connection_pool().await.map_err(|error| {
            let message = format!("Sqlx Error on [ get_connection_pool ] : {error}");
            error.change_context(UserMovieError::SqlxError).attach_printable(message)
        })?;

        let user_movies = sqlx::query_as!(UserMovie,"SELECT * FROM public.user_movie um where um.user_id = $1;", user_id)
            .fetch_all(&pool)
            .await
            .map_err(|error| {
                let message = format!("UserMovie query failed with params: ({})", user_id);
                Report::new(UserMovieError::InvalidArguments(message.clone())).attach_printable(message.clone()).attach(error)
            })?;

        Ok(user_movies)
    }
}
