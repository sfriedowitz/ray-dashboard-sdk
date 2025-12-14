use std::env;

use ray_dashboard_sdk::{RayDashboardClient, client::jobs::JobSubmissionAPI};
use tracing::{Level, info};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing subscriber
    let subscriber = FmtSubscriber::builder().with_max_level(Level::INFO).finish();
    tracing::subscriber::set_global_default(subscriber)?;

    // Get Ray Dashboard connection info from environment variables
    let host = env::var("RAY_DASHBOARD_HOST").unwrap_or("127.0.0.1".to_string());
    let port = env::var("RAY_DASHBOARD_PORT").unwrap_or("8265".to_string());
    let base_url = format!("http://{}:{}", host, port);

    info!("Connecting to Ray Dashboard at: {}", base_url);

    // Create the client
    let client = RayDashboardClient::new(&base_url)?;

    // Ping the server to check connectivity
    info!("Pinging Ray Dashboard...");
    client.ping().await?;
    info!("Successfully connected to Ray Dashboard");

    // List all jobs
    info!("Listing all jobs");
    let jobs = client.list_jobs().await?;

    if jobs.is_empty() {
        info!("No jobs found");
    } else {
        info!("Found {} job(s)", jobs.len());
        for job in jobs {
            let id = job.submission_id.unwrap();
            info!("Job: {} (status: {:?})", id, job.status);
        }
    }

    Ok(())
}
