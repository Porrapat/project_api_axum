use std::convert::Infallible;
use warp::{Reply, Rejection};

use crate::{models::*, AppState};

mod handlers_inner;

// Custom error handling for Warp
#[derive(Debug)]
pub struct CustomError {
    pub message: String,
    pub status: u16,
}

impl warp::reject::Reject for CustomError {}

impl From<handlers_inner::HandlerError> for CustomError {
    fn from(error: handlers_inner::HandlerError) -> Self {
        match error {
            handlers_inner::HandlerError::BadRequest(msg) => CustomError {
                message: msg,
                status: 400,
            },
            handlers_inner::HandlerError::InternalError(msg) => CustomError {
                message: msg,
                status: 500,
            },
        }
    }
}

// ---- CRUD for Questions ----

pub async fn create_question(
    question: Question,
    app_state: AppState,
) -> Result<impl Reply, Rejection> {
    match handlers_inner::create_question(question, app_state.questions_dao.as_ref()).await {
        Ok(question_detail) => Ok(warp::reply::json(&question_detail)),
        Err(error) => {
            let custom_error: CustomError = error.into();
            Err(warp::reject::custom(custom_error))
        }
    }
}

pub async fn read_questions(app_state: AppState) -> Result<impl Reply, Rejection> {
    match handlers_inner::read_questions(app_state.questions_dao.as_ref()).await {
        Ok(questions) => Ok(warp::reply::json(&questions)),
        Err(error) => {
            let custom_error: CustomError = error.into();
            Err(warp::reject::custom(custom_error))
        }
    }
}

pub async fn delete_question(
    question_uuid: QuestionId,
    app_state: AppState,
) -> Result<impl Reply, Rejection> {
    match handlers_inner::delete_question(question_uuid, app_state.questions_dao.as_ref()).await {
        Ok(_) => Ok(warp::reply::json(&serde_json::json!({"message": "Question deleted successfully"}))),
        Err(error) => {
            let custom_error: CustomError = error.into();
            Err(warp::reject::custom(custom_error))
        }
    }
}

// ---- CRUD for Answers ----

pub async fn create_answer(
    answer: Answer,
    app_state: AppState,
) -> Result<impl Reply, Rejection> {
    match handlers_inner::create_answer(answer, app_state.answers_dao.as_ref()).await {
        Ok(answer_detail) => Ok(warp::reply::json(&answer_detail)),
        Err(error) => {
            let custom_error: CustomError = error.into();
            Err(warp::reject::custom(custom_error))
        }
    }
}

pub async fn read_answers(
    question_uuid: String,
    app_state: AppState,
) -> Result<impl Reply, Rejection> {
    let question_id = QuestionId { question_uuid };
    
    match handlers_inner::read_answers(question_id, app_state.answers_dao.as_ref()).await {
        Ok(answers) => Ok(warp::reply::json(&answers)),
        Err(error) => {
            let custom_error: CustomError = error.into();
            Err(warp::reject::custom(custom_error))
        }
    }
}

pub async fn delete_answer(
    answer_uuid: AnswerId,
    app_state: AppState,
) -> Result<impl Reply, Rejection> {
    match handlers_inner::delete_answer(answer_uuid, app_state.answers_dao.as_ref()).await {
        Ok(_) => Ok(warp::reply::json(&serde_json::json!({"message": "Answer deleted successfully"}))),
        Err(error) => {
            let custom_error: CustomError = error.into();
            Err(warp::reject::custom(custom_error))
        }
    }
}

// Error handling for Warp rejections
pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    if let Some(custom_error) = err.find::<CustomError>() {
        let json = warp::reply::json(&serde_json::json!({
            "error": custom_error.message
        }));
        Ok(warp::reply::with_status(json, warp::http::StatusCode::from_u16(custom_error.status).unwrap()))
    } else if err.is_not_found() {
        let json = warp::reply::json(&serde_json::json!({
            "error": "Not Found"
        }));
        Ok(warp::reply::with_status(json, warp::http::StatusCode::NOT_FOUND))
    } else {
        let json = warp::reply::json(&serde_json::json!({
            "error": "Internal Server Error"
        }));
        Ok(warp::reply::with_status(json, warp::http::StatusCode::INTERNAL_SERVER_ERROR))
    }
}
