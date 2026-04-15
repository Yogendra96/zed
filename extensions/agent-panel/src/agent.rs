use anyhow::{Result, anyhow};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug, Clone)]
pub struct Agent {
    pub name: String,
    pub description: String,
    pub system_prompt: String,
    #[serde(default)]
    pub tools: Vec<String>,
    #[serde(default)]
    pub max_iterations: Option<u32>,
}

impl Agent {
    pub fn load_builtin(name: &str) -> Result<Self> {
        match name {
            "explain" => Ok(Self {
                name: "Explain".to_string(),
                description: "Explains code in detail".to_string(),
                system_prompt: "You are a senior software engineer. Explain the following code clearly and concisely.".to_string(),
                tools: vec!["read_file".to_string(), "grep".to_string()],
                max_iterations: Some(10),
            }),
            "test" => Ok(Self {
                name: "Test Generator".to_string(),
                description: "Generates unit tests".to_string(),
                system_prompt: "Generate comprehensive unit tests for the provided code using best practices.".to_string(),
                tools: vec!["read_file".to_string(), "edit_file".to_string()],
                max_iterations: Some(20),
            }),
            "audit" => Ok(Self {
                name: "Security Auditor".to_string(),
                description: "Audits code for vulnerabilities".to_string(),
                system_prompt: "Audit the following code for security vulnerabilities, performance bottlenecks, and style issues.".to_string(),
                tools: vec!["read_file".to_string(), "grep".to_string(), "terminal".to_string()],
                max_iterations: Some(15),
            }),
            "refactor" => Ok(Self {
                name: "Refactor Expert".to_string(),
                description: "Refactors code for clarity and efficiency".to_string(),
                system_prompt: "Refactor the following code to improve readability, maintainability, and efficiency while preserving its functionality.".to_string(),
                tools: vec!["read_file".to_string(), "edit_file".to_string(), "grep".to_string()],
                max_iterations: Some(25),
            }),
            _ => Err(anyhow!("Unknown agent: {}", name)),
        }
    }

    pub fn from_config(name: &str, config: crate::config::CustomAgentConfig) -> Self {
        Self {
            name: name.to_string(),
            description: config.description,
            system_prompt: config.system_prompt,
            tools: config.tools,
            max_iterations: config.max_iterations,
        }
    }

    pub fn load_dynamic(name: &str, config: crate::config::CustomAgentConfig) -> Self {
        Self::from_config(name, config)
    }

    pub fn all_builtins() -> HashMap<String, Self> {
        let mut agents = HashMap::new();
        for name in &["explain", "test", "audit", "refactor"] {
            if let Ok(agent) = Self::load_builtin(name) {
                agents.insert(name.to_string(), agent);
            }
        }
        agents
    }
}
