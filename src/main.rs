
use std::{ collections::HashMap, sync::{Arc, RwLock}};

use axum::{self, Json, Router, extract::{State, ws::{Message, WebSocket}}, http::StatusCode, response::IntoResponse, routing::get};
use controllers::user_controller;
use futures_util::{SinkExt, StreamExt};
use serde::de::Error;
use serde_json::{Value, json};
use tokio::{self, sync::mpsc};
use sqlx;
mod controllers;

// struct ws_user{
//     user_id: u64,
//     socket_addr: std::net::SocketAddr
// }
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
async fn ws_handler(ws: axum::extract::WebSocketUpgrade)-> impl IntoResponse
{
    ws.on_upgrade(|socket|handle_socket(socket))
}
// async fn handle_socket(mut socket: WebSocket){
//     // let (mut ws_sender,mut ws_receiver)=socket.split();
//     while let Some(Ok(msg))=socket.recv().await
//     {
//         match msg{
//             Message::Text(text)=>{
//                 println!("got message, {:?}",text);
//             },
//             Message::Ping(text)=>{
//                 println!("got message, {:?}",text);
//             },
//             Message::Binary(text)=>{
//                 println!("got message, {:?}",text);
//             },
//             Message::Close(text)=>{
//                 println!("got message, {:?}",text);
//             },
//             Message::Pong(text)=>{
//                 println!("got message, {:?}",text);
//             },
//             _=> {}
            
//         }
//     }
// }
async fn handle_socket(socket: WebSocket) {
    // 2. Split the socket into Sender (tx) and Receiver (rx)
    let (mut sender, mut receiver) = socket.split();

    // 3. Create an MPSC channel so other parts of your app can send messages TO this socket
    let (tx, mut rx) = mpsc::channel::<Message>(32);

    // 4. Spin Task: Forwarder (Internal Channel -> WebSocket)
    // This is your "TX Task"
    let mut send_task = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if sender.send(msg).await.is_err() {
                break; // Connection closed
            }
        }
    });

    // 5. Spin Task: Listener (WebSocket -> Logic)
    // This is your "RX Task"
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            // Process the message (e.g., print it or send it to a broadcast channel)
            println!("Received: {:?}", msg);
            
        }
    });

    // 6. Wait for one to finish, then abort the other
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
        .await
        .expect("failed to connect");
}
