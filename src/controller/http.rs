use crate::model::user::{CreateUser, User};
use axum::extract::Path;
use axum::http::StatusCode;
use axum::Json;
use nanoid::nanoid;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Params {
    user_id: String,
}

pub async fn create_user(Json(payload): Json<CreateUser>) -> (StatusCode, Json<User>) {
    let id = nanoid!();

    let user = User {
        id,
        username: payload.username,
    };

    (StatusCode::CREATED, Json(user))
}

pub async fn get_user(Path(path): Path<Params>) -> (StatusCode, Json<User>) {
    let user = User {
        id: path.user_id,
        username: "test".to_string(),
    };

    (StatusCode::OK, Json(user))
}
