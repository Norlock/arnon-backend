mod main;

use crate::models::NewProduct;

fn create_product(conn: &PgConnection, product: &NewProduct) -> Product {
    use schema::products;

    diesel::insert_into(products::table)
        .values(product)
        .get_result(conn)
        .expect("Error saving new post")
}

fn insert_dummy_data() {
    use schema::products;

    let conn = establish_connection();
    
    diesel::delete(products::table)
        .execute(&conn)
        .expect("Cannot delete table");

    let shoe = NewProduct {
        title: String::from("Shoes"),
        description_short: String::from("very good looking shoes"),
        description_long: String::from("Lorem ipsum dolor sit amet, consectetur adipiscing elit. Phasellus vulputate lacus a nibh finibus volutpat ut ut lectus. Quisque massa ante, bibendum id efficitur vel, placerat at augue. Etiam dignissim enim non ligula tincidunt, ac bibendum quam condimentum. Nulla maximus lacus eget eros varius, vitae auctor nibh placerat. Sed in ipsum massa. Suspendisse potenti. Proin pharetra bibendum consequat. Fusce feugiat dolor at scelerisque malesuada. Sed at mi non lacus facilisis dapibus. Integer in placerat libero, eu dictum purus. Ut vehicula, magna id egestas varius, nibh libero porttitor justo, ac congue lorem erat ornare dolor."),
        color: String::from("#0000FF"),
        size: String::from("46"),
        brand: String::from("Adidas"),
        price: 89.95,
        stock: 100
    };

    let dress = NewProduct {
        title: String::from("Dress"),
        description_short: String::from("Some dress to wear"),
        description_long: String::from("Lorem ipsum dolor sit amet :O, consectetur adipiscing elit. Phasellus vulputate lacus a nibh finibus volutpat ut ut lectus. Quisque massa ante, bibendum id efficitur vel, placerat at augue. Etiam dignissim enim non ligula tincidunt, ac bibendum quam condimentum. Nulla maximus lacus eget eros varius, vitae auctor nibh placerat. Sed in ipsum massa. Suspendisse potenti. Proin pharetra bibendum consequat. Fusce feugiat dolor at scelerisque malesuada. Sed at mi non lacus facilisis dapibus. Integer in placerat libero, eu dictum purus. Ut vehicula, magna id egestas varius, nibh libero porttitor justo, ac congue lorem erat ornare dolor."),
        color: String::from("#00FF00"),
        size: String::from("regular"),
        brand: String::from("Versace"),
        price: 85.95,
        stock: 29
    };
    
    let hat = NewProduct {
        title: String::from("Hat"),
        description_short: String::from("For on your head"),
        description_long: String::from("One of the first pictorial depictions of a hat appears in a tomb painting from Thebes, Egypt, which shows a man wearing a conical straw hat, dated to around 3200 BC. Hats were commonly worn in ancient Egypt. Many upper-class Egyptians shaved their heads, then covered it in a headdress intended to help them keep cool."),
        color: String::from("#000000"),
        size: String::from("regular"),
        brand: String::from("Prada"),
        price: 12.95,
        stock: 50
    };

    let cape = NewProduct {
        title: String::from("Cape"),
        description_short: String::from("From Capetown"),
        description_long: String::from("Looks shiny and nice and glows in the dark"),
        color: String::from("#123456"),
        size: String::from("long"),
        brand: String::from("Cape inc."),
        price: 15.00,
        stock: 1
    };

    create_product(&conn, &shoe);
    create_product(&conn, &hat);
    create_product(&conn, &dress);
    create_product(&conn, &cape);
}
