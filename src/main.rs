use actix_web::{web, get, post, App, HttpResponse, HttpServer, Responder};

use std::default::Default;

use rusoto_core::Region;
use rusoto_dynamodb::{DynamoDb, DynamoDbClient, ListTablesInput};

#[get("/tables")]
async fn echo(req_body: String) -> impl Responder {
    // DynamoDB.
    let region = match std::env::var("AWS_REGION") {
        Ok(r) => if String::eq(&r, &String::from("local")) {
            Region::Custom { name: String::from("eu-west-2"), endpoint: String::from("http://localhost:8000") }
        } else {
            Region::default()
        }
        Err(_e) => Region::default()
    };

    let client = DynamoDbClient::new(region);
    let list_tables_input: ListTablesInput = Default::default();

    match client.list_tables(list_tables_input).await {
        Ok(output) => {
            match output.table_names {
                Some(table_name_list) => {
                    match table_name_list.get(0) {
                        Some(table_name) => HttpResponse::Ok().body(table_name),
                        None => HttpResponse::NotFound().finish()
                    }
                },
                None => {
                    println!("No tables in database!");
                    HttpResponse::NotFound().finish()
                },
            }
        },
        Err(error) => {
            println!("Error: {:?}", error);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(echo)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
