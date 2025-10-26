// Include generated base Client
include!(concat!(env!("OUT_DIR"), "/jobs.rs"));

pub use self::types::*;

impl self::Client {
    pub fn new_with_user_agent(base_url: &str) -> Self {
        let http_client = reqwest::Client::builder()
            .user_agent(crate::constants::RAY_DASHBOARD_CLIENT_USER_AGENT)
            .build()
            .unwrap();
        Self::new_with_client(base_url, http_client)
    }
}

pub struct JobSubmitRequestBuilder {}

impl JobSubmitRequestBuilder {
    pub fn new() -> Self {
        Self {}
    }

    pub fn build(self) -> JobSubmitRequest {
        todo!()
    }
}
