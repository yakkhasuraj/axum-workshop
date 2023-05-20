use axum::{
    http::Method,
    middleware,
    routing::{get, post},
    Extension, Router,
};
use tower_http::cors::{Any, CorsLayer};

use crate::{
    handlers::{
        always_errors, get_json, hard_coded_path, hello_world, middleware_message,
        mirror_body_json, mirror_body_string, mirror_custom_header, mirror_user_agent,
        path_variables, query_params, read_middleware_custom_header, returns_201,
        validate_with_serde,
    },
    middlewares::set_middleware_custom_header,
};

#[derive(Clone)]
pub struct SharedData {
    pub message: String,
}

pub fn create_routes() -> Router {
    let cors = CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_methods([Method::GET, Method::POST])
        // allow requests from any origin
        .allow_origin(Any);

    let shared_data = SharedData {
        message: "Hello from shared date".to_owned(),
    };

    Router::new()
        .route(
            "/read-middleware-custom-header",
            get(read_middleware_custom_header),
        )
        .route_layer(middleware::from_fn(set_middleware_custom_header))
        .route("/", get(hello_world))
        .route("/mirror-string", post(mirror_body_string))
        .route("/mirror-json", post(mirror_body_json))
        .route("/path-variables/15", get(hard_coded_path))
        .route("/path-variables/:id", get(path_variables))
        .route("/query-params", get(query_params))
        .route("/mirror-user-agent", get(mirror_user_agent))
        .route("/mirror-custom-header", get(mirror_custom_header))
        .route("/middleware-message", get(middleware_message))
        .layer(cors)
        .layer(Extension(shared_data))
        .route("/always-error", get(always_errors))
        .route("/returns-201", post(returns_201))
        .route("/get-json", get(get_json))
        .route("/validate-data", post(validate_with_serde))

    // .route("/test", post(create_user))
}
