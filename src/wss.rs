
use std::{ any::type_name_of_val, collections::HashMap, net::SocketAddr, sync::Arc};

use axum::{self, Json, Router, extract::{ConnectInfo, State, WebSocketUpgrade, ws::{Message, Utf8Bytes, WebSocket}}, http::{HeaderMap, StatusCode}, response::IntoResponse, routing::get};
use futures_util::{SinkExt, StreamExt};
use serde::de::Error;
use serde_json::{Value, json};
use tokio::{self, sync::{RwLock,mpsc}};
use sqlx;
use snowflake;
use crate::AppState;
use crate::Clients;
use crate::UserConnection;
use crate::controllers::message_controller;

pub async fn ws_handler(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    State(state): State<AppState>,
    headers: HeaderMap,
    ws:WebSocketUpgrade
)-> impl IntoResponse
{
    let id=headers.get("id")
    .and_then(|value| value.to_str().ok())
    .and_then(|value| value.parse::<u64>().ok())
    .unwrap_or_else(||{
        0
    });// id from jwt

    if id==0 {return StatusCode::BAD_REQUEST.into_response();}

    return ws.on_upgrade(move |socket|{
        handle_socket(socket,state.clients,addr,id)
    });
}
pub async fn handle_socket(socket: WebSocket,clients: Clients, addr: SocketAddr,id:u64) {

    println!("Client Connected: {:?} with id {}",addr,id);

    let (mut sender, mut receiver) = socket.split();

    let (tx, mut rx) = mpsc::unbounded_channel::<Utf8Bytes>();
    
    let conn=UserConnection{
        socket_addr: addr,
        tx
    };

    {
        let mut lock_client=clients.write().await;
        lock_client.insert(id, conn);
    }
    let clients_s=clients.clone();
    
    let mut send_task = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if sender.send(Message::Text(msg)).await.is_err() {
                break;
            }
        }
    });
    //there is sender and receiver for each user
    let clients_r=clients.clone();
    //use dashmap
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            // println!("received: {:?}", msg);
            match msg{
                Message::Text(text)=>{
                    println!("got text, {:?}",text);
                    if let Ok(mut ws_msg) = serde_json::from_str::<serde_json::Value>(&text) {
                        
                        let token = ws_msg["token"]["accesstoken"].as_str();
                        let action = ws_msg["action"].as_str();
                        let receiver_id = ws_msg["id"].as_u64();
                        //we check all , lock frees then move , and unnecss things are left behind to die
                        if let (Some(t), Some(act), Some(rid)) = (token, action, receiver_id) {
                            
                            let action_owned = act.to_string(); 
                            
                            let payload = ws_msg["payload"].take();//take is copy of original pointer
                            // let b=["id"].as_u64().unwrap_or(0);
                            println!("send to:, {:?}",rid);
                            // let shared_msg = Arc::new(msg);//check the size first if string is small its better to send direclty
                            if let Some(uc)=clients_r.read().await.get(&rid){
                                uc.tx.send(text);
                            }//put this in ws controllers 
                            message_controller::ws_router::decide(&action_owned, payload);
                            drop(ws_msg); 
                            // drop(text);
                        }
                    }
                        // println!("{}",a);//check this if invalid disconnect
                        // println!("{}",a["auth_token"]);//check this if invalid disconnect
                        // println!("token{}",access_token);//verify accesstoken

                                // println!("enterned in action");
                                
                                // let c=ws_msg["payload"];
                               
                            // let c=a["payload"].as_str().unwrap().as_bytes().to_vec();
                            // let shared_msg = Arc::new(msg);
                            //check the size first if string is small its better to send direclty
                },
                Message::Ping(ping)=>{
                    println!("got ping, {:?}",ping);
                },
                Message::Pong(text)=>{
                    println!("got pong, {:?}",text);
                },
                Message::Binary(text)=>{
                    println!("got binary, {:?}",text);
                },
                Message::Close(close)=>{
                    println!("got closed, {:?}",Some(close));//some wtf??unwrap
                },
                _ => {}
            }
            
        }
    });

    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    };
}
