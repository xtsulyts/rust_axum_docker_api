use axum::{
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use std::sync::Mutex;
use crate::handlers::user_handler::{get_users, get_user_by_id, create_user};
use crate::models::user::User;

type UserDb = Arc<Mutex<std::collections::HashMap<u32, User>>>;

pub fn users_routes(db: UserDb) -> Router {
    Router::new()
        .route("/", get({
            let db = db.clone();
            move || get_users(db.clone())
        }))
        .route("/", post({
            let db = db.clone();
            move |payload| create_user(payload, db.clone())
        }))
        .route("/{id}", get({
            let db = db.clone();
            move |path| get_user_by_id(path, db.clone())
        }))
}