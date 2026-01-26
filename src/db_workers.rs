use axum::extract::State;
use axum::http::StatusCode;
use chrono::DateTime;

use crate::{AppState, controllers::message_controller_ws::messagecontroller::MessagePrivateDB};
use crate::{controllers::message_controller::recent_message_controller::ContentLabel};


pub async fn db_batcher_private(
    state: AppState,
    mut rx: tokio::sync::mpsc::UnboundedReceiver<MessagePrivateDB> 
){
    while let Some(db_rec) = rx.recv().await {
        //this is for , when there is in memory hashmap of users
        // let chat_id = if let Some(id) = db_rec.chat_id {
        //     id as i64 //check the id in memory if stored or not then only returbn 
        // } else {
        //     state.bucket_id.lock().await.get_id()
        // };//this should be in db insread
        let newchatid=state.bucket_id.lock().await.get_id();

        let (u_a, u_b) = if db_rec.sender_id < db_rec.receiver_id {
            (db_rec.sender_id as i64, db_rec.receiver_id as i64)
        } else {
            (db_rec.receiver_id as i64, db_rec.sender_id as i64)
        };

        let messaged_at = DateTime::from_timestamp(db_rec.created_at, 0)
            .unwrap_or_else(|| chrono::Utc::now());

        let description_str = std::str::from_utf8(db_rec.description.as_ref())
            .unwrap_or("");//use if let here

        let message_id = state.bucket_id.lock().await.get_id();
        //check whether the query is correct if y have made a a sperate btree for user ab id
        //if we know chat id from memory then do normal insert with cached chatid
        //can use $ for update but used exclued which is better ??
        //  description_str refers to last_message_?? 
        // because on no conflict $ 2 is description and its last_message?
        let res = sqlx::query!(
            r#"
            with convo as (
                insert into conversation_ (chat_id_, last_message_, last_time_, user_a_id_, user_b_id_)
                values ($1, LEFT($2, 30), $3, $4, $5)
                on conflict (user_a_id_, user_b_id_) 
                do update set 
                    last_message_ = excluded.last_message_,
                    last_time_ = excluded.last_time_
                returning chat_id_
            )
            insert into message_ (
                message_id_, chat_id_, sender_id_, receiver_id_, 
                content_type_, description_, messaged_at_
            )
            select $6, chat_id_, $7, $8, $9 as "content_type_: ContentLabel", $2, $3
            from convo
            "#,
            newchatid,
            description_str,
            messaged_at,
            u_a,                
            u_b,                
            message_id,
            db_rec.sender_id as i64,  
            db_rec.receiver_id as i64,
            db_rec.content_type as ContentLabel 
        )
        .execute(&state.db_pool)
        .await
        .map_err(|e| {
            eprintln!("db failed: {}", e);
            // StatusCode::INTERNAL_SERVER_ERROR no return its a task
        });
    }
}
// pub async fn updateConversation(){

// }

// at first check chatid is sent? 
// true do 0
// false do 2

// -0 check for chatid in mem
// do_> 1 if valid do normal insert and normal update
// do 2->else do cte common table expresion