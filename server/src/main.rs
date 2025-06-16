use std::net::SocketAddr;
use std::path::PathBuf;

use axum::Router;
use http::header::{AUTHORIZATION, CONTENT_TYPE};
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};

mod controllers;
mod middleware;
mod pool;
mod strategies;

#[tokio::main]
async fn main() {
    if cfg!(debug_assertions) {
        let _ = match dotenvy::dotenv() {
            Ok(path) => {
                println!("Found .env file at {}", path.display());
                path
            }
            Err(error) => {
                println!("Cannot access .env file: {}", error);
                PathBuf::from("")
            }
        };
    }

    pool::create_pool().await;

    let cors = CorsLayer::permissive()
        .allow_origin(Any)
        .allow_headers([AUTHORIZATION, CONTENT_TYPE])
        .expose_headers(Any);

    let app = Router::new()
        .nest("/auth", controllers::auth_controller::routes())
        .nest("/user", controllers::user_controller::routes())
        .nest("/ws", controllers::ws_controller::routes())
        .layer(ServiceBuilder::new().layer(cors));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));
    let listener = TcpListener::bind(addr).await.unwrap();
    match axum::serve(listener, app).await {
        Ok(_) => {
            println!("Server listening on http://{}", addr)
        }
        Err(error) => {
            panic!("Could not bind to http://{}: {}", addr, error)
        }
    }
}
