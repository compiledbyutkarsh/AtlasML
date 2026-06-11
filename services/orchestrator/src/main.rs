use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};
use redis::Commands;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use uuid::Uuid;

#[derive(Clone)]
struct AppState {
    redis_url: String,
}

#[derive(Serialize, Deserialize)]
struct Job {
    id: String,
    name: String,
}

#[derive(Deserialize)]
struct JobInput {
    name: String,
}

#[tokio::main]
async fn main() {
    let state = AppState {
        redis_url: "redis://127.0.0.1/".to_string(),
    };

    let app = Router::new()
        .route("/health", get(|| async { "ok" }))
        .route("/jobs", post(create_job))
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("🚀 AtlasML running on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn create_job(
    State(state): State<AppState>,
    Json(payload): Json<JobInput>,
) -> String {
    let job = Job {
        id: Uuid::new_v4().to_string(),
        name: payload.name,
    };

    let client = redis::Client::open(state.redis_url).unwrap();
    let mut conn = client.get_connection().unwrap();

    let data = serde_json::to_string(&job).unwrap();
    let _: () = conn.lpush("jobs_queue", data).unwrap();

    format!("queued: {}", job.id)
}