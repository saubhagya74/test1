use axum::body::Bytes;
use serde::Deserialize;

use crate::{AppState, controllers::message_controller::recent_message_controller::ContentLabel};

pub async fn group_message<'a>(
    payload: WSGroupMessagePayload<'a>,
    state:AppState,

){
    let des=Bytes::copy_from_slice(payload.description.get().as_bytes());
    let db_rec=WSGroupMessageDB{
        sender_id: payload.sender_id,
        chat_id: payload.chat_id,
        content_type: payload.content_type,
        description: des
    };
}
#[derive(Deserialize,Debug)]
pub struct WSGroupMessagePayload<'a>{
    pub sender_id: u64,
    pub chat_id: u64,
    pub content_type: ContentLabel,
    #[serde(borrow)]
    pub description: &'a serde_json::value::RawValue
}
#[derive(Debug)]
pub struct WSGroupMessageDB{
    pub sender_id: u64,
    pub chat_id: u64,
    pub content_type: ContentLabel,
    pub description: Bytes
}