use diesel;
use diesel::PgConnection;
use diesel::prelude::*;
use super::schema::products;
use super::schema::products::dsl::products as all_products;


#[derive(Queryable)]
pub struct Product<'a> {
    pub id: i32,
    pub title: &'a str,
    pub description_short: &'a str,
    pub description_long: &'a str,
    pub price: f64,
    pub stock: i32,
    pub brand: &'a str,
    pub color: &'a str,
    pub size: &'a str
}

#[derive(Insertable)]
#[table_name="products"]
pub struct NewProduct<'a> {
    pub title: &'a str,
    pub description_short: &'a str,
    pub description_long: &'a str,
    pub price: f64,
    pub stock: i32,
    pub brand: &'a str,
    pub color: &'a str,
    pub size: &'a str
}

//impl Product {
    //pub fn find_by_id(id: i32, conn: &PgConnection) -> Vec<Product> {
        //all_products
            //.find(id)
            //.load::<Product>(conn)
            //.expect("Error loading book")
    //}
//}
