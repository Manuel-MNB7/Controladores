use crate::models::{AppError, CreateProductDto, Product, UpdateProductDto};
use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;
use postgres::NoTls;

pub trait ProductRepository: Send + Sync {
    fn create(&self, dto: &CreateProductDto) -> Result<Product, AppError>;
    fn find_all(&self) -> Result<Vec<Product>, AppError>;
    fn find_by_id(&self, id: i32) -> Result<Option<Product>, AppError>;
    fn update(&self, id: i32, dto: &UpdateProductDto) -> Result<Option<Product>, AppError>;
    fn delete(&self, id: i32) -> Result<bool, AppError>;
}

pub struct PgProductRepository {
    pool: Pool<PostgresConnectionManager<NoTls>>,
}

impl PgProductRepository {
    pub fn new(pool: Pool<PostgresConnectionManager<NoTls>>) -> Self {
        Self { pool }
    }
}

impl ProductRepository for PgProductRepository {
    fn create(&self, dto: &CreateProductDto) -> Result<Product, AppError> {
        let mut client = self.pool.get()?;
        let row = client.query_one(
            "INSERT INTO products (name, description, price, stock) VALUES ($1, $2, $3, $4) RETURNING id, name, description, price, stock",
            &[&dto.name, &dto.description, &dto.price, &dto.stock],
        )?;

        Ok(Product {
            id: row.get(0),
            name: row.get(1),
            description: row.get(2),
            price: row.get(3),
            stock: row.get(4),
        })
    }

    fn find_all(&self) -> Result<Vec<Product>, AppError> {
        let mut client = self.pool.get()?;
        let rows = client.query(
            "SELECT id, name, description, price, stock FROM products ORDER BY id ASC",
            &[],
        )?;

        let products = rows
            .iter()
            .map(|row| Product {
                id: row.get(0),
                name: row.get(1),
                description: row.get(2),
                price: row.get(3),
                stock: row.get(4),
            })
            .collect();

        Ok(products)
    }

    fn find_by_id(&self, id: i32) -> Result<Option<Product>, AppError> {
        let mut client = self.pool.get()?;
        let row_opt = client.query_opt(
            "SELECT id, name, description, price, stock FROM products WHERE id = $1",
            &[&id],
        )?;

        Ok(row_opt.map(|row| Product {
            id: row.get(0),
            name: row.get(1),
            description: row.get(2),
            price: row.get(3),
            stock: row.get(4),
        }))
    }

    fn update(&self, id: i32, dto: &UpdateProductDto) -> Result<Option<Product>, AppError> {
        let existing = match self.find_by_id(id)? {
            Some(p) => p,
            None => return Ok(None),
        };

        let new_name = dto.name.as_ref().unwrap_or(&existing.name);
        let new_description = match &dto.description {
            Some(d) => Some(d.clone()),
            None => existing.description,
        };
        let new_price = dto.price.unwrap_or(existing.price);
        let new_stock = dto.stock.unwrap_or(existing.stock);

        let mut client = self.pool.get()?;
        let row = client.query_one(
            "UPDATE products SET name = $1, description = $2, price = $3, stock = $4 WHERE id = $5 RETURNING id, name, description, price, stock",
            &[new_name, &new_description, &new_price, &new_stock, &id],
        )?;

        Ok(Some(Product {
            id: row.get(0),
            name: row.get(1),
            description: row.get(2),
            price: row.get(3),
            stock: row.get(4),
        }))
    }

    fn delete(&self, id: i32) -> Result<bool, AppError> {
        let mut client = self.pool.get()?;
        let count = client.execute("DELETE FROM products WHERE id = $1", &[&id])?;
        Ok(count > 0)
    }
}
