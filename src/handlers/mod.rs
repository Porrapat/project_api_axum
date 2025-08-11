use crate::models::*;
use axum::{response::IntoResponse, Json};
use serde_json::json;
// ---- CRUD for Questions ----

pub async fn create_question(Json(question): Json<Question>) -> impl IntoResponse {
    info!("********* create_question *********");
    Json(json!({
        "status": "ok 1",
    }))
}

pub async fn read_questions() -> impl IntoResponse {
    info!("********* read_questions *********");
    Json(json!({
        "status": "ok 2",
    }))
}

pub async fn delete_question(Json(question_uuid): Json<QuestionId>) {
    info!("********* delete_question *********");
}

// ---- CRUD for Answers ----
pub async fn create_answer(Json(question): Json<Answer>) -> impl IntoResponse {
    info!("********* create_answer *********");
    Json(json!({
        "status": "ok answer 1",
    }))
}

pub async fn read_answers() -> impl IntoResponse {
    info!("********* read_answers *********");
    Json(json!({
        "status": "ok answer 2",
    }))
}

pub async fn delete_answer(Json(answer_uuid): Json<AnswerId>) {
    info!("********* delete_answer *********");
}
