use serde::{Serialize, Deserialize};

pub type ID = usize;

#[derive(Serialize, Deserialize, Debug)]
pub struct ProductBasic {
    pub id: ID,
    pub title: String,
    pub description_short: String,
    pub price: f64,
    pub stock: i32
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProductDetail {
    pub brand: String,
    pub description_long: String,
    pub size: String,
    pub size_variants: Vec<SizeVariant>,
    pub color: String,
    pub color_variants: Vec<ColorVariant>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SizeVariant {
    pub id: ID,
    pub size: String,
    pub stock: usize,
    pub product_id: ID,
    pub price: f64
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ColorVariant {
    pub id: ID,
    pub color: String,
    pub stock: usize,
    pub product_id: ID,
    pub price: f64
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProductLarge {
    
    #[serde(flatten)]
    pub basic: ProductBasic,

    #[serde(flatten)]
    pub detail: ProductDetail
}

