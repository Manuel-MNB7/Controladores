use crate::http::request::HttpRequest;
use crate::http::response::HttpResponse;
use crate::models::{AppError, CreateProductDto, UpdateProductDto};
use crate::service::ProductService;
use std::sync::Arc;

pub struct Router {
    service: Arc<ProductService>,
}

impl Router {
    pub fn new(service: Arc<ProductService>) -> Self {
        Self { service }
    }

    pub fn handle_request(&self, req: HttpRequest) -> HttpResponse {
        let path = req.path.as_str();
        let method = req.method.as_str();

        if path == "/products" || path == "/products/" {
            match method {
                "GET" => self.get_all_products(),
                "POST" => self.create_product(&req.body),
                _ => HttpResponse::error_json(405, "Method Not Allowed", "Method not allowed"),
            }
        } else if path.starts_with("/products/") {
            let id_str = &path["/products/".len()..];
            let id = match id_str.parse::<i32>() {
                Ok(val) => val,
                Err(_) => {
                    return HttpResponse::error_json(400, "Bad Request", "Invalid product ID")
                }
            };

            match method {
                "GET" => self.get_product_by_id(id),
                "PUT" => self.update_product(id, &req.body),
                "DELETE" => self.delete_product(id),
                _ => HttpResponse::error_json(405, "Method Not Allowed", "Method not allowed"),
            }
        } else {
            HttpResponse::error_json(404, "Not Found", "Endpoint not found")
        }
    }

    fn get_all_products(&self) -> HttpResponse {
        match self.service.get_all_products() {
            Ok(products) => HttpResponse::ok_json(&products),
            Err(err) => self.map_error(err),
        }
    }

    fn get_product_by_id(&self, id: i32) -> HttpResponse {
        match self.service.get_product_by_id(id) {
            Ok(product) => HttpResponse::ok_json(&product),
            Err(err) => self.map_error(err),
        }
    }

    fn create_product(&self, body: &[u8]) -> HttpResponse {
        let dto: CreateProductDto = match serde_json::from_slice(body) {
            Ok(d) => d,
            Err(e) => return HttpResponse::error_json(400, "Bad Request", &e.to_string()),
        };

        match self.service.create_product(dto) {
            Ok(product) => HttpResponse::created_json(&product),
            Err(err) => self.map_error(err),
        }
    }

    fn update_product(&self, id: i32, body: &[u8]) -> HttpResponse {
        let dto: UpdateProductDto = match serde_json::from_slice(body) {
            Ok(d) => d,
            Err(e) => return HttpResponse::error_json(400, "Bad Request", &e.to_string()),
        };

        match self.service.update_product(id, dto) {
            Ok(product) => HttpResponse::ok_json(&product),
            Err(err) => self.map_error(err),
        }
    }

    fn delete_product(&self, id: i32) -> HttpResponse {
        match self.service.delete_product(id) {
            Ok(_) => HttpResponse::json(200, "OK", &serde_json::json!({ "message": "Product deleted successfully" })),
            Err(err) => self.map_error(err),
        }
    }

    fn map_error(&self, err: AppError) -> HttpResponse {
        match err {
            AppError::NotFound(msg) => HttpResponse::error_json(404, "Not Found", &msg),
            AppError::BadRequest(msg) => HttpResponse::error_json(400, "Bad Request", &msg),
            AppError::DatabaseError(msg) => {
                HttpResponse::error_json(500, "Internal Server Error", &msg)
            }
            AppError::InternalServerError(msg) => {
                HttpResponse::error_json(500, "Internal Server Error", &msg)
            }
        }
    }
}
