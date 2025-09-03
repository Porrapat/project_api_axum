use actix_web::{web, HttpResponse, Result as ActixResult};

use crate::{models::*, AppState};

mod handlers_inner;

impl From<handlers_inner::HandlerError> for HttpResponse {
    fn from(error: handlers_inner::HandlerError) -> Self {
        match error {
            handlers_inner::HandlerError::BadRequest(msg) => {
                HttpResponse::BadRequest().json(serde_json::json!({"error": msg}))
            }
            handlers_inner::HandlerError::InternalError(msg) => {
                HttpResponse::InternalServerError().json(serde_json::json!({"error": msg}))
            }
        }
    }
}

// ---- CRUD for Questions ----

pub async fn create_question(
    app_state: web::Data<AppState>,
    question: web::Json<Question>,
) -> ActixResult<HttpResponse> {
    match handlers_inner::create_question(question.into_inner(), app_state.questions_dao.as_ref()).await {
        Ok(question_detail) => Ok(HttpResponse::Ok().json(question_detail)),
        Err(error) => Ok(error.into()),
    }
}

pub async fn read_questions(
    app_state: web::Data<AppState>,
) -> ActixResult<HttpResponse> {
    match handlers_inner::read_questions(app_state.questions_dao.as_ref()).await {
        Ok(questions) => Ok(HttpResponse::Ok().json(questions)),
        Err(error) => Ok(error.into()),
    }
}

pub async fn delete_question(
    app_state: web::Data<AppState>,
    question_uuid: web::Json<QuestionId>,
) -> ActixResult<HttpResponse> {
    match handlers_inner::delete_question(question_uuid.into_inner(), app_state.questions_dao.as_ref()).await {
        Ok(_) => Ok(HttpResponse::Ok().json(serde_json::json!({"message": "Question deleted successfully"}))),
        Err(error) => Ok(error.into()),
    }
}

// ---- CRUD for Answers ----

pub async fn create_answer(
    app_state: web::Data<AppState>,
    answer: web::Json<Answer>,
) -> ActixResult<HttpResponse> {
    match handlers_inner::create_answer(answer.into_inner(), app_state.answers_dao.as_ref()).await {
        Ok(answer_detail) => Ok(HttpResponse::Ok().json(answer_detail)),
        Err(error) => Ok(error.into()),
    }
}

pub async fn read_answers(
    app_state: web::Data<AppState>,
    path: web::Path<String>,
) -> ActixResult<HttpResponse> {
    let question_uuid = path.into_inner();
    let question_id = QuestionId { question_uuid };
    
    match handlers_inner::read_answers(question_id, app_state.answers_dao.as_ref()).await {
        Ok(answers) => Ok(HttpResponse::Ok().json(answers)),
        Err(error) => Ok(error.into()),
    }
}

pub async fn delete_answer(
    app_state: web::Data<AppState>,
    answer_uuid: web::Json<AnswerId>,
) -> ActixResult<HttpResponse> {
    match handlers_inner::delete_answer(answer_uuid.into_inner(), app_state.answers_dao.as_ref()).await {
        Ok(_) => Ok(HttpResponse::Ok().json(serde_json::json!({"message": "Answer deleted successfully"}))),
        Err(error) => Ok(error.into()),
    }
}
