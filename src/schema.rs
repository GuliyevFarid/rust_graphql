use async_graphql::{EmptyMutation, EmptySubscription, InputObject, Object, Schema, SimpleObject};

#[derive(SimpleObject)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
}

#[derive(InputObject)]
pub struct UserSearchInput {
    pub name: Option<String>,
    pub email: Option<String>,
}

fn get_users() -> Vec<User> {
    vec![
        User {
            id: 1,
            name: "Farid".to_string(),
            email: "farid@example.com".to_string(),
        },
        User {
            id: 2,
            name: "Bob".to_string(),
            email: "bob@example.com".to_string(),
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
            input.name.as_ref().map_or(true, |n| user.name.contains(n)) &&
            input.email.as_ref().map_or(true, |e| user.email.contains(e))
        })
        .collect()
    }
}

pub type MySchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

pub fn create_schema() -> MySchema {
    Schema::build(QueryRoot, EmptyMutation, EmptySubscription).finish()
}