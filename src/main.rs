use axum::{
    routing::{get, post},
    http::StatusCode,
    response::IntoResponse,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use uuid::Uuid;
use uuid::{Builder, Variant, Version};

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new()
        .route("/", get(root))
        .route("/users", post(create_user));

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}

async fn create_user(Json(payload): Json<CreateUser>) -> impl IntoResponse {
    let uu_id = Uuid::new_v4();
    let user = User {
        id: uu_id.to_string(),
        username: payload.username,
        email: payload.email,
        balance: 0,
    };
    StatusCode::OK;
    return Json(user);
}

#[derive(Deserialize)]
struct CreateUser {
    username: String,
    email: String,
}

#[derive(Serialize)]
struct User {
    id: String,
    username: String,
    email: String,
    balance: u64,
}
