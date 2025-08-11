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

use dotenvy::dotenv;

extern crate pretty_env_logger;
#[macro_use] extern crate log;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    info!("********* Question Records *********");
    // TODO: Initialize dotenv

    dotenv().ok();
    let my_database_url = &std::env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
    info!("********* {my_database_url} *********");

    // Create a new PgPoolOptions instance with a maximum of 5 connections.
    // Use dotenv to get the database url.
    // Use the `unwrap` or `expect` method instead of handling errors. If an
    // error occurs at this stage the server should be terminated.
    // See examples on GitHub page: https://github.com/launchbadge/sqlx
    // let pool = todo!();

    // Using slqx, execute a SQL query that selects all questions from the questions table.
    // Use the `unwrap` or `expect` method to handle errors. This is just some test code to
    // make sure we can connect to the database.
    // let recs = todo!();

    let app = Router::new()
        .route("/", get(handler))
        .route("/mystr", get(handler_mystr))
        .route("/question", post(create_question))
        .route("/questions", get(read_questions))
        .route("/question", delete(delete_question))
        .route("/answer", post(create_answer))
        .route("/answers/{question_uuid}", get(read_answers))
        .route("/answer", delete(delete_answer));

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