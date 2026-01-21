use axum::{Router, routing::get};

use super::usercontroller;

pub fn give_router()->Router{
    Router::new()
    .route("/user", get(usercontroller::return_user))
}