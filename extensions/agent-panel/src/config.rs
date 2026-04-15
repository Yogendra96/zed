use anyhow::{Result, anyhow};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub providers: Vec<ProviderConfig>,
    #[serde(default)]
    pub custom_agents: HashMap<String, CustomAgentConfig>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ProviderConfig {
    pub name: String,
    pub base_url: String,
    pub api_key: String,
    pub default_model: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct CustomAgentConfig {
    pub description: String,
    pub system_prompt: String,
    #[serde(default)]
    pub tools: Vec<String>,
    #[serde(default)]
    pub max_iterations: Option<u32>,
}

impl Config {
    pub fn default_local() -> Self {
        Self {
            providers: vec![ProviderConfig {
                name: "ollama".to_string(),
                base_url: "http://localhost:11434/v1".to_string(),
                api_key: "ollama".to_string(),
                default_model: "llama3".to_string(),
            }],
            custom_agents: HashMap::new(),
        }
    }

    pub fn load() -> Result<Self> {
        // Load from ~/.config/zed/agent-panel.json
        let config_path = dirs::config_dir()
            .ok_or_else(|| anyhow!("No config dir"))?
            .join("zed")
            .join("agent-panel.json");

        if config_path.exists() {
            let content = std::fs::read_to_string(&config_path)?;
            serde_json::from_str(&content).map_err(|e| anyhow!(e))
        } else {
            Ok(Self::default_local())
        }
    }

    pub fn get_agent(&self, name: &str) -> Option<CustomAgentConfig> {
        self.custom_agents.get(name).cloned()
    }

    pub fn list_agents(&self) -> Vec<(String, CustomAgentConfig)> {
        self.custom_agents
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect()
    }
}
