use serde::Deserialize;

use crate::AppState;



pub async fn send_request(
    payload: WSRequestPayload,
    state:AppState,
    user_id: u64
)
{
    
}

#[derive(Deserialize,Debug)]
pub struct WSRequestPayload{
    pub sender_id:u64,
    pub receiver_id:u64,
    pub status: u64
    //1 sent 2 accep 3 decline
}