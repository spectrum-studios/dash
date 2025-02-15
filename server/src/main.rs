use std::net::SocketAddr;

use axum::Router;
use http::header::{ AUTHORIZATION, CONTENT_TYPE };
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::cors::{ Any, CorsLayer };

mod pool;

#[tokio::main]
async fn main() {
    if cfg!(debug_assertions) {
        match dotenv::dotenv() {
            Ok(path) => {
                println!("Found .env file at {}", path.display());
            }
            Err(error) => {
                println!("Cannot access .env file: {}", error);
            }
        };
    }

    pool::create_pool().await;

    let cors = CorsLayer::permissive()
        .allow_origin(Any)
        .allow_headers([AUTHORIZATION, CONTENT_TYPE])
        .expose_headers(Any);

    let app = Router::new().layer(ServiceBuilder::new().layer(cors));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));
    let listener = TcpListener::bind(addr).await.unwrap();
    match axum::serve(listener, app).await {
        Ok(_) => { println!("Server listening on http://{}", addr) }
        Err(error) => { panic!("Could not bind to http://{}: {}", addr, error) }
    }
}
