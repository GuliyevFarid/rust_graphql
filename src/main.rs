mod schema;
use actix_web::{web, App, HttpRequest, HttpServer};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use async_graphql::http::GraphiQLSource;
use schema::{create_schema, MySchema};

async fn graphql_handler(
    schema: web::Data<MySchema>,
    _req: HttpRequest, 
    gql_req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(gql_req.into_inner()).await.into()
}

async fn graphql_playground() -> actix_web::Result<actix_web::HttpResponse> {
    Ok(actix_web::HttpResponse::Ok()
    .content_type("text/html; charset=utf-8")
    .body(GraphiQLSource::build().endpoint("/graphql").finish()))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let schema = create_schema();

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
