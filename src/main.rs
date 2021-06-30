use actix_web::{get, post, web, middleware::Logger, App, Result, HttpResponse, HttpServer, Responder};
use actix_cors::Cors;

use serde::Deserialize;

mod model;
pub mod schema;
pub mod models;

use model::product::*;

#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

use crate::models::NewProduct;

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
    let mut product_list: Vec<ProductBasic> = Vec::new();

    let basic = ProductBasic {
        id: 1,
        title: String::from("Shoe"),
        description_short: String::from("Good shoes"),
        price: 20.65,
        stock: 3
    };

    let basic2 = ProductBasic {
        id: 1,
        title: String::from("Hats"),
        description_short: String::from("Good hats"),
        price: 20.65,
        stock: 3
    };

    let basic3 = ProductBasic {
        id: 1,
        title: String::from("Dress"),
        description_short: String::from("Nice looking dress"),
        price: 20.65,
        stock: 3
    };

    product_list.push(basic);
    product_list.push(basic2);
    product_list.push(basic3);

    Ok(HttpResponse::Ok().json(product_list))
}

#[get("/product/{id}")]
async fn get_product(input: web::Path<Id>) -> Result<HttpResponse> {

    println!("test {}", input.id);
    let basic = ProductBasic {
        id: 1,
        title: String::from("Shoe"),
        description_short: String::from("Good shoes"),
        price: 20.65,
        stock: 3
    };

    let mut color_variants: Vec<ColorVariant> = Vec::new();
    color_variants.push(ColorVariant {
        id: 1,
        color: "#00FF00".to_string(),
        product_id: 999,
        price: 21.65,
        stock: 2
    });

    color_variants.push(ColorVariant {
        id: 1,
        color: "#0000FF".to_string(),
        product_id: 999,
        price: 21.65,
        stock: 2
    });

    let mut size_variants: Vec<SizeVariant> = Vec::new();
    size_variants.push(SizeVariant {
        id: 1,
        size: "large".to_string(),
        stock: 1,
        price: 20.65,
        product_id: 950
    });

    size_variants.push(SizeVariant {
        id: 1,
        size: "medium".to_string(),
        stock: 1,
        price: 20.65,
        product_id: 949
    });

    let prod = ProductLarge {
        basic,
        detail: ProductDetail {
            brand: String::from("Nike"),
            size: String::from("46"),
            description_long: String::from("juustem lorem ipsum lorem ipsum lorem ipsum lorem ipsum"),
            color: "#FF0000".to_string(),
            color_variants,
            size_variants
        }
    };

    Ok(HttpResponse::Ok().json(prod))
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

fn insertDummyData() {
    use schema::products;
    let connection = establish_connection();

    let new_product = NewProduct {
        title: "Shoes",
        description_short: "very good looking shoes",
        description_long: "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Phasellus vulputate lacus a nibh finibus volutpat ut ut lectus. Quisque massa ante, bibendum id efficitur vel, placerat at augue. Etiam dignissim enim non ligula tincidunt, ac bibendum quam condimentum. Nulla maximus lacus eget eros varius, vitae auctor nibh placerat. Sed in ipsum massa. Suspendisse potenti. Proin pharetra bibendum consequat. Fusce feugiat dolor at scelerisque malesuada. Sed at mi non lacus facilisis dapibus. Integer in placerat libero, eu dictum purus. Ut vehicula, magna id egestas varius, nibh libero porttitor justo, ac congue lorem erat ornare dolor.",
        color: "#0000FF",
        size: "46",
        brand: "Adidas",
        price: 85.95,
        stock: 100
    };

    diesel::insert_into(products::table)
        .values(&new_product)
        .get_result(&connection)
        .expect("Error saving new post")
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
