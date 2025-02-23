use async_graphql::{EmptyMutation, EmptySubscription, InputObject, Object, Schema, SimpleObject};

#[derive(SimpleObject)]
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

fn get_users() -> Vec<User> {
    vec![
        User {
            id: 1,
            name: "Farid".to_string(),
            email: "farid@example.com".to_string(),
            age: 20,
        },
        User {
            id: 2,
            name: "Bob".to_string(),
            email: "bob@example.com".to_string(),
            age: 17,
        }
    ]
}

pub struct QueryRoot;

#[Object]
impl QueryRoot {

    async fn all_users(&self) -> Vec<User> {
        get_users()
    }

    async fn user_by_id(&self, id: i32) -> Option<User> {
        get_users().into_iter().find(|user| user.id == id)
    }

    async fn search_users(&self, input: UserSearchInput) -> Vec<User> {
        get_users()
        .into_iter()
        .filter(|user| {
            input.name.as_ref().map_or(true, |n| user.name.to_lowercase().contains(&n.to_lowercase())) &&
            input.email.as_ref().map_or(true, |e| user.email.to_lowercase().contains(&e.to_lowercase())) &&
            input.age.as_ref().map_or(true, |a| user.age >= *a)
        })
        .collect()
    }
}

pub type MySchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

pub fn create_schema() -> MySchema {
    Schema::build(QueryRoot, EmptyMutation, EmptySubscription).finish()
}