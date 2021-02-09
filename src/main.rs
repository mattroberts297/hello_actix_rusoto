use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use std::default::Default;
use rusoto_core::Region;
use rusoto_dynamodb::{DynamoDb, DynamoDbClient, ListTablesInput};

#[get("/tables")]
async fn tables(_req_body: String) -> impl Responder {
    let client = DynamoDbClient::new(Region::default());
    let list_tables_input: ListTablesInput = Default::default();
    match client.list_tables(list_tables_input).await {
        Ok(_output) => HttpResponse::Ok().finish(),
        Err(_error) => HttpResponse::InternalServerError().finish(),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| { App::new().service(tables) })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
