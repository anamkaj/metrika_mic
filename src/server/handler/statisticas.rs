use crate::{
    metrika::model::{metrika::statistics::StatGoals, server::post_model::RequestStatatics},
    server::server::AppState,
};
use axum::{extract::State, response::IntoResponse, Json};
use reqwest::StatusCode;
use std::sync::Arc;
use tokio::time::Instant;

//? Звонки и писма */
pub async fn handler_get_statistics(
    State(data): State<Arc<AppState>>,
    Json(body): Json<RequestStatatics>,
) -> impl IntoResponse {
    let start_time_request: Instant = Instant::now();

    let start_time: String = body.date_start.clone();
    let end_time: String = body.date_end.clone();
    let id: i64 = body.id_count;

    let num_str: String = id.abs().to_string();

    if num_str.len() < 5 {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "status": "ok",
                "response_time": format!("{} ms", start_time_request.elapsed().as_millis()),
                "data": "Счетчик не может быть меньше 5 цифр",
            })),
        );
    }

    if start_time.is_empty() || end_time.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "status": "ok",
                "response_time": format!("{} ms", start_time_request.elapsed().as_millis()),
                "data": "Время начала и конца не могут быть пустыми",
            })),
        );
    }

    //*Получение целей счетчика */
    let resp = match StatGoals::get_stat_goals(id, start_time, end_time, data.db.clone()).await {
        Ok(data) => (
            StatusCode::OK,
            Json(serde_json::json!({
                "status": "ok",
                "response_time": format!("{} ms", start_time_request.elapsed().as_millis()),
                "data": data,
            })),
        ),

        Err(err) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "status": "error",
                "response_time": format!("{} ms", start_time_request.elapsed().as_millis()),
                "data": err.to_string(),
            })),
        ),
    };

    resp
}
