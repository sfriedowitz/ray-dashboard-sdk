// Reference: https://docs.ray.io/en/latest/_modules/ray/runtime_env/runtime_env.html#RuntimeEnv

use serde;
use std::collections::HashMap;

#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct RuntimeEnvConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    setup_timeout_seconds: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    eager_install: Option<bool>,
}

impl RuntimeEnvConfig {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_setup_timeout_seconds(mut self, seconds: u64) -> Self {
        self.setup_timeout_seconds = Some(seconds);
        self
    }

    pub fn with_eager_install(mut self, eager: bool) -> Self {
        self.eager_install = Some(eager);
        self
    }
}

#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct RuntimeEnv {
    #[serde(skip_serializing_if = "Option::is_none")]
    working_dir: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    env_vars: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    py_modules: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    config: Option<RuntimeEnvConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pip: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    uv: Option<Vec<String>>,
}

impl RuntimeEnv {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_working_dir(mut self, working_dir: impl Into<String>) -> Self {
        self.working_dir = Some(working_dir.into());
        self
    }

    pub fn with_env_var(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        if self.env_vars.is_none() {
            self.env_vars = Some(HashMap::new());
        }
        self.env_vars.as_mut().unwrap().insert(name.into(), value.into());
        self
    }

    pub fn with_py_module(mut self, module: impl Into<String>) -> Self {
        if self.py_modules.is_none() {
            self.py_modules = Some(Vec::new());
        }
        self.py_modules.as_mut().unwrap().push(module.into());
        self
    }

    pub fn with_config(mut self, config: RuntimeEnvConfig) -> Self {
        self.config = Some(config);
        self
    }

    pub fn with_pip(mut self, pip: &[String]) -> Self {
        self.pip = Some(pip.to_vec());
        self
    }

    pub fn with_uv(mut self, uv: &[String]) -> Self {
        self.uv = Some(uv.to_vec());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::RuntimeEnv;

    #[test]
    fn test_skip_none() {
        let env = RuntimeEnv::new();
        let json = serde_json::to_value(&env).unwrap();
        let expected = serde_json::json!({});
        assert_eq!(json, expected);
    }

    #[test]
    fn test_round_trip() {
        let env = RuntimeEnv::new().with_working_dir("/tests");
        let value = serde_json::to_value(&env).unwrap();
        let deserialized_env: RuntimeEnv = serde_json::from_value(value).unwrap();
        assert_eq!(deserialized_env.working_dir, Some("/tests".to_string()));
    }
}
