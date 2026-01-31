use tokio::time::{Instant, timeout};

use crate::{AppState, controllers::message_controller_ws::group_creation_controller::{WSCreateGroupDB, WSInitialMemberDB}};

pub async fn create_group(
    state: AppState,
    mut rx: tokio::sync::mpsc::UnboundedReceiver<WSCreateGroupDB>
) {
    loop {
        let start = Instant::now();
        let limit = tokio::time::Duration::from_millis(15000);

        let mut group_ids = Vec::new();
        let mut group_names = Vec::new();
        let mut creator_ids = Vec::new();
        let mut created_at = Vec::new();
        let mut initial_members_json = Vec::new();

        while start.elapsed() < limit {
            let time_remaining = limit.saturating_sub(start.elapsed());
            match timeout(time_remaining, rx.recv()).await {
                Ok(Some(db_rec)) => {
                    let g_id = {
                        let mut idbuck = state.bucket_id.lock().await;
                        idbuck.get_id()
                    };

                    let m_json: serde_json::Value = serde_json::from_slice(&db_rec.initial_members)
                        .unwrap_or_else(|_| serde_json::json!([]));

                    group_ids.push(g_id);
                    group_names.push(db_rec.group_name.to_string());
                    creator_ids.push(db_rec.creator_id);
                    created_at.push(db_rec.created_at);
                    initial_members_json.push(m_json);
                }
                Ok(None) => break, //ai says channel closes
                Err(_) => break,
            }
        }
        //do validation of every vec length which should be equal
        if !group_ids.is_empty() {
            let res = sqlx::query!(
                r#"
                with raw_data as (
                    select * from unnest(
                        $1::int8[],       -- g_ids
                        $2::text[],       -- names
                        $3::int8[],       -- creators
                        $4::timestamptz[], -- dates
                        $5::jsonb[]       -- members_json
                    ) AS t (g_id, g_name, c_id, c_at, m_list)
                ),
                insert_conv as (
                    insert into group_conversation_ (group_id_, last_message_,
                        last_time_, created_at_, admin_id_)
                    select g_id, 'Group Created', c_at, c_at, c_id from raw_data
                )
                insert into group_member_ (group_id_, member_id_, joined_at_)
                    select distinct ON (target.g_id, target.m_id) 
                        target.g_id, target.m_id, target.c_at
                from (
                    -- flatten json ids and combinig with admin id
                    select rd.g_id, (jsonb_array_elements_text(rd.m_list)::int8) as m_id,
                    rd.c_at from raw_data rd
                    union all
                    select rd.g_id, rd.c_id, rd.c_at from raw_data rd --creator id ni chaiyo nita, huna ta initial member mai creator add gareni huni but paxi jwt bata garne bela yei thikxa
                ) as target
                inner join users_ u ON u.user_id_ = target.m_id -- validation join
                on conflict (group_id_, member_id_) do nothing;
                "#,
                &group_ids,
                &group_names,
                &creator_ids,
                &created_at,
                &initial_members_json
            ).execute(&state.db_pool).await;

            if let Err(e) = res {
                println!("Error batch inserting groups: {:?}", e);
            }
        }
    }
}
pub async fn insert_initial_member(
    state:AppState,
    mut rx: tokio::sync::mpsc::UnboundedReceiver<WSInitialMemberDB>
){
    loop{
        let start=Instant::now();
        let limit=tokio::time::Duration::from_millis(15000);

        // let mut  group_ids=Vec::new();
        // let mut group_names=Vec::new();
        // let mut creator_ids=Vec::new();
        // let mut created_at=Vec::new();

        while start.elapsed()<limit{
            let time_remaining=limit.saturating_sub(start.elapsed());
            match timeout(time_remaining, rx.recv()).await{
                Ok(Some(db_rec))=>{
                //     group_ids.push(db_rec.group_id);
                //     group_names.push(db_rec.group_name);
                //     creator_ids.push(db_rec.creator_id);
                //     created_at.push(db_rec.created_at)
                },
                Ok(None)=>{
                    println!("why no msg in channel create_group db worker");
                },
                Err(_)=>{
                    break;
                }
            }
        }
        // let res
    }
}