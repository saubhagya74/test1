
use std::{ any::type_name_of_val, collections::HashMap, net::SocketAddr, sync::Arc};

use axum::{self, Json, Router, body::Bytes, extract::{ConnectInfo, State, WebSocketUpgrade, ws::{Message, Utf8Bytes, WebSocket}}, http::{HeaderMap, StatusCode}, response::IntoResponse, routing::get};
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, de::Error};
use serde_json::{Value, json, value};
use tokio::{self, runtime::Id, sync::{RwLock,mpsc}};
use sqlx;
use snowflake;
use crate::{AppState, controllers::message_controller_ws::ws_router::decide};
use crate::Clients;
use crate::UserConnection;
use crate::controllers::message_controller_ws;

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
        handle_socket(socket,state,addr,id)
    });
}
pub async fn handle_socket(socket: WebSocket,state:AppState, addr: SocketAddr,id:u64) {

    println!("Client Connected: {:?} with id {}",addr,id);

    let (mut sender, mut receiver) = socket.split();

    let (tx, mut rx) = mpsc
    ::unbounded_channel::<Bytes>();
    
    let conn=UserConnection{
        socket_addr: addr,
        tx
    };

    {
        let mut lock_client=state.clients.write().await;
        lock_client.insert(id, conn);
    }
    // let s_state=state.clients.clone();
    
    let mut send_task = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if sender.send(Message::Binary(msg)).await.is_err(){
                break;
            }
        }
    });
    //there is sender and receiver for each user
    let r_state=state.clone();
    //use dashmap
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            // println!("received: {:?}", msg);
            match msg{
                Message::Binary(bin_data)=>{
                    println!("{:?}",bin_data);
                    if let Ok(ws_msg)=serde_json::from_slice::<WsEnvelope>(&bin_data){
                        // let atoken=ws_msg.token.accesstoken;
                        // let action=ws_msg.action;
                        // println!("{:?}",ws_msg);
                        // println!("{:?}",ws_msg.action);
                        // println!("{:?}",ws_msg.token);
                        // println!("{:?}",atoken);
                        message_controller_ws::ws_router
                        ::decide(ws_msg.action, ws_msg.payload, r_state.clone(), id).await;
                    }
                },
                Message::Text(text)=>{
                    println!("got text, {:?}",text);
                },
                Message::Ping(ping)=>{
                    println!("got ping, {:?}",ping);
                },
                Message::Pong(text)=>{
                    println!("got pong, {:?}",text);
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
#[derive(Deserialize,Debug)]
struct WsEnvelope<'a> {
    #[serde(borrow)]
    token: TokenData<'a>,
    action: &'a str,
    #[serde(borrow)]
    payload: &'a value::RawValue,
}

#[derive(Deserialize,Debug)]
struct TokenData<'a> {
    accesstoken: &'a str,
}//payload lai ni same estai garne