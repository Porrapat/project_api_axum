use poem::{
    web::{Data, Json, Path},
    Result as PoemResult, Response,
    http::StatusCode,
    handler,
};

use crate::{models::*, AppState};

mod handlers_inner;

impl From<handlers_inner::HandlerError> for poem::Error {
    fn from(error: handlers_inner::HandlerError) -> Self {
        match error {
            handlers_inner::HandlerError::BadRequest(msg) => {
                poem::Error::from_string(msg, StatusCode::BAD_REQUEST)
            }
            handlers_inner::HandlerError::InternalError(msg) => {
                poem::Error::from_string(msg, StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
    }
}

// ---- CRUD for Questions ----

#[handler]
pub async fn create_question(
    Data(app_state): Data<&AppState>,
    Json(question): Json<Question>,
) -> PoemResult<Json<QuestionDetail>> {
    match handlers_inner::create_question(question, app_state.questions_dao.as_ref()).await {
        Ok(question_detail) => Ok(Json(question_detail)),
        Err(error) => Err(error.into()),
    }
}

#[handler]
pub async fn read_questions(
    Data(app_state): Data<&AppState>,
) -> PoemResult<Json<Vec<QuestionDetail>>> {
    match handlers_inner::read_questions(app_state.questions_dao.as_ref()).await {
        Ok(questions) => Ok(Json(questions)),
        Err(error) => Err(error.into()),
    }
}

#[handler]
pub async fn delete_question(
    Data(app_state): Data<&AppState>,
    Json(question_uuid): Json<QuestionId>,
) -> PoemResult<Response> {
    match handlers_inner::delete_question(question_uuid, app_state.questions_dao.as_ref()).await {
        Ok(_) => Ok(Response::builder()
            .status(StatusCode::OK)
            .body(serde_json::json!({"message": "Question deleted successfully"}).to_string())),
        Err(error) => Err(error.into()),
    }
}

// ---- CRUD for Answers ----

#[handler]
pub async fn create_answer(
    Data(app_state): Data<&AppState>,
    Json(answer): Json<Answer>,
) -> PoemResult<Json<AnswerDetail>> {
    match handlers_inner::create_answer(answer, app_state.answers_dao.as_ref()).await {
        Ok(answer_detail) => Ok(Json(answer_detail)),
        Err(error) => Err(error.into()),
    }
}

#[handler]
pub async fn read_answers(
    Data(app_state): Data<&AppState>,
    Path(question_uuid): Path<String>,
) -> PoemResult<Json<Vec<AnswerDetail>>> {
    let question_id = QuestionId { question_uuid };
    
    match handlers_inner::read_answers(question_id, app_state.answers_dao.as_ref()).await {
        Ok(answers) => Ok(Json(answers)),
        Err(error) => Err(error.into()),
    }
}

#[handler]
pub async fn delete_answer(
    Data(app_state): Data<&AppState>,
    Json(answer_uuid): Json<AnswerId>,
) -> PoemResult<Response> {
    match handlers_inner::delete_answer(answer_uuid, app_state.answers_dao.as_ref()).await {
        Ok(_) => Ok(Response::builder()
            .status(StatusCode::OK)
            .body(serde_json::json!({"message": "Answer deleted successfully"}).to_string())),
        Err(error) => Err(error.into()),
    }
}
