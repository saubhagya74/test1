use std::collections::HashMap;

use axum::extract::State;
use axum::http::StatusCode;
use chrono::{DateTime, Duration};
use tokio::sync;
use tokio::time::{Instant, timeout};

use crate::controllers::message_controller_ws::send_request_controller::{WSRequestDB, WSRequestPayload};
use crate::{AppState, controllers::message_controller_ws::messagecontroller::MessagePrivateDB};
use crate::{controllers::message_controller::recent_message_controller::ContentLabel};

// pub struct ForBatch{
//     pub message_id : u64,
//     pub sender_id : u64,
//     pub sender_id : u64,
//     pub sender_id : u64,
//     pub sender_id : u64,
// }
// #[derive(Hash, Eq, PartialEq)]
// pub struct Hkey{
//     pub user_a_id: i64,
//     pub user_b_id: i64
// }
pub async fn db_batcher_private(
    state: AppState,
    mut rx: tokio::sync::mpsc::UnboundedReceiver<MessagePrivateDB> 
){
    // let mut j :i64=1;
    loop {
        // println!("batch no:{}",j);
        let start = Instant::now();
        let limit = tokio::time::Duration::from_millis(15000);
        //key is chatid and value is index no in vec
        // let mut hm=HashMap::<Hkey,u64>::new();
        let mut i:u64=0;
        let mut new_chat_ids = Vec::new();
        let mut message_ids = Vec::new();
        let mut sender_ids = Vec::new();
        let mut receiver_ids = Vec::new();
        let mut user_a_ids = Vec::new();
        let mut user_b_ids = Vec::new();
        let mut content_types = Vec::new();
        let mut descriptions = Vec::new();
        let mut trimmed_descriptions=Vec::new();
        let mut messaged_ats = Vec::new();

        while start.elapsed() < limit {
            let time_remaining = limit.saturating_sub(start.elapsed());
            // println!("time rem:{:?}",time_remaining);
            match timeout(time_remaining, rx.recv()).await {
                Ok(Some(db_rec)) => {

                    let (u_a, u_b) = if db_rec.sender_id < db_rec.receiver_id {
                        (db_rec.sender_id as i64, db_rec.receiver_id as i64)
                    } else {
                        (db_rec.receiver_id as i64, db_rec.sender_id as i64)
                    };
                    //yo time thing pani look more
                    let messaged_at = DateTime::from_timestamp(db_rec.created_at, 0)
                        .unwrap_or_else(|| chrono::Utc::now());

                    let mut id_gen = state.bucket_id.lock().await;
                    let msg_id = id_gen.get_id();
                    let chat_id = id_gen.get_id(); 
                    drop(id_gen);
                    
                    // let hkey=Hkey{
                    //     user_a_id:u_a,
                    //     user_b_id: u_b
                    // };
                    // hm.insert(hkey, i);

                    new_chat_ids.push(chat_id);
                    message_ids.push(msg_id);
                    sender_ids.push(db_rec.sender_id as i64);
                    receiver_ids.push(db_rec.receiver_id as i64);
                    user_a_ids.push(u_a);
                    user_b_ids.push(u_b);
                    content_types.push(db_rec.content_type as ContentLabel);
                    let mut desc=String::from_utf8_lossy(&db_rec.description).into_owned();
                    // let mut tri_desc=String::new();
                    // if desc.len()<=31{
                    //     tri_desc=desc.clone().split_off(30);
                    // }else{
                    //     tri_desc=desc.clone();
                    // } wrong xa ,tala ko gud, but do it in the way tried earlier
                    // let tri_desc: String = desc.chars().take(30).collect();
                    let tri_desc = if desc.len() > 30 {
                        format!("{}...", &desc.chars().take(27).collect::<String>())
                    } else {
                        desc.clone()
                    };
                    // println!("msg:{}in vec",desc);
                    descriptions.push(desc);
                    trimmed_descriptions.push(tri_desc);
                    messaged_ats.push(messaged_at);
                    i=i+1;
                },
                Ok(None) => return, //channel closed, dont do this
                Err(_) => break,    //500ms passed
            }
        }

        // //do limit validation of 2000 in application side
        // let new_convos=sqlx::query!(
        //     r#"SELECT 
        //         r.u_a, 
        //         r.u_b
        //     FROM UNNEST($1::int8[], $2::int8[]) AS r(u_a, u_b)
        //     LEFT JOIN conversation_ c ON c.user_a_id_ = r.u_a AND c.user_b_id_ = r.u_b
        //     WHERE c.chat_id_ IS NULL
        //     "#,
        //     &user_a_ids,
        //     &user_b_ids
        // ).fetch_all(&state.db_pool)
        // .await;
        // Use the "Mega-Query" logic within sqlx
        //spqan in task instead
        let s1=Instant::now();
        if( !message_ids.is_empty()){

            let result = sqlx::query!(
                r#"
            with raw_data as (
                SELECT * FROM UNNEST(
                    $1::int8[], $2::int8[], $3::int8[], $4::int8[], 
                    $5::text[], $6::text[], $7::timestamptz[], $8::int8[], $9::int8[]
                ) AS t(msg_id, s_id, r_id, new_c_id, descrip , tri_desc, m_at, u_a, u_b)
            ),
            upsert_convo as (
                insert into conversation_ (chat_id_, user_a_id_, user_b_id_, last_message_, last_time_)
                select distinct on (u_a, u_b) 
                    new_c_id, u_a, u_b, tri_desc, m_at 
                from raw_data
                on conflict (user_a_id_, user_b_id_) 
                do update set 
                    last_message_ = EXCLUDED.last_message_,
                    last_time_ = EXCLUDED.last_time_
                returning chat_id_, user_a_id_, user_b_id_
            )
            insert into message_ (message_id_, chat_id_, sender_id_, receiver_id_, description_, messaged_at_)
            select 
                r.msg_id, u.chat_id_, r.s_id, r.r_id, r.descrip , r.m_at
            from raw_data r
            join upsert_convo u ON r.u_a = u.user_a_id_ AND r.u_b = u.user_b_id_;
            "#,
            &message_ids,
            &sender_ids, 
            &receiver_ids, 
            &new_chat_ids,
            &descriptions, 
            &trimmed_descriptions,
            &messaged_ats,
            &user_a_ids, 
            &user_b_ids
        ).execute(&state.db_pool).await;

        // println!("batch in db in:{}",s1.elapsed().as_millis());
        }
        else{
            // println!("no msg to insert");
        }
        // j=j+1;
    }
}

pub async fn ws_request_batcher(
    state:AppState,
    mut rx: tokio::sync::mpsc::UnboundedReceiver<WSRequestDB>
){
    let mut j=1;
    loop{
        // println!("batch no:{}",j);
        let start=tokio::time::Instant::now();
        let limit=tokio::time::Duration::from_millis(15000);

        let mut request_ids= Vec::new();
        //can make composite key of request id and userid for uniquness but for now homeid is fine
        // let mut home_ids= Vec::new();
        let mut sender_ids=Vec::new();
        let mut receiver_ids=Vec::new();
        let mut statuses=Vec::new();
        let mut timestamps: Vec<chrono::DateTime<chrono::Utc>> = Vec::new();
        let mut i=1;
        //put a counter to limit rows to be inserted in a batch because rx is way too fast
        while start.elapsed()<limit{
            let time_remaining=limit.saturating_sub(start.elapsed());
            // println!("timerem:{:?}",time_remaining);
            match timeout(time_remaining, rx.recv()).await{
                Ok(value)=>{
                    match value{
                        Some(db_rec)=>{
                            let mut idbuck=state.bucket_id.lock().await;
                            request_ids.push(idbuck.get_id());
                            // home_ids.push(idbuck.get_id());
                            drop(idbuck);
                            sender_ids.push(db_rec.sender_id);
                            receiver_ids.push(db_rec.receiver_id);
                            statuses.push(db_rec.status);
                            timestamps.push(db_rec.created_at);
                        },
                        None=>{
                            println!("why none in wsrewuesrt batcher msg?");
                        }
                    }
                },
                Err(_)=>{
                    break;
                }
            }
            // println!("received request payload no:{}",i);
            i=i+1;
        }
        if(!request_ids.is_empty()){

                //receiver validation?>??
                let s3=Instant::now();
            let res = sqlx::query!(
                r#"
                with raw_data as (
                    select * from unnest(
                        $1::int8[], -- request_id
                        $2::int8[], -- sender_id
                        $3::int8[], -- receiver_id
                        $4::int2[], -- status
                        $5::timestamptz[] -- timestamp
                    ) AS rd (r_id, se_id, re_id, st_id, tim_id)
                ),
                insert_to_requests as (
                    insert into request_ (request_id_, sender_id_, receiver_id_,
                     request_status_, timestamp_)
                    select r_id, se_id, re_id, st_id, tim_id FROM raw_data
                ),
                insert_to_home_sender AS (
                    INSERT INTO home_ (notification_id_, user_id_,
                     notification_object_, created_at_)
                    select r_id, se_id, 
                        jsonb_build_object(
                            'request_', jsonb_build_object(
                                'request_id_', r_id,
                                'sender_id_', se_id,
                                'receiver_id_', re_id,
                                'request_status_', st_id
                            )
                        ), 
                        tim_id 
                    from raw_data
                ) -- no comma
                insert into home_ (notification_id_, user_id_, 
                notification_object_, created_at_)
                select r_id, re_id, 
                    jsonb_build_object(
                        'request_', jsonb_build_object(
                            'request_id_', r_id,
                            'sender_id_', se_id,
                            'receiver_id_', re_id,
                            'request_status_', st_id
                        )
                    ), 
                    tim_id 
                from raw_data;
                "#,
                &request_ids,
                &sender_ids,
                &receiver_ids,
                &statuses,
                &timestamps
            ).execute(&state.db_pool).await;
            match res {
                Ok(info) => {
                    // tracing::info!
                    println!("Batch successful: {} rows modified", info.rows_affected());
                }
                Err(e) => {
                    // tracing::error!
                    println!("Failed to flush batch to DB: {:?}", e);
                }
            }
            println!("req batch finished in :{:?}",s3.elapsed());
        }else{
            // println!("there was no req to batch");
        }
        j=j+1;
    }
}