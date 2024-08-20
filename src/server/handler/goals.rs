use crate::{
    metrika::model::{
        db::delete_count::delete_count,
        metrika::{
            goal_client::{Goal, GoalClient},
            goals::GoalCounter,
        },
        server::post_model::{GoalsUpdate, RequestGoals},
    },
    server::server::AppState,
};
use axum::{
    extract::{Query, State},
    response::IntoResponse,
    Json,
};
use reqwest::StatusCode;
use std::{sync::Arc, time::Instant};

//? Получение целей из API Метрики*/
pub async fn handler_goals(
    State(data): State<Arc<AppState>>,
    id_count: Option<Query<RequestGoals>>,
) -> impl IntoResponse {
    let start_time: Instant = Instant::now();

    let id: i64 = id_count.as_ref().unwrap().id_count;
    let num_str: String = id.abs().to_string();
    if num_str.len() < 5 {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "status": "error",
                "response_time": format!("{} ms", start_time.elapsed().as_millis()),
                "data": "Счетчик не может быть меньше 5 цифр",
            })),
        );
    }

    //*Получение целей счетчика */
    let resp = match GoalCounter::api_get_goals(id, data.db.clone()).await {
        Ok(data) => (
            StatusCode::OK,
            Json(serde_json::json!({
                "status": "ok",
                "response_time": format!("{} ms", start_time.elapsed().as_millis()),
                "data": data,
            })),
        ),

        Err(err) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "status": "error",
                "response_time": format!("{} ms", start_time.elapsed().as_millis()),
                "data": err.to_string(),
            })),
        ),
    };

    resp
}

//? Получение всех целей из базы */
pub async fn handler_all_goals(
    State(data): State<Arc<AppState>>,
    id_count: Option<Query<RequestGoals>>,
) -> impl IntoResponse {
    let start_time: Instant = Instant::now();

    let id: i64 = id_count.as_ref().unwrap().id_count;
    let num_str: String = id.abs().to_string();
    if num_str.len() < 4 {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "status": "error",
                "response_time": format!("{} ms", start_time.elapsed().as_millis()),
                "data": "Счетчик не может быть меньше 4 цифр",
            })),
        );
    }

    //*Получение целей счетчика */
    let resp = match GoalClient::get_goals_counter(id, data.db.clone()).await {
        Ok(data) => (
            StatusCode::OK,
            Json(serde_json::json!({
                "status": "ok",
                "response_time": format!("{} ms", start_time.elapsed().as_millis()),
                "data": data,
            })),
        ),

        Err(err) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "status": "error",
                "response_time": format!("{} ms", start_time.elapsed().as_millis()),
                "data": err.to_string(),
            })),
        ),
    };

    resp
}

//? Обновление статуса целей */
pub async fn handler_update_goals(
    State(data): State<Arc<AppState>>,
    Json(body): Json<GoalsUpdate>,
) -> impl IntoResponse {
    let start_time: Instant = Instant::now();

    if body.goals.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "status": "error",
                "response_time": format!("{} ms", start_time.elapsed().as_millis()),
                "data": "Массив целей пуст",
            })),
        );
    }

    //*Получение целей счетчика */
    let resp = match Goal::update_goals_in_db(&body.goals, data.db.clone()).await {
        Ok(_) => (
            StatusCode::OK,
            Json(serde_json::json!({
                "status": "ok",
                "response_time": format!("{} ms", start_time.elapsed().as_millis()),
                "data": "Цели обновлены",
            })),
        ),

        Err(err) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "status": "error",
                "response_time": format!("{} ms", start_time.elapsed().as_millis()),
                "data": err.to_string(),
            })),
        ),
    };

    resp
}

//? Удаление счетчика*/
pub async fn handler_delete_counter(
    State(data): State<Arc<AppState>>,
    id_count: Option<Query<RequestGoals>>,
) -> impl IntoResponse {
    let start_time: Instant = Instant::now();

    let id: i64 = id_count.as_ref().unwrap().id_count;
    let num_str: String = id.abs().to_string();
    if num_str.len() < 4 {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "status": "error",
                "response_time": format!("{} ms", start_time.elapsed().as_millis()),
                "data": "Счетчик не может быть меньше 4 цифр",
            })),
        );
    }

    //*Получение целей счетчика */
    let resp = match delete_count(id, data.db.clone()).await {
        Ok(_) => (
            StatusCode::OK,
            Json(serde_json::json!({
                "status": "ok",
                "response_time": format!("{} ms", start_time.elapsed().as_millis()),
                "data": "Счетчик удален" ,
            })),
        ),

        Err(err) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "status": "error",
                "response_time": format!("{} ms", start_time.elapsed().as_millis()),
                "data": err.to_string(),
            })),
        ),
    };

    resp
}
