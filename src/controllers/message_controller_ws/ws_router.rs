use serde_json::{Value, value};

use crate::{AppState, Clients, controllers::message_controller_ws::messagecontroller::WSMessagePrivatePayload};

use super::{group_creation_controller::{self, WSCreateGroupPayload}, messagecontroller, send_request_controller::{ self, WSAcceptRequest, WSRequestPayload}};

// println!("indecidefunc {}",payload);
pub async fn decide(
    action: &str, 
    raw_payload: &value::RawValue,
    state: AppState,
    user_id :u64
) {
    match action {
        
        "sendMessagePrivate" => {
            // println!("called sendMessagePrivate");
            
            match serde_json::from_str::<WSMessagePrivatePayload>(raw_payload.get()) {
                Ok(payload) => {
                    messagecontroller::send_message_private(payload, state.clone(), user_id).await;
                }
                Err(e) => {
                    println!("parsing failed: {:?}", e);
                }
            }
        },
        "sendRequest" => {
            match serde_json::from_str::<WSRequestPayload>(raw_payload.get()){
                Ok(payload)=>{
                    send_request_controller::send_request(payload,state.clone(),user_id).await;
                },
                Err(e)=>{
                    println!("wsrouter;sendRequest;parsing failed {:?}",e)
                }
            }
        },
        "acceptRequest" => {
            match serde_json::from_str::<WSAcceptRequest>(raw_payload.get()){
                Ok(payload)=>{
                    send_request_controller::accept_request(payload, state.clone(), user_id).await;
                },
                Err(e)=>{
                    println!("wsacceptreqtparsingerror{:?}",e);
                }
            }
        },
        "declineRequest" => {
            println!("logic here..");
        }, 
        "createGroup" => {
            match serde_json::from_str::<WSCreateGroupPayload>(raw_payload.get()){
                Ok(payload)=>{
                    group_creation_controller::create_group(payload, state.clone(), user_id).await;
                },
                Err(e)=>{
                    println!("groupCreatePayloadParsing:{e}");
                }
            }
        },
        "like" => {
            println!("logic here..");
        },
        "comment" => {
            println!("logic here..");
        },
        // "" => {
        //     println!("logic here..");
        // },
        _ => {
            println!("invald action: {}", action);
        }
    }
}