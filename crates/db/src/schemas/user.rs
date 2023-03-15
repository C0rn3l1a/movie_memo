use chrono::{NaiveDateTime};
use error_stack::{Report, Result};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow};
use std::{fmt, error::Error};
use uuid::Uuid;

use crate::connection::get_connection_pool;

// Define function error [FnNewUserError] for {User::new}
#[derive(Debug)]
pub enum FnNewUserError {
    SqlxError,
    AlreadyExists(String),
    InvalidArguments(String)
}

impl fmt::Display for FnNewUserError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str("New User Error: Could not create user.")
    }
}
impl Error for FnNewUserError {}

// Define function error [FnUserExistsError] for {User::exists}
#[derive(Debug)]
pub enum FnUserExistsError {
    SqlxError
}

impl fmt::Display for FnUserExistsError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str("User Exists Error: Failed to check if user exists.")
    }
}

impl Error for FnUserExistsError {}

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct User {
    id: Uuid,
    username: String,
    created_on: NaiveDateTime,
    updated_on: NaiveDateTime,
}

// Define [User] struct
impl User {
    pub async fn new(username: &str) -> Result<User, FnNewUserError> {
        let pool = get_connection_pool().await.map_err(|error| {
            let message = format!("Sqlx Error on [ get_connection_pool ] : {error}");
            error.change_context(FnNewUserError::SqlxError).attach_printable(message)
        })?;
        
        let user_exists = User::exists(username).await.map_err(|error| {
            let message = format!("Sqlx Error on [ User::exists ] : {error}");
            error.change_context(FnNewUserError::SqlxError).attach_printable(message)
        })?;

        if user_exists {
            let message = String::from("User already exists");
            return Err(Report::new(FnNewUserError::AlreadyExists(message.clone())).attach_printable(message.clone()));
        }

        let users = sqlx::query_as!(User,"INSERT INTO public.user (id, username, created_on, updated_on) VALUES (gen_random_uuid(), $1, NOW(), NOW()) RETURNING *;", username)
            .fetch_all(&pool)
            .await
            .map_err(|_| {
                let message = String::from("User creation failed with params: ({username})");
                Report::new(FnNewUserError::InvalidArguments(message.clone())).attach_printable(message.clone())
            })?;

        Ok(users.into_iter().next().unwrap())
    }
    
    pub async fn exists(username: &str) -> Result<bool, FnUserExistsError>{
        let pool = get_connection_pool().await.map_err(|error| {
            let message = format!("Sqlx Error on [ get_connection_pool ] : {error}");
            error.change_context(FnUserExistsError::SqlxError).attach_printable(message)
        })?;
        
        let users = sqlx::query!("SELECT u.id FROM public.user u where u.username = $1;", username)
            .fetch_all(&pool).await;

        match users {
            Ok(users) => Ok(users.len() > 0),
            Err(error) => {
                let message = format!("Sqlx Error on [ User::exists ] : {error}");
                Err(Report::new(FnUserExistsError::SqlxError).attach_printable(message))
            }
        }
    }
}
