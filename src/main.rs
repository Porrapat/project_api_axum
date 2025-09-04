#[macro_use]
extern crate log;

extern crate pretty_env_logger;

use std::sync::Arc;
use std::convert::Infallible;

use warp::Filter;
use dotenvy::dotenv;

use persistance::{
    answers_dao::{AnswersDao, AnswersDaoImpl},
    questions_dao::{QuestionsDao, QuestionsDaoImpl},
};
use sqlx::postgres::PgPoolOptions;

mod handlers;
mod models;
mod persistance;

use handlers::{handle_rejection, *};

#[derive(Clone)]
pub struct AppState {
    pub questions_dao: Arc<dyn QuestionsDao + Send + Sync>,
    pub answers_dao: Arc<dyn AnswersDao + Send + Sync>,
}

fn with_app_state(
    app_state: AppState,
) -> impl Filter<Extract = (AppState,), Error = Infallible> + Clone {
    warp::any().map(move || app_state.clone())
}

#[tokio::main]
async fn main() {
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

    // Question routes
    let create_question = warp::path("question")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_app_state(app_state.clone()))
        .and_then(create_question);

    let read_questions = warp::path("questions")
        .and(warp::get())
        .and(with_app_state(app_state.clone()))
        .and_then(read_questions);

    let delete_question = warp::path("question")
        .and(warp::delete())
        .and(warp::body::json())
        .and(with_app_state(app_state.clone()))
        .and_then(delete_question);

    // Answer routes
    let create_answer = warp::path("answer")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_app_state(app_state.clone()))
        .and_then(create_answer);

    let read_answers = warp::path("answers")
        .and(warp::path::param::<String>())
        .and(warp::get())
        .and(with_app_state(app_state.clone()))
        .and_then(read_answers);

    let delete_answer = warp::path("answer")
        .and(warp::delete())
        .and(warp::body::json())
        .and(with_app_state(app_state.clone()))
        .and_then(delete_answer);

    let routes = create_question
        .or(read_questions)
        .or(delete_question)
        .or(create_answer)
        .or(read_answers)
        .or(delete_answer)
        .with(warp::log("api"))
        .recover(handle_rejection);

    println!("Starting server on 127.0.0.1:8000");
    warp::serve(routes)
        .run(([127, 0, 0, 1], 8000))
        .await;
}
