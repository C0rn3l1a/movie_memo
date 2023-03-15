use chrono::{NaiveDateTime};
use error_stack::{Report, Result};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow};
use std::{fmt, error::Error};
use uuid::Uuid;

use crate::connection::get_connection_pool;

#[derive(Debug)]
pub enum UserError {
    SqlxError,
    AlreadyExists(String),
    InvalidArguments(String),
    NotFound(String),
}

impl fmt::Display for UserError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UserError::SqlxError => fmt.write_str("Database error"),
            UserError::AlreadyExists(_) => fmt.write_str("User already exists"),
            UserError::InvalidArguments(_) => fmt.write_str("Invalid arguments"),
            UserError::NotFound(_) => fmt.write_str("User not found"),
        }
    }
}

impl Error for UserError {}

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct User {
    id: Uuid,
    username: String,
    created_on: NaiveDateTime,
    updated_on: NaiveDateTime,
}

// Define [User] struct
impl User {
    pub async fn new(username: &str) -> Result<User, UserError> {
        let pool = get_connection_pool().await.map_err(|error| {
            let message = format!("Sqlx Error on [ get_connection_pool ] : {error}");
            error.change_context(UserError::SqlxError).attach_printable(message)
        })?;
        
        let user_exists = User::exists(username).await.map_err(|error| {
            let message = format!("Sqlx Error on [ User::exists ] : {error}");
            error.change_context(UserError::SqlxError).attach_printable(message)
        })?;

        if user_exists {
            let message = String::from("User already exists");
            return Err(Report::new(UserError::AlreadyExists(message.clone())).attach_printable(message.clone()));
        }

        let users = sqlx::query_as!(User,"INSERT INTO public.user (id, username, created_on, updated_on) VALUES (gen_random_uuid(), $1, NOW(), NOW()) RETURNING *;", username)
            .fetch_all(&pool)
            .await
            .map_err(|_| {
                let message = String::from("User creation failed with params: ({username})");
                Report::new(UserError::InvalidArguments(message.clone())).attach_printable(message.clone())
            })?;

        Ok(users.into_iter().next().unwrap())
    }
    
    pub async fn exists(username: &str) -> Result<bool, UserError>{
        let pool = get_connection_pool().await.map_err(|error| {
            let message = format!("Sqlx Error on [ get_connection_pool ] : {error}");
            error.change_context(UserError::SqlxError).attach_printable(message)
        })?;
        
        let users = sqlx::query!("SELECT u.id FROM public.user u where u.username = $1;", username)
            .fetch_all(&pool).await;

        match users {
            Ok(users) => Ok(users.len() > 0),
            Err(error) => {
                let message = format!("Sqlx Error on [ User::exists ] : {error}");
                Err(Report::new(UserError::SqlxError).attach_printable(message))
            }
        }
    }
}
