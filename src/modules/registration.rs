use axum::{
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use tokio_postgres::{NoTls, Error};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub async fn create_user(Json(payload): Json<CreateUser>) -> impl IntoResponse {

    #[tokio::main]
    async fn connection() -> Result<(), Error> {

        let (client, connection) = tokio_postgres::connect("postgresql://postgres:postgres@localhost:5243/postgres", NoTls).await?;
        Ok(())
    }

    tokio::task::spawn_blocking(|| {connection()}).await.expect("error");

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