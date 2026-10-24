use rust_pure_crud_api::http::{HttpRequest, Router};
use rust_pure_crud_api::models::{AppError, CreateProductDto, Product, UpdateProductDto};
use rust_pure_crud_api::repository::ProductRepository;
use rust_pure_crud_api::service::ProductService;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

struct MockProductRepository {
    products: Mutex<Vec<Product>>,
}

impl MockProductRepository {
    fn new() -> Self {
        Self {
            products: Mutex::new(Vec::new()),
        }
    }
}

impl ProductRepository for MockProductRepository {
    fn create(&self, dto: &CreateProductDto) -> Result<Product, AppError> {
        let mut products = self.products.lock().unwrap();
        let id = (products.len() as i32) + 1;
        let product = Product {
            id,
            name: dto.name.clone(),
            description: dto.description.clone(),
            price: dto.price,
            stock: dto.stock,
        };
        products.push(product.clone());
        Ok(product)
    }

    fn find_all(&self) -> Result<Vec<Product>, AppError> {
        let products = self.products.lock().unwrap();
        Ok(products.clone())
    }

    fn find_by_id(&self, id: i32) -> Result<Option<Product>, AppError> {
        let products = self.products.lock().unwrap();
        Ok(products.iter().find(|p| p.id == id).cloned())
    }

    fn update(&self, id: i32, dto: &UpdateProductDto) -> Result<Option<Product>, AppError> {
        let mut products = self.products.lock().unwrap();
        if let Some(product) = products.iter_mut().find(|p| p.id == id) {
            if let Some(ref name) = dto.name {
                product.name = name.clone();
            }
            if let Some(ref desc) = dto.description {
                product.description = Some(desc.clone());
            }
            if let Some(price) = dto.price {
                product.price = price;
            }
            if let Some(stock) = dto.stock {
                product.stock = stock;
            }
            Ok(Some(product.clone()))
        } else {
            Ok(None)
        }
    }

    fn delete(&self, id: i32) -> Result<bool, AppError> {
        let mut products = self.products.lock().unwrap();
        let initial_len = products.len();
        products.retain(|p| p.id != id);
        Ok(products.len() < initial_len)
    }
}

#[test]
fn test_router_http_crud_flow() {
    let mock_repo = Arc::new(MockProductRepository::new());
    let service = Arc::new(ProductService::new(mock_repo));
    let router = Router::new(service);

    // 1. GET /products -> empty array
    let req = HttpRequest {
        method: "GET".to_string(),
        path: "/products".to_string(),
        headers: HashMap::new(),
        body: vec![],
    };
    let resp = router.handle_request(req);
    assert_eq!(resp.status_code, 200);
    assert_eq!(String::from_utf8_lossy(&resp.body), "[]");

    // 2. POST /products -> create product
    let create_payload = serde_json::json!({
        "name": "Mouse",
        "description": "Wireless Mouse",
        "price": 25.99,
        "stock": 50
    })
    .to_string();

    let req = HttpRequest {
        method: "POST".to_string(),
        path: "/products".to_string(),
        headers: HashMap::new(),
        body: create_payload.into_bytes(),
    };
    let resp = router.handle_request(req);
    assert_eq!(resp.status_code, 201);
    let created: Product = serde_json::from_slice(&resp.body).unwrap();
    assert_eq!(created.id, 1);
    assert_eq!(created.name, "Mouse");

    // 3. GET /products/1 -> fetch created product
    let req = HttpRequest {
        method: "GET".to_string(),
        path: "/products/1".to_string(),
        headers: HashMap::new(),
        body: vec![],
    };
    let resp = router.handle_request(req);
    assert_eq!(resp.status_code, 200);
    let fetched: Product = serde_json::from_slice(&resp.body).unwrap();
    assert_eq!(fetched.name, "Mouse");

    // 4. PUT /products/1 -> update product
    let update_payload = serde_json::json!({
        "price": 19.99
    })
    .to_string();

    let req = HttpRequest {
        method: "PUT".to_string(),
        path: "/products/1".to_string(),
        headers: HashMap::new(),
        body: update_payload.into_bytes(),
    };
    let resp = router.handle_request(req);
    assert_eq!(resp.status_code, 200);
    let updated: Product = serde_json::from_slice(&resp.body).unwrap();
    assert_eq!(updated.price, 19.99);

    // 5. DELETE /products/1 -> delete product
    let req = HttpRequest {
        method: "DELETE".to_string(),
        path: "/products/1".to_string(),
        headers: HashMap::new(),
        body: vec![],
    };
    let resp = router.handle_request(req);
    assert_eq!(resp.status_code, 200);

    // 6. GET /products/1 -> should be 404
    let req = HttpRequest {
        method: "GET".to_string(),
        path: "/products/1".to_string(),
        headers: HashMap::new(),
        body: vec![],
    };
    let resp = router.handle_request(req);
    assert_eq!(resp.status_code, 404);
}
