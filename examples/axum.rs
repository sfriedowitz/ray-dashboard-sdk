use std::sync::Arc;

use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::post,
};
use ray_dashboard_sdk::{
    RayDashboardClient,
    client::jobs::JobSubmissionAPI,
    schemas::{
        env::RuntimeEnv,
        jobs::{JobSubmitRequest, JobSubmitResponse},
    },
};
use serde::{Deserialize, Serialize};
use tempfile::TempDir;
use thiserror::Error;
use tracing::{Level, info};
use tracing_subscriber::FmtSubscriber;
use uuid::Uuid;

#[derive(Debug, Clone)]
struct AppState {
    ray_client: RayDashboardClient,
}

#[derive(Debug, Deserialize)]
struct JobRequest {
    script: String,
}

#[derive(Debug, Serialize)]
struct JobResponse {
    job_id: String,
}

#[derive(Debug, Error)]
enum AppError {
    #[error("Ray error: {0}")]
    RayError(#[from] ray_dashboard_sdk::Error),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let message = self.to_string();
        (StatusCode::INTERNAL_SERVER_ERROR, message).into_response()
    }
}

async fn submit_job(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<JobRequest>,
) -> Result<Json<JobResponse>, AppError> {
    // Create a temporary directory for the job script
    let temp_dir = TempDir::new()?;
    let script_path = temp_dir.path().join("script.py");

    // Write the script to the temp directory
    std::fs::write(&script_path, payload.script)?;

    // Generate a unique submission ID
    let submission_id = Uuid::new_v4().to_string();

    // Create the runtime environment with the working directory
    let runtime_env = RuntimeEnv::new().with_working_dir(temp_dir.path());

    // Create the job submission request
    let job_request = JobSubmitRequest::new("python script.py")
        .with_submission_id(submission_id.clone())
        .with_runtime_env(runtime_env);

    // Submit the job to Ray
    let response: JobSubmitResponse = state.ray_client.submit_job(&job_request).await?;

    Ok(Json(JobResponse {
        job_id: response.submission_id,
    }))
}

#[tokio::main]
async fn main() {
    // Initialize tracing subscriber
    let subscriber = FmtSubscriber::builder().with_max_level(Level::INFO).finish();
    tracing::subscriber::set_global_default(subscriber).expect("Failed to set subscriber");

    // Get Ray Dashboard connection info from environment variables
    let host = std::env::var("RAY_DASHBOARD_HOST").unwrap_or("127.0.0.1".to_string());
    let port = std::env::var("RAY_DASHBOARD_PORT").unwrap_or("8265".to_string());
    let ray_url = format!("http://{}:{}", host, port);

    // Create Ray Dashboard client
    let ray_client = RayDashboardClient::new(&ray_url).expect("Failed to create Ray client");

    // Create shared application state
    let state = Arc::new(AppState { ray_client });

    // Build the Axum router
    let app = Router::new()
        .route("/api/jobs", post(submit_job))
        .with_state(state);

    // Get the server bind address
    let bind_addr = "127.0.0.1:8000";

    info!("Starting Axum server on {}", bind_addr);
    info!("Ray Dashboard URL: {}", ray_url);

    // Start the server
    let listener = tokio::net::TcpListener::bind(&bind_addr)
        .await
        .expect("Failed to bind address");

    axum::serve(listener, app).await.expect("Failed to start server");
}
