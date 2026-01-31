use argon2::Block;
use axum::body::Bytes;
use serde::Deserialize;

use crate::AppState;

pub async fn create_group<'a>(
    payload:WSCreateGroupPayload<'a>,
    state: AppState,
    user_id: u64
){ //validate name, creator
    let initial_members_bytes=Bytes::copy_from_slice(payload.initial_members.
        get().as_bytes());
    let db_rec=WSCreateGroupDB{
        group_name: payload.group_name.into_boxed_str(),
        creator_id: payload.creator_id as i64,
        initial_members: initial_members_bytes,
        created_at: chrono::Utc::now()
    };
    state.tx_db_ws_create_group.send(db_rec);
}
#[derive(Deserialize,Debug)]
pub struct WSCreateGroupPayload<'a>{
    pub group_name: String,
    pub creator_id:u64,
    #[serde(borrow)]
    pub initial_members: &'a serde_json::value::RawValue
}
#[derive(Debug)]
pub struct WSCreateGroupDB{
    pub group_name: Box<str>,
    pub creator_id:i64,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub initial_members: Bytes,
}
#[derive(Debug)]
pub struct WSInitialMemberDB{
    pub group_id: i64,
    pub member_id: i64
}