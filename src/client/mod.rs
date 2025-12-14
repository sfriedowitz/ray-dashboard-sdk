use tracing::info;

use crate::schemas::common::RayVersionResponse;

pub mod jobs;
pub mod packages;

#[derive(Debug, Clone)]
pub struct RayDashboardClient {
    base_url: url::Url,
    client: reqwest::Client,
}

impl RayDashboardClient {
    pub fn new(base_url: &str) -> crate::Result<Self> {
        let client = reqwest::Client::builder().build()?;
        Self::new_with_client(base_url, client)
    }

    pub fn new_with_client(base_url: &str, client: reqwest::Client) -> crate::Result<Self> {
        let base_url = url::Url::parse(base_url)?;
        Ok(Self { base_url, client })
    }

    pub async fn ping(&self) -> crate::Result<()> {
        self.get_version().await?;
        Ok(())
    }

    pub async fn get_version(&self) -> crate::Result<RayVersionResponse> {
        let path = "/api/version";
        let request = self.base_request(reqwest::Method::GET, path)?;
        let response = request.send().await?.error_for_status()?;
        Ok(response.json::<RayVersionResponse>().await?)
    }

    /// Build base request with common headers.
    /// Ray dashboard server requires User-Agent header to be set or else 500s.
    fn base_request(&self, method: reqwest::Method, path: &str) -> crate::Result<reqwest::RequestBuilder> {
        let url = self.base_url.join(path)?;
        info!("Building request: {} {}", method, url);
        let request = self
            .client
            .request(method, url)
            .header("User-Agent", crate::constants::SDK_USER_AGENT);
        Ok(request)
    }
}
