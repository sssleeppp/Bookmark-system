mod db;
mod handlers;
mod models;
mod result;

use axum::extract::DefaultBodyLimit;
use axum::routing::{get, post};
use axum::Router;
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let pool = db::init_db();

    let frontend_dir =
        std::env::var("FRONTEND_DIR").unwrap_or_else(|_| "../frontend/dist".to_string());

    let app = Router::new()
        .route("/user/login", post(handlers::user::login))
        .route("/user/register", post(handlers::user::register))
        .route("/category/list", get(handlers::category::list))
        .route("/category/add", post(handlers::category::add))
        .route(
            "/category/batchUpdate",
            post(handlers::category::batch_update),
        )
        .route("/category/delete", post(handlers::category::delete))
        .route("/bookmark/list", get(handlers::bookmark::list))
        .route("/bookmark/add", post(handlers::bookmark::add))
        .route("/bookmark/delete", post(handlers::bookmark::delete))
        .route(
            "/bookmark/export",
            get(handlers::bookmark::export_bookmarks),
        )
        .route(
            "/bookmark/import",
            post(handlers::bookmark::import_bookmarks),
        )
        .layer(DefaultBodyLimit::max(50 * 1024 * 1024))
        .layer(CorsLayer::permissive())
        .with_state(pool)
        .fallback_service(ServeDir::new(&frontend_dir));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8989")
        .await
        .expect("Failed to bind to port 8989");

    println!("Server running on http://localhost:8989");

    axum::serve(listener, app).await.expect("Server error");
}
