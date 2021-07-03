use diesel;
use diesel::PgConnection;
use diesel::prelude::*;
use super::schema::products;
use super::schema::products::dsl::products as all_products;
use serde::Serialize;


#[derive(Queryable, Serialize)]
pub struct Product {
    pub id: i32,
    pub title: String,
    pub description_short: String,
    pub description_long: String,
    pub price: f64,
    pub stock: i32,
    pub brand: String,
    pub color: String,
    pub size: String
}

#[derive(Insertable)]
#[table_name="products"]
pub struct NewProduct {
    pub title: String,
    pub description_short: String,
    pub description_long: String,
    pub price: f64,
    pub stock: i32,
    pub brand: String,
    pub color: String,
    pub size: String
}

impl Product {
    pub fn find_by_id(id: i32, conn: &PgConnection) -> Vec<Product> {
        all_products
            .find(id)
            .load::<Product>(conn)
            .expect("Error loading book")
    }

    pub fn get_product_list(conn: &PgConnection) -> Vec<Product> {
        all_products.load::<Product>(conn)
            .expect("Can't retrieve products")
    }
}
