use actix_web::{App, guard, HttpResponse, HttpServer, Result, web};
use async_graphql::{EmptyMutation, Schema};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_actix_web::{Request, Response, WSSubscription};

use graphql::{Query, Subscription, TestSchema};

mod graphql;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let schema = Schema::build(Query, EmptyMutation, Subscription)
        .enable_federation()
        .finish();

    HttpServer::new(move || {
        App::new()
            .data(schema.clone())
            .service(web::resource("/").guard(guard::Post()).to(index))
            .service(web::resource("/").guard(guard::Get()).to(index_playground))
    })
        .bind("127.0.0.1:8010")?
        .run()
        .await
}

pub async fn index(schema: web::Data<TestSchema>, req: Request) -> Response {
    schema.execute(req.into_inner()).await.into()
}

pub async fn index_playground() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(GraphQLPlaygroundConfig::new("/")))
    )
}