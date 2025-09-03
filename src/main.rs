#[macro_use]
extern crate log;

extern crate pretty_env_logger;

use std::sync::Arc;

use salvo::prelude::*;
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

static mut APP_STATE: Option<AppState> = None;

#[handler]
async fn inject_state(req: &mut Request, depot: &mut Depot, res: &mut Response, ctrl: &mut FlowCtrl) {
    unsafe {
        if let Some(ref state) = APP_STATE {
            depot.insert("app_state", state.clone());
        }
    }
    ctrl.call_next(req, depot, res).await;
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

    unsafe {
        APP_STATE = Some(app_state);
    }

    let router = Router::new()
        .hoop(salvo::logging::Logger::new())
        .hoop(inject_state)
        .push(Router::with_path("/question").post(create_question).delete(delete_question))
        .push(Router::with_path("/questions").get(read_questions))
        .push(Router::with_path("/answer").post(create_answer).delete(delete_answer))
        .push(Router::with_path("/answers/<question_uuid>").get(read_answers));

    let acceptor = TcpListener::new("127.0.0.1:8000").bind().await;
    Server::new(acceptor).serve(router).await;
}
