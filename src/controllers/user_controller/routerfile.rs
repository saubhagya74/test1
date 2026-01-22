use axum::{Router, routing::{get, post}};

use crate::AppState;

use super::usercontroller;

pub fn give_router()->Router<AppState>{//why return <AppState>
    Router::new()
    .route("/user", get(usercontroller::return_user))
    .route("/createuser", post(usercontroller::create_user))
    .route("/login", post(usercontroller::login_user))
}