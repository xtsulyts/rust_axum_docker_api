use axum::{
    extract::Path,
    response::Json,
};
use crate::models::user::User;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

// Base de datos simulada (en memoria)
type UserDb = Arc<Mutex<HashMap<u32, User>>>;

// Handler para GET /users
pub async fn get_users(db: UserDb) -> Json<Vec<User>> {
    let db = db.lock().unwrap();
    let users: Vec<User> = db.values().cloned().collect();
    Json(users)
}

// Handler para GET /users/:id
pub async fn get_user_by_id(
    Path(id): Path<u32>,
    db: UserDb,
) -> Result<Json<User>, (axum::http::StatusCode, String)> {
    let db = db.lock().unwrap();
    
    match db.get(&id) {
        Some(user) => Ok(Json(user.clone())),
        None => Err((axum::http::StatusCode::NOT_FOUND, format!("User {} not found", id))),
    }
}

// Handler para POST /users
pub async fn create_user(
    Json(payload): Json<User>,
    db: UserDb,
) -> Json<User> {
    let mut db = db.lock().unwrap();
    let user = User {
        id: (db.len() + 1) as u32,
        name: payload.name,
        email: payload.email,
    };
    db.insert(user.id, user.clone());
    Json(user)
}

// Función para crear la DB simulada (la usaremos en main.)
pub fn create_db() -> UserDb {
    let mut users = HashMap::new();
    users.insert(1, User {
        id: 1,
        name: "Alice".to_string(),
        email: "alice@example.com".to_string(),
    });
    users.insert(2, User {
        id: 2,
        name: "Bob".to_string(),
        email: "bob@example.com".to_string(),
    });
    Arc::new(Mutex::new(users))
}