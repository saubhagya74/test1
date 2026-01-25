use serde_json::{Value, value};

use crate::{AppState, Clients, controllers::message_controller_ws::messagecontroller::WSMessagePrivate};

use super::messagecontroller;

// println!("indecidefunc {}",payload);
pub async fn decide(
    action: &str, 
    raw_payload: &value::RawValue,
    state: AppState,
    user_id :u64
) {
    match action {
        
        "sendMessagePrivate" => {
            println!("called sendMessagePrivate");
            
            match serde_json::from_str::<WSMessagePrivate>(raw_payload.get()) {
                Ok(payload) => {
                    messagecontroller::send_message_private(payload, state.clone(), user_id).await;
                }
                Err(e) => {
                    println!("parsing failed: {:?}", e);
                }
            }
        },
        "sendRequest" => {
            println!("logic here..");
        },
        "acceptRequest" => {
            println!("logic here..");
        },
        "declineRequest" => {
            println!("logic here..");
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