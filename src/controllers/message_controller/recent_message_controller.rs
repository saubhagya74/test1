use axum::{Json, extract::State, http::{HeaderMap, StatusCode}, response::IntoResponse};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::AppState;

// Define exactly what a Message looks like for the JSON response
#[derive(Serialize)]
pub struct MessageResponse {
    pub message_id_: i64,
    pub chat_id_: i64,
    pub sender_id_: i64,
    pub receiver_id_: i64,
    pub content_type_: Option<ContentLabel>, // Matching your DB nullability
    pub description_: Option<String>,
    pub messaged_at_: chrono::DateTime<chrono::Utc>,
    pub compression_type_: Option<CompressionLabel>,
    pub encryption_type_: Option<EncryptionLabel>,
    pub reaction_id_: Option<i16>,
    pub is_edited_: bool,
    pub is_deleted_: bool,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "content_label_")]
pub enum ContentLabel {
    text,
    video,
    audio
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "compression_label_")]
pub enum CompressionLabel {
    lz4,
    gzip,
    zstd,
    none
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "encryption_label_")]
pub enum EncryptionLabel {
    ecc,
    rsa,
    none
}


pub async fn get_recent_messages(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<Value>
) -> Result<impl IntoResponse, StatusCode> {
    
    let user_id = headers.get("id")
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.parse::<i64>().ok())
        .unwrap_or(0);

    if user_id == 0 { return Err(StatusCode::UNAUTHORIZED); }

    let chat_id = payload["chatid"].as_i64()
        .ok_or(StatusCode::BAD_REQUEST)?;
    
    let timestamp_str = payload["timestamp"].as_str()
        .ok_or(StatusCode::BAD_REQUEST)?;
    let timestamp = chrono::DateTime::parse_from_rfc3339(timestamp_str)
        .map_err(|_| StatusCode::BAD_REQUEST)?
        .with_timezone(&chrono::Utc);
    
    let messages = sqlx::query_as!(
    MessageResponse,
    r#"
        select
            m.message_id_, m.chat_id_, m.sender_id_, m.receiver_id_, 
            m.content_type_ as "content_type_: ContentLabel", 
            m.description_, m.messaged_at_, 
            m.compression_type_ as "compression_type_: CompressionLabel",
            m.encryption_type_ as "encryption_type_: EncryptionLabel",
            m.reaction_id_,
            m.is_edited_ as "is_edited_!",
            m.is_deleted_ as "is_deleted_!"
        from message_ m 
        inner join conversation_ c ON m.chat_id_ = c.chat_id_
        where m.chat_id_ = $1
          and (c.user_a_id_ = $2 OR c.user_b_id_ = $2)
          and m.messaged_at_ < $3
          and m.is_deleted_ = false
        order by m.messaged_at_ DESC
        limit $4
    "#,
        chat_id, user_id, timestamp, 10i64
    )
    .fetch_all(&state.db_pool)
    .await
    .map_err(|e| {
        println!("sqlx: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR})?;

    Ok(Json(messages))

}