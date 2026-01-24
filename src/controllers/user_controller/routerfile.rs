use axum::{Router, routing::{get, post}};

use crate::AppState;

use super::usercontroller;

pub fn get_router()->Router<AppState>{//why return <AppState>
    Router::new()
    .route("/user", get(usercontroller::return_user))
    .route("/createUser", post(usercontroller::create_user))
    .route("/login", post(usercontroller::login_user))
}