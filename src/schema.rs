use crate::{errors::UserError, models::DBUser};
use async_graphql::{
    Context, EmptySubscription, Error, ErrorExtensions, InputObject, Object, Schema, SimpleObject,
};
use lazy_static::lazy_static;
use sqlx::PgPool;
use std::sync::Mutex;

#[derive(SimpleObject, Clone)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub age: u8,
}

#[derive(InputObject)]
pub struct UserSearchInput {
    pub name: Option<String>,
    pub email: Option<String>,
    pub age: Option<u8>,
}

#[derive(InputObject)]
pub struct UserCreateInput {
    pub name: String,
    pub email: String,
    pub age: u8,
}

lazy_static! {
    static ref USERS: Mutex<Vec<User>> = Mutex::new(Vec::new());
}

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn db_users(&self, ctx: &Context<'_>) -> Result<Vec<DBUser>, Error> {
        let pool = ctx
            .data::<PgPool>()
            .map_err(|_| UserError::DatabaseError("Failed to get DB pool".to_string()))?;
        let users = sqlx::query_as::<_, DBUser>("SELECT * FROM users")
            .fetch_all(pool)
            .await?;
        Ok(users)
    }

    async fn db_user_by_id(&self, ctx: &Context<'_>, id: i32) -> Result<DBUser, Error> {
        let pool = ctx
            .data::<PgPool>()
            .map_err(|_| UserError::DatabaseError("Failed to get DB pool".to_string()))?;
        let user = sqlx::query_as!(DBUser, "SELECT * FROM users WHERE id = $1", id)
            .fetch_optional(pool)
            .await
            .map_err(UserError::from)?;

        user.ok_or_else(|| UserError::NotFound.into())
    }

    async fn all_users(&self) -> Vec<User> {
        let users = USERS.lock().unwrap();
        users.to_vec()
    }

    async fn user_by_id(&self, id: i32) -> Option<User> {
        let users = USERS.lock().unwrap();
        users.to_vec().into_iter().find(|user| user.id == id)
    }

    async fn search_users(&self, input: UserSearchInput) -> Vec<User> {
        let users = USERS.lock().unwrap();
        users
            .to_vec()
            .into_iter()
            .filter(|user| {
                input.name.as_ref().map_or(true, |n| {
                    user.name.to_lowercase().contains(&n.to_lowercase())
                }) && input.email.as_ref().map_or(true, |e| {
                    user.email.to_lowercase().contains(&e.to_lowercase())
                }) && input.age.as_ref().map_or(true, |a| user.age >= *a)
            })
            .collect()
    }
}

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn create_user(&self, input: UserCreateInput) -> User {
        let mut users = USERS.lock().unwrap();
        let id = (users.len() as i32) + 1;
        let user = User {
            id,
            name: input.name,
            email: input.email,
            age: input.age,
        };
        users.push(user.clone());
        user
    }

    async fn update_user(
        &self,
        id: i32,
        name: Option<String>,
        email: Option<String>,
        age: Option<u8>,
    ) -> Result<User, Error> {
        let mut users = USERS.lock().unwrap();
        if let Some(user) = users.iter_mut().find(|user| user.id == id) {
            if let Some(new_name) = name {
                user.name = new_name;
            }
            if let Some(new_email) = email {
                user.email = new_email;
            }
            if let Some(new_age) = age {
                user.age = new_age;
            }
            return Ok(user.clone());
        }

        Err(Error::new("User not found").extend_with(|_, e| e.set("id", id)))
    }

    async fn delete_user(&self, id: i32) -> bool {
        let mut users = USERS.lock().unwrap();
        let original_len = users.len();
        users.retain(|user| user.id != id);
        users.len() < original_len
    }

    async fn reset_users(&self) -> bool {
        let mut users = USERS.lock().unwrap();
        users.clear();
        true
    }
}

pub type MySchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

pub fn create_schema(pool: PgPool) -> MySchema {
    Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(pool.clone())
        .finish()
}
