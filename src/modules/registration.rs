use axum::{
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub async fn create_user(Json(payload): Json<CreateUser>) -> impl IntoResponse {
    let uu_id = Uuid::new_v4();
    let user = User {
        id: uu_id.to_string(),
        username: payload.username,
        email: payload.email,
        balance: 0,
    };

    return (StatusCode::CREATED, Json(user))
}

#[derive(Deserialize)]
pub struct CreateUser {
    username: String,
    email: String,
}

#[derive(Serialize)]
pub struct User {
    id: String,
    username: String,
    email: String,
    balance: u64,
}