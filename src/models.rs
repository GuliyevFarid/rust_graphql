use serde::{Serialize, Deserialize};
use async_graphql::SimpleObject;
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, SimpleObject, FromRow)]
pub struct DBUser {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub age: i32,
}