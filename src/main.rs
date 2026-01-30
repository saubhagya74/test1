use std::{ collections::HashMap, f32::consts::E, net::SocketAddr, sync::Arc};

use axum::{self, Json, Router, body::Bytes, extract::{ConnectInfo, State, WebSocketUpgrade, ws::{Message, Utf8Bytes, WebSocket}}, http::StatusCode, response::IntoResponse, routing::get};
use futures_util::{SinkExt, StreamExt, lock::Mutex};
use serde::de::Error;
use serde_json::{Value, json};
use tokio::{self, sync::{RwLock, mpsc::{self, UnboundedReceiver}}};
use chrono;
use argon2; 
use sqlx::{self, Pool, Postgres};
use snowflake::{self, SnowflakeIdBucket};
use controllers::{message_controller, message_controller_ws::{messagecontroller::MessagePrivateDB, send_request_controller::{WSRequestDB, WSRequestPayload}}, user_controller};
use tower_http;
use tower_http::cors::CorsLayer;
mod controllers;
mod db_workers;
mod dbb;
mod wss;

pub struct UserConnection{
    socket_addr: std::net::SocketAddr,
    tx: tokio::sync::mpsc::UnboundedSender<Bytes>
}

pub type Clients=Arc<RwLock<HashMap<u64,UserConnection>>>;

#[derive(Clone)]
pub struct AppState{
    clients: Clients,
    bucket_id: Arc<Mutex<SnowflakeIdBucket>>,
    db_pool: Pool<Postgres>,
    tx_db_batch_private: Arc<tokio::sync::mpsc::UnboundedSender<MessagePrivateDB>>,
    tx_db_wsrequest: Arc<tokio::sync::mpsc::UnboundedSender<WSRequestDB>>
}//ai says no need arc cuz unbounder sender is already as cheap to clone

#[tokio::main]
async fn main(){
    let clients:Clients= Arc::new(RwLock::new(HashMap::new()));

    let cors = CorsLayer::new()
    .allow_origin(tower_http::cors::Any)
    .allow_methods([axum::http::Method::POST, axum::http::Method::OPTIONS]) // Add OPTIONS here
    .allow_headers([axum::http::header::CONTENT_TYPE, axum::http::header::HeaderName::from_static("id")]); // Add "id" explicitly
    
    let my_addr="0.0.0.0:6745";
    let my_listener=tokio::net::TcpListener::bind(my_addr).await.unwrap();
    // let mut bucketid=snowflake::SnowflakeIdBucket::new(1,1);
    // let ide=bucketid.get_id()
    let db_pool = match dbb::pgcon().await {
        Ok(pool) => {
            println!("successfully connected to database");
            Some(pool)
        }
        Err(e) => {
            println!("error connecting to db: {:?}", e);
            None
        }
    };
    let mut bucket_id=Arc::new(Mutex::new(
        snowflake::SnowflakeIdBucket::new(1, 1)));
    // let mut rx_hashmap: HashMap<String,UnboundedReceiver<Mess>>
    let (db_tx_p,db_rx_p)=tokio::sync::mpsc
    ::unbounded_channel::<MessagePrivateDB>();
    let (tx_db_wr,rx_db_wr)=tokio::sync::mpsc
    ::unbounded_channel::<WSRequestDB>();
    let state= AppState{
        clients:clients.clone(),
        bucket_id: bucket_id.clone(),
        db_pool: db_pool.unwrap(),
        tx_db_batch_private: Arc::new(db_tx_p),
        tx_db_wsrequest: Arc::new(tx_db_wr)
    };

    // db_batcher_spawner(state.clone());
    let s1=state.clone();
    tokio::spawn(async move{
        db_workers::db_batcher_private(s1, db_rx_p).await;
    });
    let s2=state.clone();
    tokio::spawn(async move{
        db_workers::ws_request_batcher(s2,rx_db_wr).await;
    });

    // tokio::spawn
    
    let myrouter=Router::new()
    .route("/ws", get(wss::ws_handler));
    
    let router=Router::new()
    .merge(myrouter)
    .merge(user_controller::routerfile::get_router())
    .merge(message_controller::routerfile::get_router().await)
    .with_state(state)
    .layer(cors);//fror dev purpose onlyt
    
    

    println!("Server Started"); 
    axum::serve(my_listener, router.into_make_service_with_connect_info::<std::net::SocketAddr>()).await.unwrap();
    
}
// fn db_batcher_spawner(state: AppState){

    

// }