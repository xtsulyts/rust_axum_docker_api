use axum::Router;
use std::net::SocketAddr;
use tokio::net::TcpListener;

mod models;
mod handlers;
mod routes;

use crate::handlers::user_handler::create_db;
use crate::routes::users::users_routes;

#[tokio::main]
async fn main() {
    // Crear "base de datos" compartida
    let db = create_db();

    // Construir el router con todas las rutas
    let app = Router::new()
        .nest("/users", users_routes(db.clone()));

    // También mantenemos la ruta raíz del hello world
    let app = app.route("/", axum::routing::get(|| async { "¡Hola, mundo desde Axum!" }));

    // Iniciar servidor
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 Servidor corriendo en http://{}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}