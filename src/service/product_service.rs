use crate::models::{AppError, CreateProductDto, Product, UpdateProductDto};
use crate::repository::ProductRepository;
use std::sync::Arc;

pub struct ProductService {
    repo: Arc<dyn ProductRepository>,
}

impl ProductService {
    pub fn new(repo: Arc<dyn ProductRepository>) -> Self {
        Self { repo }
    }

    pub fn create_product(&self, dto: CreateProductDto) -> Result<Product, AppError> {
        if dto.name.trim().is_empty() {
            return Err(AppError::BadRequest("Product name cannot be empty".to_string()));
        }
        if dto.price < 0.0 {
            return Err(AppError::BadRequest("Product price cannot be negative".to_string()));
        }
        if dto.stock < 0 {
            return Err(AppError::BadRequest("Product stock cannot be negative".to_string()));
        }

        self.repo.create(&dto)
    }

    pub fn get_all_products(&self) -> Result<Vec<Product>, AppError> {
        self.repo.find_all()
    }

    pub fn get_product_by_id(&self, id: i32) -> Result<Product, AppError> {
        self.repo
            .find_by_id(id)?
            .ok_or_else(|| AppError::NotFound(format!("Product with id {} not found", id)))
    }

    pub fn update_product(&self, id: i32, dto: UpdateProductDto) -> Result<Product, AppError> {
        if let Some(ref name) = dto.name {
            if name.trim().is_empty() {
                return Err(AppError::BadRequest("Product name cannot be empty".to_string()));
            }
        }
        if let Some(price) = dto.price {
            if price < 0.0 {
                return Err(AppError::BadRequest("Product price cannot be negative".to_string()));
            }
        }
        if let Some(stock) = dto.stock {
            if stock < 0 {
                return Err(AppError::BadRequest("Product stock cannot be negative".to_string()));
            }
        }

        self.repo
            .update(id, &dto)?
            .ok_or_else(|| AppError::NotFound(format!("Product with id {} not found", id)))
    }

    pub fn delete_product(&self, id: i32) -> Result<(), AppError> {
        let deleted = self.repo.delete(id)?;
        if deleted {
            Ok(())
        } else {
            Err(AppError::NotFound(format!("Product with id {} not found", id)))
        }
    }
}
