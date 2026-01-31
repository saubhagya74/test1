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
use controllers::{message_controller, message_controller_ws::{group_creation_controller::{WSCreateGroupDB, WSInitialMemberDB}, messagecontroller::MessagePrivateDB, send_request_controller::{WSAcceptRequest, WSAcceptRequestDB, WSRequestDB, WSRequestPayload}}, user_controller};
use tower_http;
use tower_http::cors::CorsLayer;
mod controllers;
mod db_workers;
mod db_worker2;
mod db_group_worker;
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
    tx_db_ws_request: Arc<tokio::sync::mpsc::UnboundedSender<WSRequestDB>>,
    tx_db_ws_accept_req: Arc<tokio::sync::mpsc::UnboundedSender<WSAcceptRequestDB>>,
    tx_db_ws_create_group: Arc<tokio::sync::mpsc::UnboundedSender<WSCreateGroupDB>>,
    tx_db_ws_initial_member: Arc<tokio::sync::mpsc::UnboundedSender<WSInitialMemberDB>>,
}

//ai says no need arc cuz unbounder sender is already as cheap to clone

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
    let (tx_db_war,rx_db_war)=tokio::sync::mpsc
    ::unbounded_channel::<WSAcceptRequestDB>();
    let (tx_db_wcg,rx_db_wcg)=tokio::sync::mpsc
    ::unbounded_channel::<WSCreateGroupDB>();
        let (tx_db_wim,rx_db_wim)=tokio::sync::mpsc
    ::unbounded_channel::<WSInitialMemberDB>();

    let state= AppState{
        clients:clients.clone(),
        bucket_id: bucket_id.clone(),
        db_pool: db_pool.unwrap(),
        tx_db_batch_private: Arc::new(db_tx_p),
        tx_db_ws_request: Arc::new(tx_db_wr),
        tx_db_ws_accept_req: Arc::new(tx_db_war),
        tx_db_ws_create_group: Arc::new(tx_db_wcg),
        tx_db_ws_initial_member: Arc::new(tx_db_wim)
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
    let s3=state.clone();
    tokio::spawn(async move{
        db_worker2::ws_accept_decline_request(s3,rx_db_war).await;
    });
    let s4=state.clone();
    tokio::spawn(async move{
        db_group_worker::create_group(s4,rx_db_wcg).await;
    });
    let s5=state.clone();
    tokio::spawn(async move{
        db_group_worker::insert_initial_member(s5,rx_db_wim).await;
    });
    
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