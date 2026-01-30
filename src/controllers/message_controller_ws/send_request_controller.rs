use serde::Deserialize;

use crate::AppState;



pub async fn send_request(
    payload: WSRequestPayload,
    state:AppState,
    user_id: u64
)
{
    println!("payload:{:?}",payload);
    let db_rec= WSRequestDB{
        sender_id: user_id as i64,
        receiver_id: payload.receiver_id as i64,
        status: payload.status as i16,
        created_at : chrono::Utc::now()
    };
    println!("db rec:{:?}",db_rec);
    let res=state.tx_db_wsrequest.
        send(db_rec);
        match res{
            Ok(_)=>{},
            Err(e)=>{
                println!("error;send tx_db_wsrequest{:?}",e);
            }
        }
}

#[derive(Deserialize,Debug)]
pub struct WSRequestPayload{
    pub sender_id:u64,
    pub receiver_id:u64,
    pub status: u16
    //1 sent 2 accep 3 decline
}
#[derive(Debug)]
pub struct WSRequestDB{
    pub sender_id:i64,
    pub receiver_id:i64,
    pub status:i16,
    pub created_at: chrono::DateTime<chrono::Utc>
}