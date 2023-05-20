use axum::{
    extract::{Path, Query},
    headers::UserAgent,
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    Extension, Json, TypedHeader,
};
use serde::{Deserialize, Serialize};

use crate::routes::SharedData;

// basic handler that responds with a static string
pub async fn hello_world() -> &'static str {
    "Hello, World!!"
}

pub async fn mirror_body_string(body: String) -> String {
    body
}

#[derive(Deserialize)]
pub struct MirrorJson {
    message: String,
}

#[derive(Serialize)]
pub struct MirrorJsonResponse {
    message: String,
    message_from_server: String,
}

pub async fn mirror_body_json(
    Json(payload): Json<MirrorJson>,
) -> (StatusCode, Json<MirrorJsonResponse>) {
    (
        StatusCode::CREATED,
        Json(MirrorJsonResponse {
            message: payload.message,
            message_from_server: "Hello from Axum".to_owned(),
        }),
    )
}

pub async fn path_variables(Path(id): Path<i32>) -> String {
    id.to_string()
}

pub async fn hard_coded_path() -> String {
    "Fifteen 15".to_owned()
}

#[derive(Deserialize, Serialize)]
pub struct QueryParams {
    message: String,
    id: i32,
}

pub async fn query_params(Query(query): Query<QueryParams>) -> (StatusCode, Json<QueryParams>) {
    (StatusCode::OK, Json(query))
}

pub async fn mirror_user_agent(TypedHeader(user_agent): TypedHeader<UserAgent>) -> String {
    user_agent.to_string()
}

pub async fn mirror_custom_header(headers: HeaderMap) -> String {
    let message_value = headers.get("x-message").unwrap();
    let message = message_value.to_str().unwrap().to_owned();
    message
}

pub async fn middleware_message(Extension(shared_data): Extension<SharedData>) -> String {
    shared_data.message
}

#[derive(Clone)]
pub struct HeaderMessage(pub String);

pub async fn read_middleware_custom_header(Extension(message): Extension<HeaderMessage>) -> String {
    message.0
}

pub async fn always_errors() -> Result<(), StatusCode> {
    // Ok(())
    Err(StatusCode::IM_A_TEAPOT)
}

pub async fn returns_201() -> Response {
    (StatusCode::CREATED, "Created successfully".to_owned()).into_response()
}

#[derive(Serialize)]
pub struct Data {
    message: String,
    count: i32,
    username: String,
}

pub async fn get_json() -> Json<Data> {
    let data = Data {
        count: 100,
        message: "Message".to_owned(),
        username: "username".to_owned(),
    };

    Json(data)
}

#[derive(Serialize, Deserialize)]
pub struct RequestUser {
    username: Option<String>,
    password: String,
}

pub async fn validate_with_serde(Json(user): Json<RequestUser>) -> Json<RequestUser> {
    let data = RequestUser {
        username: user.username,
        password: user.password,
    };

    Json(data)
}
