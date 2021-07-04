use crate::models::NewProduct;
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

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

#[get("/")]
async fn default() -> impl Responder {
    HttpResponse::Ok().body("Arnon backend")
}

#[derive(Deserialize)]
struct Id {
    id: i32,
}

#[get("/products")]
async fn get_product_list() -> Result<HttpResponse> {
    let conn = establish_connection();

    Ok(HttpResponse::Ok().json(Product::get_product_list(&conn)))
}

#[get("/product/{id}")]
async fn get_product(fields: web::Path<Id>) -> Result<HttpResponse> {
    let conn = establish_connection();
    let result  = Product::find_by_id(fields.id, &conn);

    Ok(HttpResponse::Ok().json(result))
}

#[post("/product")]
async fn post_product(new_product: web::Json<NewProduct>) -> Result<HttpResponse> {
    let conn = establish_connection();
    let product = Product::insert_product(&conn, &new_product);

    Ok(HttpResponse::Ok().json(product))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server started at: http://127.0.0.1:8000");

    HttpServer::new(|| {
        App::new()
            .service(default)
            .service(get_product)
            .service(post_product)
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
