use axum::Json;
use serde_json::{Value, json};

pub async fn return_user()-> Json<Value>{
    Json(json!(
        {
            "name":"saubhagya",
            "address":"kathmandu",
            "rollno": 241642
        }
    ))
}
