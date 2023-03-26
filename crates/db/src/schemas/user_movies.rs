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

#[derive(Debug, Deserialize, Serialize)]
pub struct NewMovieParams {
    pub movie_id: i32,
    pub title: String,
    pub user_id: Option<Uuid>,
    pub seen: Option<bool>,
    pub watch_again: Option<bool>,
    pub rating: Option<i32>,
}

impl UserMovie {
    pub async fn new(params: NewMovieParams) -> Result<UserMovie, UserMovieError> {
        let movie_id = params.movie_id;
        let title = params.title;
        let user_id = match params.user_id {
            Some(user_id) => user_id,
            None => return Err(Report::new(UserMovieError::InvalidArguments(String::from("user_id is required"))))
        };
        let seen = params.seen.unwrap_or(false);
        let watch_again = params.watch_again.unwrap_or(false);
        let rating = params.rating;

        let pool = get_connection_pool().await.map_err(|error| {
            let message = format!("Sqlx Error on [ get_connection_pool ] : {error}");
            error.change_context(UserMovieError::SqlxError).attach_printable(message)
        })?;
        
        let user_movie_exists = UserMovie::exists(user_id, movie_id).await.map_err(|error| {
            let message = format!("Sqlx Error on [ User::exists ] : {error}");
            error.change_context(UserMovieError::SqlxError).attach_printable(message)
        })?;

        if user_movie_exists {
            let message = String::from("UserMovie already exists");
            return Err(Report::new(UserMovieError::AlreadyExists(message.clone())).attach_printable(message.clone()));
        }

        let user_movie = sqlx::query_as!(UserMovie,
            "
            INSERT INTO public.user_movie 
            (movie_id, title, user_id, seen, watch_again, rating, created_on, updated_on) 
            VALUES 
            ($1, $2, $3, $4, $5, $6, NOW(), NOW()) RETURNING *;
            ", movie_id,title,user_id,seen,watch_again,rating)
            .fetch_one(&pool)
            .await
            .map_err(|error| {
                let message = format!("UserMovie creation failed with params: ({},{:?},{},{},{},{:?})", movie_id,title,user_id,seen,watch_again,rating);
                Report::new(UserMovieError::InvalidArguments(message.clone())).attach_printable(message.clone()).attach(error)
            })?;

        Ok(user_movie)
    }

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

    pub async fn exists(user_id: Uuid, movie_id: i32) -> Result<bool, UserMovieError>{
        let pool = get_connection_pool().await.map_err(|error| {
            let message = format!("Sqlx Error on [ get_connection_pool ] : {error}");
            error.change_context(UserMovieError::SqlxError).attach_printable(message)
        })?;
        
        let movies = sqlx::query!("SELECT um.id FROM public.user_movie um where um.user_id = $1 AND um.movie_id = $2;", user_id, movie_id)
            .fetch_all(&pool).await;

        match movies {
            Ok(movies) => Ok(movies.len() > 0),
            Err(error) => {
                let message = format!("Sqlx Error on [ UserMovie::exists ] : {error}");
                Err(Report::new(UserMovieError::SqlxError).attach_printable(message).attach(error))
            }
        }
    }
}
