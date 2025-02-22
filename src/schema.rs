use async_graphql::{SimpleObject, Object, EmptyMutation, EmptySubscription, Schema};

#[derive(SimpleObject)]
pub struct User {
    pub id: i32,
    pub name: String,
}

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn user(&self) -> User {
        User {
            id: 1,
            name: "Farid".to_string(),
        }
    }
}

pub type MySchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

pub fn create_schema() -> MySchema {
    Schema::build(QueryRoot, EmptyMutation, EmptySubscription).finish()
}