use std::{ thread, time::{self, Duration}};

use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde::Deserialize;
use serde_json::{Value, json};
use snowflake;
use tokio::runtime::Id;
use argon2::{
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier, password_hash::{SaltString, rand_core::OsRng}
};
use crate::AppState;
pub async fn return_user()-> Json<Value>{
    Json(json!(
        {
            "name":"joe",
            "address":"kathmandu",
            "rollno": 234574
        }
    ))
}
#[derive(Deserialize)] 
pub struct CreateUserPayload {
    pub username: String,
    pub email: String,
    pub password: String
}
pub async fn create_user(
    State(state): State<AppState>, 
    Json(payload): Json<CreateUserPayload>
) -> impl IntoResponse {
    
    let new_id = state.bucket_id.lock().await.get_id();
    
    let password_bytes = payload.password.as_bytes();
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    ///creates a string like: $argon2id$v=19$m=4096,t=3,p=1$SALT$HASH
    let password_hash = match argon2.hash_password(password_bytes, &salt) {
        Ok(p) => p.to_string(),
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "error hashing password").into_response(),
    };
    
    let query_result = sqlx::query!(
    "INSERT INTO users_ (user_id_, username_, email_, password_hash_) VALUES ($1, $2, $3,$4)",
    new_id as i64,
    payload.username,
    payload.email,
    password_hash
    )
    .execute(&state.db_pool)
    .await;
    
    match query_result {
        Ok(_) => {
            (axum::http::StatusCode::CREATED, format!("user created with ID: {},username:{}, email {}", new_id,payload.username,payload.email))
            .into_response()
        }
        Err(e) => {
            // println!("database error: {:?}", e);
            (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "failed to create user")
            .into_response()
        }
    }
}
pub async fn login_user(
    State(state): State<AppState>,
    Json(payload): Json<LoginPayload>
) -> impl IntoResponse {

    let row = sqlx::query!("seleCt password_hash_ FROM users_ WHERE email_ = $1", payload.email)
        .fetch_optional(&state.db_pool)
        .await;

    thread::sleep(time::Duration::from_secs(3));

    let db_user = match row {
        Ok(Some(user)) => user,
        Ok(None) => return (StatusCode::UNAUTHORIZED, "invalid email or password").into_response(),
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "database error").into_response(),
    };
    let hash_str = db_user.password_hash_.as_deref().unwrap_or("");
    let parsed_hash = match PasswordHash::new(hash_str) {
        Ok(hash) => hash,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "invalid hahsformat in db").into_response(),
    };

    match Argon2::default().verify_password(payload.password.as_bytes(), &parsed_hash) {
        Ok(_) => (StatusCode::OK,
        Json(json!(
        {
            "token":"afdshdftghdsdvsdfv"
        }))
    ).into_response(),
        Err(_) => (StatusCode::UNAUTHORIZED, "invalid email or password").into_response(),
    }
}
#[derive(serde::Deserialize)]
pub struct LoginPayload {
    pub email: String,
    pub password: String,
}
