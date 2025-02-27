use async_graphql::{EmptySubscription, InputObject, Object, Schema, SimpleObject};
use std::sync::Mutex;
use lazy_static::lazy_static;

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
        users.to_vec()
        .into_iter()
        .filter(|user| {
            input.name.as_ref().map_or(true, |n| user.name.to_lowercase().contains(&n.to_lowercase())) &&
            input.email.as_ref().map_or(true, |e| user.email.to_lowercase().contains(&e.to_lowercase())) &&
            input.age.as_ref().map_or(true, |a| user.age >= *a)
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

    async fn update_user(&self, id: i32, name: Option<String>, 
        email: Option<String>, age: Option<u8>) -> Option<User> {
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
            return Some(user.clone());
        }
        None
    }

    async fn delete_user(&self, id: i32) -> bool {
        let mut users = USERS.lock().unwrap();
        let original_len = users.len();
        users.retain(|user| user.id != id);
        users.len() < original_len
    }
}

pub type MySchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

pub fn create_schema() -> MySchema {
    Schema::build(QueryRoot, MutationRoot, EmptySubscription).finish()
}