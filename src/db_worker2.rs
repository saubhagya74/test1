use tokio::time::{Instant, timeout};

use crate::{AppState, controllers::message_controller_ws::send_request_controller::WSAcceptRequestDB};



pub async fn ws_accept_decline_request(
    state: AppState,
    mut rx: tokio::sync::mpsc::UnboundedReceiver<WSAcceptRequestDB>
){
    loop {
        let start=Instant::now();
        let limit=tokio::time::Duration::from_millis(15000);

        let mut request_ids=Vec::new();
        let mut sender_ids=Vec::new();
        let mut receiver_ids=Vec::new();
        let mut statues=Vec::new();
        let mut changed_ats=Vec::new();

        while start.elapsed()<limit{
            let time_remaining=limit.saturating_sub(start.elapsed());
            match timeout(time_remaining, rx.recv()).await{
                Ok(value)=>{
                    match value{
                        Some(db_rec)=>{
                            request_ids.push(db_rec.request_id);
                            sender_ids.push(db_rec.sender_id);
                            receiver_ids.push(db_rec.receiver_id);
                            statues.push(db_rec.status);
                            changed_ats.push(db_rec.changed_at);
                        },
                        None=>{
                            println!("no value in dbrec in accept req");
                        }
                    }
                },
                Err(_)=>{
                    break;
                }
            }
        }
        //db batch validate the request also!!!!!!
        //following follower table ma insert khai
        let res=sqlx::query!(
            r#"
                with raw_data as(
                    select * from unnest(
                        $1::int8[], --request id
                        $2::int8[], --sender id
                        $3::int8[], --receiver id
                        $4::int2[], --status
                        $5::timestamptz[] --changed at
                    ) as t (r_id,se_id,re_id,st_id,tim_id)
                )
                update request_
                set timestamp_=rd.tim_id,
                request_status_=rd.st_id
                from raw_data rd
                where request_id_= rd.r_id;
            "#,
            &request_ids,
            &sender_ids,
            &receiver_ids,
            &statues,
            &changed_ats
        ).execute(&state.db_pool).await;
        match res{
            Ok(val)=>{

            },
            Err(e)=>{
                println!("error in ws_batchaccept req:{e}");
            }
        }
    }
}
