use serde_json::Value;

use crate::Clients;

use super::messagecontroller;

// println!("indecidefunc {}",payload);
pub fn decide(action: &str, payload: Value,my_clients: Clients) {
    match action {
        
        "sendMessagePrivate" => {
            println!("logic here..");
            messagecontroller::send_message_private(payload,my_clients.clone());
        },
        "login" => {
            println!("logic here..");
        },
        _ => {
            println!("invald action: {}", action);
        }
    }
}