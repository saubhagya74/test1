
use std::{ collections::HashMap, f32::consts::E, net::SocketAddr, sync::Arc};

use axum::{self, Json, Router, extract::{ConnectInfo, State, WebSocketUpgrade, ws::{Message, Utf8Bytes, WebSocket}}, http::StatusCode, response::IntoResponse, routing::get};
use futures_util::{SinkExt, StreamExt, lock::Mutex};
use serde::de::Error;
use serde_json::{Value, json};
use tokio::{self, sync::{RwLock, mpsc}};
use sqlx::{self, Pool, Postgres};
use argon2;
use snowflake::{self, SnowflakeIdBucket};
use controllers::user_controller;
mod controllers;
mod dbb;
mod wss;
pub struct UserConnection{
    socket_addr: std::net::SocketAddr,
    tx: tokio::sync::mpsc::UnboundedSender<Utf8Bytes>
}

pub type Clients=Arc<RwLock<HashMap<u64,UserConnection>>>;

#[derive(Clone)]
pub struct AppState{
    clients: Clients,
    bucket_id: Arc<Mutex<SnowflakeIdBucket>>,
    db_pool: Pool<Postgres>
}

#[tokio::main]
async fn main(){
    let clients:Clients= Arc::new(RwLock::new(HashMap::new()));
    
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

    let state= AppState{
        clients:clients.clone(),
        bucket_id: bucket_id.clone(),
        db_pool: db_pool.unwrap()
    };
    let myrouter=Router::new()
    .route("/hello", get(||async{"hello".to_string()}))
    .route("/ws", get(wss::ws_handler));
    
    let router=Router::new()
    .merge(myrouter)
    .merge(user_controller::routerfile::give_router())
    .with_state(state);
    

    println!("Server Started"); 
    axum::serve(my_listener, router.into_make_service_with_connect_info::<std::net::SocketAddr>()).await.unwrap();
    
}


