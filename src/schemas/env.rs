// Reference: https://docs.ray.io/en/latest/_modules/ray/runtime_env/runtime_env.html#RuntimeEnv

use serde;
use std::collections::HashMap;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub enum RuntimeEnvPipSettings {
    Packages(Vec<String>),
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub enum RuntimeEnvUVSettings {
    Packages(Vec<String>),
}

#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct RuntimeEnvConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    setup_timeout_seconds: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    eager_install: Option<bool>,
}

impl RuntimeEnvConfig {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_setup_timeout_seconds(mut self, seconds: u32) -> Self {
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
    uv: Option<RuntimeEnvPipSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pip: Option<RuntimeEnvUVSettings>,
}

impl RuntimeEnv {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_working_dir(mut self, working_dir: Option<&str>) -> Self {
        self.working_dir = working_dir.map(Into::into);
        self
    }

    pub fn with_env_var(mut self, name: &str, value: &str) -> Self {
        if self.env_vars.is_none() {
            self.env_vars = Some(HashMap::new());
        }
        self.env_vars
            .as_mut()
            .unwrap()
            .insert(name.to_string(), value.to_string());
        self
    }

    pub fn with_py_modules(mut self, module: &str) -> Self {
        if self.py_modules.is_none() {
            self.py_modules = Some(Vec::new());
        }
        self.py_modules.as_mut().unwrap().push(module.to_string());
        self
    }

    pub fn with_config(mut self, config: RuntimeEnvConfig) -> Self {
        self.config = Some(config);
        self
    }

    pub fn with_pip(mut self, pip: RuntimeEnvUVSettings) -> Self {
        self.pip = Some(pip);
        self
    }

    pub fn with_uv(mut self, uv: RuntimeEnvPipSettings) -> Self {
        self.uv = Some(uv);
        self
    }
}

#[cfg(test)]
mod tests {}
