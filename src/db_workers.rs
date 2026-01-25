use axum::extract::State;
use chrono::DateTime;

use crate::{AppState, controllers::message_controller_ws::messagecontroller::MessagePrivateDB};
use crate::{controllers::message_controller::recent_message_controller::ContentLabel};


pub async fn db_batcher(
    state: AppState,
    mut rx: tokio::sync::mpsc::UnboundedReceiver<MessagePrivateDB> 
){
    while let Some(db_rec) = rx.recv().await {

        let chat_id = if let Some(id) = db_rec.chat_id {
            id as i64
        } else {
            state.bucket_id.lock().await.get_id()
        };//this should be in db insread

        let messaged_at = DateTime::from_timestamp(db_rec.created_at, 0)
            .unwrap_or_else(|| chrono::Utc::now());

        let description_str = std::str::from_utf8(db_rec.description.as_ref())
            .unwrap_or("");//use if let here
        let message_id = state.bucket_id.lock().await.get_id();

        let res = sqlx::query!(
            r#"
            inserT into message_ (
                message_id_, 
                chat_id_, 
                sender_id_, 
                receiver_id_, 
                content_type_, 
                description_, 
                messaged_at_,
                is_edited_,
                is_deleted_
            ) 
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            "#,
            message_id,    
            chat_id,
            db_rec.sender_id as i64,
            db_rec.receiver_id as i64,
            db_rec.content_type as ContentLabel,   
            description_str,            
            messaged_at,         
            false,                   
            false                         
        )
        .execute(&state.db_pool)
        .await;
        println!("{:?}",message_id);
        if let Err(e) = res {
            eprintln!("db insert error: {}", e);
        }
    }
}