// Reference: https://docs.ray.io/en/latest/_modules/ray/runtime_env/runtime_env.html#RuntimeEnv

use serde;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct RuntimeEnvConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_timeout_seconds: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eager_install: Option<bool>,
}

impl RuntimeEnvConfig {
    /// Create a new RuntimeEnvConfig.
    pub fn new() -> Self {
        Default::default()
    }

    /// Set the setup timeout seconds for the runtime environment.
    pub fn with_setup_timeout_seconds(mut self, seconds: u64) -> Self {
        self.setup_timeout_seconds = Some(seconds);
        self
    }

    /// Set the eager install flag for the runtime environment.
    pub fn with_eager_install(mut self, eager: bool) -> Self {
        self.eager_install = Some(eager);
        self
    }
}

#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct RuntimeEnv {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub working_dir: Option<PathBuf>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env_vars: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<RuntimeEnvConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pip: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uv: Option<Vec<String>>,
}

impl RuntimeEnv {
    /// Create a new RuntimeEnv.
    pub fn new() -> Self {
        Default::default()
    }

    /// Set the working directory for the runtime environment.
    /// This should be a local directory path that will be uploaded to Ray.
    pub fn with_working_dir(mut self, working_dir: &Path) -> Self {
        self.working_dir = Some(working_dir.to_path_buf());
        self
    }

    /// Set environment variables for the runtime environment.
    pub fn with_env_vars(mut self, env_vars: HashMap<String, String>) -> Self {
        self.env_vars = Some(env_vars);
        self
    }

    /// Set the configuration for the runtime environment.
    pub fn with_config(mut self, config: RuntimeEnvConfig) -> Self {
        self.config = Some(config);
        self
    }

    /// Set the pip packages for the runtime environment.
    pub fn with_pip_packages(mut self, pip: &[String]) -> Self {
        self.pip = Some(pip.to_vec());
        self
    }

    /// Set the UV packages for the runtime environment.
    pub fn with_uv_packages(mut self, uv: &[String]) -> Self {
        self.uv = Some(uv.to_vec());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::RuntimeEnv;
    use std::path::Path;

    #[test]
    fn test_skip_none() {
        let env = RuntimeEnv::new();
        let json = serde_json::to_value(&env).unwrap();
        let expected = serde_json::json!({});
        assert_eq!(json, expected);
    }

    #[test]
    fn test_round_trip() {
        let env = RuntimeEnv::new().with_working_dir(Path::new("/tests"));
        let value = serde_json::to_value(&env).unwrap();
        let deserialized_env: RuntimeEnv = serde_json::from_value(value).unwrap();
        assert_eq!(
            deserialized_env.working_dir,
            Some(Path::new("/tests").to_path_buf())
        );
    }
}
