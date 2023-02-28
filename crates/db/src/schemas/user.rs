use chrono::{NaiveDateTime};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow};
use uuid::Uuid;


use crate::connection::get_connection_pool;
use crate::errors::OperationError;
use crate::errors::already_exists::AlreadyExistsError;

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct User {
    id: Uuid,
    username: String,
    created_on: NaiveDateTime,
    updated_on: NaiveDateTime,
}

impl User {
    pub async fn new(username: &str) -> Result<bool, OperationError<User>> {
        let pool = get_connection_pool().await?;
        
        let user_exists = User::exists(username).await?;
        if user_exists {
            return Err(OperationError::from(AlreadyExistsError::<User>::new_none("User already exists")));
        }

        // let users = sqlx::query!("INSERT yadda yadda", username)
        // .fetch_all(&pool).await;

        todo!()
    }

    pub async fn exists(username: &str) -> Result<bool, sqlx::Error>{
        let pool = get_connection_pool().await?;
        
        let users = sqlx::query!("SELECT u.id FROM public.user u where u.username = $1;", username)
            .fetch_all(&pool).await;

        match users {
            Ok(users) => Ok(users.len() > 0),
            Err(err) => Err(err)
        }
    }
}
