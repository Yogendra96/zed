use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskComplexity {
    Simple,
    Medium,
    Complex,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentSpawnConfig {
    pub label: String,
    pub task: String,
    pub estimated_duration: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupervisorDecision {
    pub should_spawn_parallel: bool,
    pub agents: Vec<AgentSpawnConfig>,
    pub should_synthesize: bool,
}

pub struct Supervisor;

impl Supervisor {
    pub fn new() -> Self {
        Self
    }

    pub fn analyze_task(&self, user_input: &str) -> SupervisorDecision {
        if let Some(cmd_idx) = user_input.find("/parallel ") {
            let config_str = &user_input[cmd_idx + "/parallel ".len()..];
            let end_idx = config_str.find('\n').unwrap_or(config_str.len());
            let tasks_str = &config_str[..end_idx];

            let agents = self.extract_parallel_tasks(tasks_str);
            if !agents.is_empty() {
                return SupervisorDecision {
                    should_spawn_parallel: true,
                    agents,
                    should_synthesize: true,
                };
            }
        }

        SupervisorDecision {
            should_spawn_parallel: false,
            agents: vec![AgentSpawnConfig {
                label: "general".to_string(),
                task: user_input.to_string(),
                estimated_duration: 30,
            }],
            should_synthesize: false,
        }
    }

    fn extract_parallel_tasks(&self, tasks_str: &str) -> Vec<AgentSpawnConfig> {
        tasks_str
            .split(',')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .enumerate()
            .map(|(_, task_name)| AgentSpawnConfig {
                label: task_name.to_lowercase(),
                task: format!("Perform {} task", task_name),
                estimated_duration: 30,
            })
            .collect()
    }

    pub fn estimate_complexity(&self, input: &str) -> TaskComplexity {
        let word_count = input.split_whitespace().count();
        let complex_keywords = [
            "architecture",
            "redesign",
            "migration",
            "security",
            "performance",
            "optimization",
            "parallel",
            "distributed",
        ];
        let has_complex_keywords = complex_keywords
            .iter()
            .any(|kw| input.to_lowercase().contains(kw));

        if word_count > 100 || has_complex_keywords {
            TaskComplexity::Complex
        } else if word_count > 30 {
            TaskComplexity::Medium
        } else {
            TaskComplexity::Simple
        }
    }
}

impl Default for Supervisor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parallel_detection() {
        let supervisor = Supervisor::new();

        let result = supervisor.analyze_task(
            "/parallel security scan, performance analysis, code quality check\nHere is my prompt",
        );

        assert!(result.should_spawn_parallel);
        assert!(result.agents.len() >= 3);
        assert!(result.should_synthesize);
    }

    #[test]
    fn test_simple_task() {
        let supervisor = Supervisor::new();

        let result = supervisor.analyze_task("explain what does this function do?");

        assert!(!result.should_spawn_parallel);
        assert_eq!(result.agents.len(), 1);
    }
}
