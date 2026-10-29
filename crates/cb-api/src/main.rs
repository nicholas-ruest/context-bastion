use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use cb_errors::BastionError;
use cb_orchestrator::Orchestrator;
use cb_types::{AddEventRequest, CreateSessionRequest, ExecuteRequest, SessionView};
use std::sync::{Arc, Mutex};
use tower_http::cors::{Any, CorsLayer};

type Shared = Arc<Mutex<Orchestrator>>;

#[derive(Debug)]
struct ApiError(BastionError);

impl From<BastionError> for ApiError {
    fn from(error: BastionError) -> Self {
        Self(error)
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status = match self.0 {
            BastionError::Validation(_) => StatusCode::UNPROCESSABLE_ENTITY,
            BastionError::Authorization(_) => StatusCode::UNAUTHORIZED,
            BastionError::NotFound(_) => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };
        (
            status,
            Json(serde_json::json!({
                "code": status.as_u16(),
                "message": self.0.to_string()
            })),
        )
            .into_response()
    }
}

fn auth(headers: &HeaderMap) -> Result<String, ApiError> {
    let value = headers
        .get("authorization")
        .and_then(|value| value.to_str().ok());
    let incoming = headers
        .get("x-correlation-id")
        .and_then(|value| value.to_str().ok());
    Ok(cb_middleware::context(value, incoming)?.correlation_id)
}

async fn health() -> Json<serde_json::Value> {
    Json(serde_json::json!({"status":"ok","service":"context-bastion"}))
}

async fn stop_for_test(headers: HeaderMap) -> Result<StatusCode, ApiError> {
    auth(&headers)?;
    if std::env::var("BASTION_TEST_MODE").ok().as_deref() != Some("1") {
        return Err(ApiError(BastionError::Authorization(
            "test restart endpoint disabled".into(),
        )));
    }
    tokio::spawn(async {
        tokio::time::sleep(std::time::Duration::from_millis(120)).await;
        std::process::exit(0);
    });
    Ok(StatusCode::ACCEPTED)
}

async fn create(
    State(state): State<Shared>,
    headers: HeaderMap,
    Json(input): Json<CreateSessionRequest>,
) -> Result<(HeaderMap, Json<SessionView>), ApiError> {
    respond(auth(&headers)?, state.lock().expect("lock").create(input)?)
}

async fn event(
    Path(id): Path<String>,
    State(state): State<Shared>,
    headers: HeaderMap,
    Json(input): Json<AddEventRequest>,
) -> Result<(HeaderMap, Json<SessionView>), ApiError> {
    respond(
        auth(&headers)?,
        state.lock().expect("lock").add_event(&id, input)?,
    )
}

async fn compact(
    Path(id): Path<String>,
    State(state): State<Shared>,
    headers: HeaderMap,
) -> Result<(HeaderMap, Json<SessionView>), ApiError> {
    respond(auth(&headers)?, state.lock().expect("lock").compact(&id)?)
}

async fn execute(
    Path(id): Path<String>,
    State(state): State<Shared>,
    headers: HeaderMap,
    Json(input): Json<ExecuteRequest>,
) -> Result<(HeaderMap, Json<SessionView>), ApiError> {
    respond(
        auth(&headers)?,
        state.lock().expect("lock").execute(&id, input)?,
    )
}

async fn get_one(
    Path(id): Path<String>,
    State(state): State<Shared>,
    headers: HeaderMap,
) -> Result<(HeaderMap, Json<SessionView>), ApiError> {
    respond(auth(&headers)?, state.lock().expect("lock").get(&id)?)
}

fn respond(
    correlation_id: String,
    view: SessionView,
) -> Result<(HeaderMap, Json<SessionView>), ApiError> {
    let mut headers = HeaderMap::new();
    headers.insert(
        "x-correlation-id",
        correlation_id.parse().expect("valid header"),
    );
    Ok((headers, Json(view)))
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().json().init();
    let config = cb_config::Config::from_env().expect("configuration");
    let state = Arc::new(Mutex::new(
        Orchestrator::new(&config.data_dir).expect("orchestrator"),
    ));
    let app = Router::new()
        .route("/healthz", get(health))
        .route("/readyz", get(health))
        .route("/api/v1/admin/shutdown", post(stop_for_test))
        .route("/api/v1/sessions", post(create))
        .route("/api/v1/sessions/{id}", get(get_one))
        .route("/api/v1/sessions/{id}/events", post(event))
        .route("/api/v1/sessions/{id}/compact", post(compact))
        .route("/api/v1/sessions/{id}/actions", post(execute))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_headers(Any)
                .allow_methods(Any),
        )
        .with_state(state);
    let listener = tokio::net::TcpListener::bind(&config.bind)
        .await
        .expect("bind");
    tracing::info!(
        bind = %config.bind,
        namespace = %config.namespace,
        "context bastion ready"
    );
    axum::serve(listener, app).await.expect("serve");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn authorization_header_is_required() {
        let headers = HeaderMap::new();
        assert!(auth(&headers).is_err());
    }
}
