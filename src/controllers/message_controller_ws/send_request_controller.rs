use serde::Deserialize;

use crate::AppState;

pub async fn send_request(
    payload: WSRequestPayload,
    state:AppState,
    user_id: u64
)
{
    // println!("payload:{:?}",payload);
    let db_rec= WSRequestDB{
        sender_id: user_id as i64,
        receiver_id: payload.receiver_id as i64,
        status: payload.status as i16,
        created_at : chrono::Utc::now()
    };
    println!("db rec:{:?}",db_rec);
    let res=state.tx_db_ws_request.
        send(db_rec);
        match res{
            Ok(_)=>{},
            Err(e)=>{
                println!("error;send tx_db_wsrequest{:?}",e);
            }
        }
}
//almost everything should be cached so that we can decline earlier
pub async fn accept_request(
    payload:WSAcceptRequest,
    state: AppState,
    user_id: u64 
){
    let db_rec=WSAcceptRequestDB{
        request_id:payload.request_id as i64,
        sender_id: payload.sender_id as  i64,
        receiver_id: payload.receiver_id as i64,
        status:payload.state as i16,
        changed_at: chrono::Utc::now()
    };
    let res= state.tx_db_ws_accept_req.send(db_rec);
    match res{
        Ok(_)=>{},
        Err(e)=>{
            println!("failed to send");
        }
    }
}

pub async fn decline_request(){

}
#[derive(Deserialize,Debug)]
pub struct WSAcceptRequest{
    pub request_id: u64,
    pub sender_id: u64,
    pub receiver_id: u64,
    pub state: u16
}
#[derive(Debug)]
pub struct WSAcceptRequestDB{
    pub request_id: i64,
    pub sender_id: i64,
    pub receiver_id: i64,
    pub status: i16,
    pub changed_at: chrono::DateTime<chrono::Utc>
}
#[derive(Deserialize,Debug)]
pub struct WSRequestPayload{
    pub sender_id:u64,
    pub receiver_id:u64,
    pub status: u16
    //1 sent 2 accept 3 decline
}
#[derive(Debug)]
pub struct WSRequestDB{
    pub sender_id:i64,
    pub receiver_id:i64,
    pub status:i16,
    pub created_at: chrono::DateTime<chrono::Utc>
}