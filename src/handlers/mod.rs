use crate::models::*;
use axum::{response::IntoResponse, Json};
use serde_json::json;
// ---- CRUD for Questions ----

pub async fn create_question(Json(question): Json<Question>) -> impl IntoResponse {
    info!("********* create_question *********");

    // let pretty = serde_json::to_string_pretty(&QuestionDetail {
    //     question_uuid: "abced".to_owned(),
    //     title: "My title".to_owned(),
    //     description: "My Description".to_owned(),
    //     created_at: "2025-08-11 12:00:00".to_owned()
    // }).unwrap();

    // (
    //     axum::http::StatusCode::OK,
    //     [(axum::http::header::CONTENT_TYPE, "application/json")],
    //     pretty
    // )

    Json(json!(QuestionDetail {
        question_uuid: "abced".to_owned(),
        title: "My title".to_owned(),
        description: "My Description".to_owned(),
        created_at: "2025-08-11 12:00:00".to_owned()
    }))
}

pub async fn read_questions() -> impl IntoResponse {
    info!("********* read_questions *********");
    Json(json!(vec![
        QuestionDetail {
            question_uuid: "abced".to_owned(),
            title: "My title 1".to_owned(),
            description: "My Description".to_owned(),
            created_at: "2025-08-11 12:00:00".to_owned()
        },
        QuestionDetail {
            question_uuid: "abced".to_owned(),
            title: "My title 2".to_owned(),
            description: "My Description".to_owned(),
            created_at: "2025-08-11 12:00:00".to_owned()
        },
        QuestionDetail {
            question_uuid: "abced".to_owned(),
            title: "My title 3".to_owned(),
            description: "My Description".to_owned(),
            created_at: "2025-08-11 12:00:00".to_owned()
        }
    ]))
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
