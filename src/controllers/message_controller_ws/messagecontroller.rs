use serde_json::Value;

use crate::Clients;

pub fn send_message_private(payload :Value, my_clients: Clients){

    println!("{:?}",payload);
    // if let Some(uc)=my_clients.read().await.get(&(payload[id])){
    //     uc.tx.send(text);
    // }//put this in ws controllers 

}