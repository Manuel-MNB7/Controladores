use rust_pure_crud_api::http::{HttpRequest, HttpResponse, Router};
use rust_pure_crud_api::repository::PgProductRepository;
use rust_pure_crud_api::service::ProductService;
use postgres::NoTls;
use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;
use std::env;
use std::net::TcpListener;
use std::sync::Arc;
use std::thread;

fn main() {
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/store_db".to_string());
    let server_addr = env::var("SERVER_ADDR").unwrap_or_else(|_| "0.0.0.0:8080".to_string());

    println!("Connecting to database: {}", database_url);
    let manager = PostgresConnectionManager::new(
        database_url.parse().expect("Invalid DATABASE_URL"),
        NoTls,
    );

    let pool = match Pool::builder().max_size(10).build(manager) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Failed to create connection pool: {}", e);
            std::process::exit(1);
        }
    };

    let repo = Arc::new(PgProductRepository::new(pool));
    let service = Arc::new(ProductService::new(repo));
    let router = Arc::new(Router::new(service));

    let listener = match TcpListener::bind(&server_addr) {
        Ok(l) => l,
        Err(e) => {
            eprintln!("Failed to bind to address {}: {}", server_addr, e);
            std::process::exit(1);
        }
    };

    println!("Server running at http://{}", server_addr);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let router_clone = Arc::clone(&router);
                thread::spawn(move || match HttpRequest::parse(&stream) {
                    Ok(req) => {
                        let resp = router_clone.handle_request(req);
                        let _ = resp.send(&stream);
                    }
                    Err(e) => {
                        let resp = HttpResponse::error_json(400, "Bad Request", &e);
                        let _ = resp.send(&stream);
                    }
                });
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }
}
