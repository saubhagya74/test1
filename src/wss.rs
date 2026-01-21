
use std::{ any::type_name_of_val, collections::HashMap, net::SocketAddr, sync::Arc};

use axum::{self, Json, Router, extract::{ConnectInfo, State, WebSocketUpgrade, ws::{Message, Utf8Bytes, WebSocket}}, http::{HeaderMap, StatusCode}, response::IntoResponse, routing::get};
use futures_util::{SinkExt, StreamExt};
use serde::de::Error;
use serde_json::{Value, json};
use tokio::{self, sync::{RwLock,mpsc}};
use sqlx;

use crate::AppState;
use crate::Clients;
use crate::UserConnection;

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

    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            // println!("received: {:?}", msg);
            match msg{
                Message::Text(text)=>{
                    println!("got text, {:?}",text);
                    let mut a:serde_json::Value=serde_json::from_str(&text).unwrap_or_default();
                    println!("{}",a["auth_token"]);
                    println!("{}",a["action"]);
                    println!("{}",a["payload"]);
                    // let shared_msg = Arc::new(msg);//check the size first if string is small its better to send direclty
                    if let Some(uc)=clients_r.read().await.get(&id){
                        uc.tx.send(text);
                    }
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
