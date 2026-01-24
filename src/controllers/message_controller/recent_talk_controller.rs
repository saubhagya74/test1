use axum::{Json, extract::State, http::{HeaderMap, StatusCode}, response::IntoResponse};

use crate::AppState;


#[derive(serde::Serialize)]
pub struct ConversationRow {
pub chat_id_: i64,
pub chat_name_: Option<String>,
pub last_message_: Option<String>,
pub last_time_: Option<chrono::DateTime<chrono::Utc>>,
pub user_a_id_: i64,
pub user_b_id_: i64,
}

pub async fn get_recent_talk(
    State(state):State<AppState>,
    headers: HeaderMap
)->Result<impl IntoResponse, StatusCode>{

    let user_id=headers.get("id")
    .and_then(|value| value.to_str().ok())
    .and_then(|value| value.parse::<u64>().ok())
    .unwrap_or(0);//extract from jwt

    if(user_id==0) {return Err(StatusCode::UNAUTHORIZED);}
    
    let conversation_rows = sqlx::query_as!(
        ConversationRow,
        "select chat_id_, chat_name_, last_message_, last_time_, user_a_id_, user_b_id_ 
        from conversation_ 
        where user_a_id_ = $1 OR user_b_id_ = $1 
        limit $2",
        user_id as i64,
        10i64
    )
    .fetch_all(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(conversation_rows))
}
    