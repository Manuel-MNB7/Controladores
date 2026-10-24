use rust_pure_crud_api::models::{AppError, CreateProductDto, Product, UpdateProductDto};
use rust_pure_crud_api::repository::ProductRepository;
use rust_pure_crud_api::service::ProductService;
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
fn test_create_and_get_product_service() {
    let mock_repo = Arc::new(MockProductRepository::new());
    let service = ProductService::new(mock_repo);

    let create_dto = CreateProductDto {
        name: "Laptop".to_string(),
        description: Some("Gaming Laptop".to_string()),
        price: 1200.50,
        stock: 10,
    };

    let created = service.create_product(create_dto).unwrap();
    assert_eq!(created.id, 1);
    assert_eq!(created.name, "Laptop");

    let fetched = service.get_product_by_id(1).unwrap();
    assert_eq!(fetched.name, "Laptop");
}

#[test]
fn test_service_validation() {
    let mock_repo = Arc::new(MockProductRepository::new());
    let service = ProductService::new(mock_repo);

    let invalid_dto = CreateProductDto {
        name: "   ".to_string(),
        description: None,
        price: -10.0,
        stock: 5,
    };

    let result = service.create_product(invalid_dto);
    assert!(result.is_err());
}
