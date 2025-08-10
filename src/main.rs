#![allow(unused)]
use std::net::SocketAddr;

use axum::{
    routing::{delete, get, post},
    Router,
    response::Html
};

mod handlers;
mod models;

use handlers::*;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(handler))
        .route("/mystr", get(handler_mystr))
        .route("/question", post(create_question))
        .route("/questions", get(read_questions))
        .route("/question", delete(delete_question));
        // .route("/answer", post(create_answer))
        // .route("/answers", get(read_answers))
        // .route("/answer", delete(delete_answer));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();

    println!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

async fn handler_mystr() -> &'static str {
    "Hello, World! Krub"
}