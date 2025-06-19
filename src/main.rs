mod line;
mod pdf;
mod utils;

use axum::{routing::post, Router};
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/callback", post(line::handle_webhook))
        .nest_service("/static", ServeDir::new("static"));

    println!("Server running at http://localhost:3000");
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
