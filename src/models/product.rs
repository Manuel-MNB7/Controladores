use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub price: f64,
    pub stock: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateProductDto {
    pub name: String,
    pub description: Option<String>,
    pub price: f64,
    pub stock: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateProductDto {
    pub name: Option<String>,
    pub description: Option<String>,
    pub price: Option<f64>,
    pub stock: Option<i32>,
}
