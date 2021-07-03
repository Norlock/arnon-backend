use actix_web::{get, post, web, middleware::Logger, App, Result, HttpResponse, HttpServer, Responder};
use actix_cors::Cors;

use models::Product;
use serde::Deserialize;

mod model;
pub mod schema;
pub mod models;

#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

use schema::products::dsl::products as all_products;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[derive(Deserialize)]
struct Id {
    id: i32,
}

#[get("/products")]
async fn get_product_list() -> Result<HttpResponse> {
    let conn = establish_connection();

    let prods = all_products.load::<Product>(&conn)
            .expect("Can't retrieve products");

    Ok(HttpResponse::Ok().json(prods))
}

#[get("/product/{id}")]
async fn get_product(fields: web::Path<Id>) -> Result<HttpResponse> {
    let conn = establish_connection();
    let result  = Product::find_by_id(fields.id, &conn);
    Ok(HttpResponse::Ok().json(result))
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server started at: http://127.0.0.1:8000");

    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(get_product)
            .service(echo)
            .service(get_product_list)
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .wrap(Cors::default()
                  .allowed_origin("http://localhost:8080")
                  .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                  .max_age(3600))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
