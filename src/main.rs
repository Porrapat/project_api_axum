#[macro_use]
extern crate log;

extern crate pretty_env_logger;

use std::sync::Arc;

use poem::{
    listener::TcpListener,
    middleware::Tracing,
    Route, Server, EndpointExt,
};
use dotenvy::dotenv;

use persistance::{
    answers_dao::{AnswersDao, AnswersDaoImpl},
    questions_dao::{QuestionsDao, QuestionsDaoImpl},
};
use sqlx::postgres::PgPoolOptions;

mod handlers;
mod models;
mod persistance;

use handlers::*;

#[derive(Clone)]
pub struct AppState {
    pub questions_dao: Arc<dyn QuestionsDao + Send + Sync>,
    pub answers_dao: Arc<dyn AnswersDao + Send + Sync>,
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    pretty_env_logger::init();
    dotenv().ok();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&std::env::var("DATABASE_URL").expect("DATABASE_URL must be set."))
        .await
        .expect("Failed to create Postgres connection pool!");

    let questions_dao = Arc::new(QuestionsDaoImpl::new(pool.clone()));
    let answers_dao = Arc::new(AnswersDaoImpl::new(pool));

    let app_state = AppState {
        questions_dao,
        answers_dao,
    };

    let app = Route::new()
        .at("/question", poem::post(create_question).delete(delete_question))
        .at("/questions", poem::get(read_questions))
        .at("/answer", poem::post(create_answer).delete(delete_answer))
        .at("/answers/:question_uuid", poem::get(read_answers))
        .with(Tracing)
        .data(app_state);

    Server::new(TcpListener::bind("127.0.0.1:8000"))
        .run(app)
        .await
}
