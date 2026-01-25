use axum::body::Bytes;
use serde::Deserialize;
use serde_json::{Value, value};

use crate::{AppState, Clients, controllers::message_controller::recent_message_controller::ContentLabel};


#[derive(Deserialize,Debug)]
pub struct WSMessagePrivate<'a>{
    pub sender_id : u64,
    pub receiver_id : u64,
    pub chat_id : Option<u64>,
    pub content_type: ContentLabel,
    #[serde(borrow)]
    pub description: &'a value::RawValue,
}
// #[derive(Deserialize,Debug)]

pub async fn send_message_private<'a>(
    payload :WSMessagePrivate<'a>,
    state:AppState,
    user_id:u64
){
    println!("{:?}",payload);
    let receiver_id=payload.receiver_id;
    println!("{:?}",payload.receiver_id);

    let des=Bytes::copy_from_slice(payload.description.get().as_bytes());

    if let Some(uc)=state.clients.read().await.get(&receiver_id){
        //ithink we need to creat time right here
        uc.tx.send(des.clone());
    }
    //conversation table update garna lai ni new channel 
    let db_rec=MessagePrivateDB
    {
        sender_id: payload.sender_id,
        receiver_id: payload.receiver_id,
        chat_id: payload.chat_id,
        content_type: payload.content_type,
        description: des.clone(),
        created_at: chrono::Utc::now().timestamp()
    } ;
    let _ = state.tx_db_batch.send(db_rec);
}

pub struct MessagePrivateDB{
   pub sender_id : u64,
   pub receiver_id:u64,
   pub chat_id: Option<u64>,
   pub content_type: ContentLabel,
   pub description: Bytes,
   pub created_at: i64
}