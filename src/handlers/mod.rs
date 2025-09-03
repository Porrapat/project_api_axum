use salvo::prelude::*;

use crate::{models::*, AppState};

mod handlers_inner;

// ---- CRUD for Questions ----

#[handler]
pub async fn create_question(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let question = match req.parse_json::<Question>().await {
        Ok(q) => q,
        Err(_) => {
            res.status_code(StatusCode::BAD_REQUEST);
            res.render(Json(serde_json::json!({"error": "Invalid JSON"})));
            return;
        }
    };
    
    let app_state = depot.get::<AppState>("app_state").unwrap();
    
    match handlers_inner::create_question(question, app_state.questions_dao.as_ref()).await {
        Ok(question_detail) => {
            res.render(Json(question_detail));
        }
        Err(handlers_inner::HandlerError::BadRequest(msg)) => {
            res.status_code(StatusCode::BAD_REQUEST);
            res.render(Json(serde_json::json!({"error": msg})));
        }
        Err(handlers_inner::HandlerError::InternalError(msg)) => {
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
            res.render(Json(serde_json::json!({"error": msg})));
        }
    }
}

#[handler]
pub async fn read_questions(depot: &mut Depot, res: &mut Response) {
    let app_state = depot.get::<AppState>("app_state").unwrap();
    
    match handlers_inner::read_questions(app_state.questions_dao.as_ref()).await {
        Ok(questions) => {
            res.render(Json(questions));
        }
        Err(handlers_inner::HandlerError::BadRequest(msg)) => {
            res.status_code(StatusCode::BAD_REQUEST);
            res.render(Json(serde_json::json!({"error": msg})));
        }
        Err(handlers_inner::HandlerError::InternalError(msg)) => {
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
            res.render(Json(serde_json::json!({"error": msg})));
        }
    }
}

#[handler]
pub async fn delete_question(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let question_uuid = match req.parse_json::<QuestionId>().await {
        Ok(q) => q,
        Err(_) => {
            res.status_code(StatusCode::BAD_REQUEST);
            res.render(Json(serde_json::json!({"error": "Invalid JSON"})));
            return;
        }
    };
    
    let app_state = depot.get::<AppState>("app_state").unwrap();
    
    match handlers_inner::delete_question(question_uuid, app_state.questions_dao.as_ref()).await {
        Ok(_) => {
            res.render(Json(serde_json::json!({"message": "Question deleted successfully"})));
        }
        Err(handlers_inner::HandlerError::BadRequest(msg)) => {
            res.status_code(StatusCode::BAD_REQUEST);
            res.render(Json(serde_json::json!({"error": msg})));
        }
        Err(handlers_inner::HandlerError::InternalError(msg)) => {
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
            res.render(Json(serde_json::json!({"error": msg})));
        }
    }
}

// ---- CRUD for Answers ----

#[handler]
pub async fn create_answer(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let answer = match req.parse_json::<Answer>().await {
        Ok(a) => a,
        Err(_) => {
            res.status_code(StatusCode::BAD_REQUEST);
            res.render(Json(serde_json::json!({"error": "Invalid JSON"})));
            return;
        }
    };
    
    let app_state = depot.get::<AppState>("app_state").unwrap();
    
    match handlers_inner::create_answer(answer, app_state.answers_dao.as_ref()).await {
        Ok(answer_detail) => {
            res.render(Json(answer_detail));
        }
        Err(handlers_inner::HandlerError::BadRequest(msg)) => {
            res.status_code(StatusCode::BAD_REQUEST);
            res.render(Json(serde_json::json!({"error": msg})));
        }
        Err(handlers_inner::HandlerError::InternalError(msg)) => {
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
            res.render(Json(serde_json::json!({"error": msg})));
        }
    }
}

#[handler]
pub async fn read_answers(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let app_state = depot.get::<AppState>("app_state").unwrap();
    
    let question_uuid = req.param::<String>("question_uuid").unwrap_or_default();
    let question_id = QuestionId { question_uuid };
    
    match handlers_inner::read_answers(question_id, app_state.answers_dao.as_ref()).await {
        Ok(answers) => {
            res.render(Json(answers));
        }
        Err(handlers_inner::HandlerError::BadRequest(msg)) => {
            res.status_code(StatusCode::BAD_REQUEST);
            res.render(Json(serde_json::json!({"error": msg})));
        }
        Err(handlers_inner::HandlerError::InternalError(msg)) => {
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
            res.render(Json(serde_json::json!({"error": msg})));
        }
    }
}

#[handler]
pub async fn delete_answer(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let answer_uuid = match req.parse_json::<AnswerId>().await {
        Ok(a) => a,
        Err(_) => {
            res.status_code(StatusCode::BAD_REQUEST);
            res.render(Json(serde_json::json!({"error": "Invalid JSON"})));
            return;
        }
    };
    
    let app_state = depot.get::<AppState>("app_state").unwrap();
    
    match handlers_inner::delete_answer(answer_uuid, app_state.answers_dao.as_ref()).await {
        Ok(_) => {
            res.render(Json(serde_json::json!({"message": "Answer deleted successfully"})));
        }
        Err(handlers_inner::HandlerError::BadRequest(msg)) => {
            res.status_code(StatusCode::BAD_REQUEST);
            res.render(Json(serde_json::json!({"error": msg})));
        }
        Err(handlers_inner::HandlerError::InternalError(msg)) => {
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
            res.render(Json(serde_json::json!({"error": msg})));
        }
    }
}
