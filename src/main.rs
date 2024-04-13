use axum::{Extension, Router};
use axum::routing::{get, post};
use rusqlite::Error;
use crate::controller::{get_content_by_token, store_content};
use crate::db_service::DBService;

mod model;
mod controller;
mod db_service;
#[tokio::main]
async fn main() -> Result<(), Error> {
    // PasteBin using RUST - SQLite
    let service = DBService::new().await?;

    let app = Router::new()
        .route("/content", post(store_content))
        .route("/content/:token",get(get_content_by_token))
        .layer(Extension(service));

    let listener =
        tokio::net::TcpListener::bind("0.0.0.0:3000")
            .await
            .unwrap();

    println!("Listening...!");

    axum::serve(listener, app)
        .await
        .unwrap();
    Ok(())
}
