#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct RayVersionResponse {
    pub version: String,
    pub ray_version: String,
    pub ray_commit: String,
}
