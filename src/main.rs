mod models;
mod schema;
mod errors;

use actix_web::{web, App, HttpRequest, HttpServer};
use async_graphql::http::GraphiQLSource;
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use schema::{create_schema, MySchema};

use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;

async fn graphql_handler(
    schema: web::Data<MySchema>,
    _req: HttpRequest,
    gql_req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(gql_req.into_inner()).await.into()
}

async fn graphql_playground() -> actix_web::Result<actix_web::HttpResponse> {
    Ok(actix_web::HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(GraphiQLSource::build().endpoint("/graphql").finish()))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db_pool = PgPoolOptions::new()
        .max_connections(10)
        .min_connections(2)
        .idle_timeout(std::time::Duration::from_secs(60))
        .connect(&database_url)
        .await
        .expect("Failed to connect to the database");

    println!("✅ Connected to PostgreSQL!");

    let schema = create_schema(db_pool);

    println!("🚀 GraphQL server running at http://localhost:8080/graphql");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(schema.clone()))
            .route("/graphql", web::post().to(graphql_handler))
            .route("/play", web::get().to(graphql_playground))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
