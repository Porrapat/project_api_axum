#[macro_use]
extern crate log;

extern crate pretty_env_logger;

use std::sync::Arc;

use actix_web::{web, App, HttpServer, middleware::Logger};
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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
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

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .wrap(Logger::default())
            .route("/question", web::post().to(create_question))
            .route("/questions", web::get().to(read_questions))
            .route("/question", web::delete().to(delete_question))
            .route("/answer", web::post().to(create_answer))
            .route("/answers/{question_uuid}", web::get().to(read_answers))
            .route("/answer", web::delete().to(delete_answer))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
