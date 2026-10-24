pub mod http;
pub mod models;
pub mod repository;
pub mod service;

pub use http::{HttpRequest, HttpResponse, Router};
pub use models::{AppError, CreateProductDto, Product, UpdateProductDto};
pub use repository::{PgProductRepository, ProductRepository};
pub use service::ProductService;
