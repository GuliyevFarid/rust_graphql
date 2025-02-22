use async_graphql::{SimpleObject, Object, EmptyMutation, EmptySubscription, Schema};

#[derive(SimpleObject)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
}

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn user(&self) -> User {
        User {
            id: 1,
            name: "Farid".to_string(),
            email: "farid@example.com".to_string(),
        }
    }

    async fn user_by_id(&self, id: i32) -> User {
        if id == 1 {
            User {
                id: 1,
                name: "Farid".to_string(),
                email: "farid@example.com".to_string(),
            }
        }
        else {
            User {
                id: 0,
                name: "".to_string(),
                email: "".to_string()
            }
        }
    }
}

pub type MySchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

pub fn create_schema() -> MySchema {
    Schema::build(QueryRoot, EmptyMutation, EmptySubscription).finish()
}