#![allow(unused)]
use std::net::SocketAddr;

use axum::{
    routing::{delete, get, post},
    Router,
    response::Html
};

mod handlers;
mod models;
mod persistance;

use handlers::*;

use dotenvy::dotenv;

use sqlx::postgres::PgPoolOptions;

extern crate pretty_env_logger;
#[macro_use] extern crate log;

use persistance::{
    answers_dao::{AnswersDao, AnswersDaoImpl},
    questions_dao::{QuestionsDao, QuestionsDaoImpl},
};

use models::Question;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    // info!("********* Question Records *********");

    dotenv().ok();
    let my_database_url = &std::env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(my_database_url)
        .await
        .expect("Failed to create Postgres connection pool!");

    // let doa = QuestionsDaoImpl::new(pool);

    // let results = doa.get_questions().await.expect("Error Here!!!!");
    // println!("{:?}", results);

    let a_doa = AnswersDaoImpl::new(pool);

    // let a_results = a_doa.get_answers("82152280-b5ce-44b1-aac4-7c4a2fae8bfe".to_owned()).await.expect("Error Here!!!!");
    // println!("{:?}", a_results);
    // let result = doa
    //     .create_question(Question {
    //         title: "test xxx title".to_owned(),
    //         description: "test xxx description".to_owned(),
    //     })
    //     .await
    //     .map_err(|e| format!("{:?}", e)).expect("Error HERE!!!!!");

    // let recs = sqlx::query!("SELECT * FROM questions")
    //     .fetch_all(&pool)
    //     .await
    //     .unwrap();

    // info!("********* Question Records *********");
    // info!("{:?}", recs);

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