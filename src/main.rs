
use std::{ collections::HashMap, net::SocketAddr, sync::{Arc, RwLock}};

use axum::{self, Json, Router, extract::{ConnectInfo, State, ws::{Message, WebSocket}}, http::StatusCode, response::IntoResponse, routing::get};
use controllers::user_controller;
use futures_util::{SinkExt, StreamExt};
use serde::de::Error;
use serde_json::{Value, json};
use tokio::{self, sync::mpsc};
use sqlx;
mod controllers;

struct UserConnection{
    socket_addr: std::net::SocketAddr,
    tx: tokio::sync::mpsc::UnboundedSender<tokio_tungstenite::tungstenite::Message>
}
type Clients=Arc<RwLock<HashMap<u64,UserConnection>>>;

#[derive(Clone)]
pub struct AppState{
    clients: Clients,
}

#[tokio::main]
async fn main(){
    let clients:Clients= Arc::new(RwLock::new(HashMap::new()));
    
    let my_addr="0.0.0.0:6745";
    let my_listener=tokio::net::TcpListener::bind(my_addr).await.unwrap();

    let state= AppState{
        clients:clients.clone(),
    };
    
    let myrouter=Router::new()
    .route("/hello", get(||async{"hello".to_string()}))
    .route("/ws", get(ws_handler))
    .with_state(state);
    
    let router=Router::new()
    .merge(myrouter)
    .merge(user_controller::routerfile::give_router());
    

    pgcon().await;

    println!("Server Started"); 
    axum::serve(my_listener, router.into_make_service_with_connect_info::<std::net::SocketAddr>()).await.unwrap();
    
}
async fn ws_handler(ws: axum::extract::WebSocketUpgrade,ConnectInfo(addr): ConnectInfo<SocketAddr>,)-> impl IntoResponse
{
    println!("Connection attempt from: {}", addr);
    ws.on_upgrade(|socket|handle_socket(socket))
}
async fn handle_socket(socket: WebSocket) {

    let (mut sender, mut receiver) = socket.split();

    let (tx, mut rx) = mpsc::channel::<Message>(32);

    let mut send_task = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if sender.send(msg).await.is_err() {
                break;
            }
        }
    });

    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            println!("Received: {:?}", msg);
            
        }
    });

    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    };
}

async fn pgcon(){

    let db_url = "postgresql://devuser:mypassword@192.168.1.97:5432/channelapi";
    
    let conn_pool =sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(db_url)
        .await;
    match conn_pool{
            Ok(value)=>{println!("connected to database")},
            Err(e)=>{ println!("error connecting database {:?}",e)}
    }
}
