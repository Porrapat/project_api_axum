use crate::models::*;
use axum::{response::IntoResponse, Json};
use serde_json::json;
// ---- CRUD for Questions ----

pub async fn create_question(Json(question): Json<Question>) -> impl IntoResponse {
    // todo!()
    Json(json!({
        "status": "ok 1",
        // "received": question
    }))
}

pub async fn read_questions() -> impl IntoResponse {
    // todo!()
    Json(json!({
        "status": "ok 2",
        // "received": question
    }))
}

pub async fn delete_question(Json(question_uuid): Json<QuestionId>) {
    // todo!()
    // Json(json!({
    //     "status": "ok 3",
    //     // "received": question
    // }))
    // Json()
}

// ---- CRUD for Answers ----

// TODO: Create a POST route to /answer which accepts an `Answer` and returns `AnswerDetail` as JSON.
//       The handler function should be called `create_answer`.
//
//       hint: this function should look very similar to the create_question function above

// TODO: Create a GET route to /answers which accepts an `QuestionId` and returns a vector of `AnswerDetail` as JSON.
//       The handler function should be called `read_answers`.
//
//       hint: this function should look very similar to the read_questions function above

// TODO: Create a DELETE route to /answer which accepts an `AnswerId` and does not return anything.
//       The handler function should be called `delete_answer`.
//
//       hint: this function should look very similar to the delete_question function above
