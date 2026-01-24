use axum::{Router, http::StatusCode, response::IntoResponse, routing::{get, post}};

use crate::AppState;

pub async fn get_router()-> Router<AppState>{
    Router::new()
    .route("/health1", get(health_check))
    .route("/getRecentTalks", post(super::recent_talk_controller::get_recent_talk))
    .route("/getRecentMessages", post(super::recent_message_controller::get_recent_messages))
}
async fn health_check()-> impl IntoResponse{
    StatusCode::OK
}