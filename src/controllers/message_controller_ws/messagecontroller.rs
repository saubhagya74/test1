use axum::body::Bytes;
use serde::Deserialize;
use serde_json::{Value, value};

use crate::{AppState, Clients, controllers::message_controller::recent_message_controller::ContentLabel};


// #[derive(Deserialize,Debug)]

pub async fn send_message_private<'a>(
    payload :WSMessagePrivatePayload<'a>,
    state:AppState,
    user_id:u64
){//content type see , and return presigned url
    println!("{:?}",payload);
    let receiver_id=payload.receiver_id;
    println!("{:?}",payload.receiver_id);

    let des=Bytes::copy_from_slice(payload.description.get().as_bytes());

    if let Some(uc)=state.clients.read().await.get(&receiver_id){
        //ithink we need to creat time right here
        uc.tx.send(des.clone());
    }
    let db_rec=MessagePrivateDB
    {
        sender_id: user_id,
        receiver_id: payload.receiver_id,
        chat_id: payload.chat_id,
        content_type: payload.content_type,
        description: des.clone(),
        created_at: chrono::Utc::now().timestamp()
    } ;
    
    let _ = state.tx_db_batch_private.send(db_rec);

}

#[derive(Deserialize,Debug)]
pub struct WSMessagePrivatePayload<'a>{
    pub sender_id : u64,
    pub receiver_id : u64,
    pub chat_id : Option<u64>,
    pub content_type: ContentLabel,
    #[serde(borrow)]
    pub description: &'a value::RawValue,
}
pub struct MessagePrivateDB{
   pub sender_id : u64,
   pub receiver_id:u64,
   pub chat_id: Option<u64>,
   pub content_type: ContentLabel,
   pub description: Bytes,
   pub created_at: i64
}
