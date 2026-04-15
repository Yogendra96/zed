use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
        let lower = user_input.to_lowercase();

        let parallel_indicators = [
            "in parallel",
            "at the same time",
            "simultaneously",
            "both",
            "all three",
            "all four",
            "and also",
            "split into",
            "divide into",
            "do them together",
            "find vulnerabilities",
            "analyze performance",
            "check quality",
            "security scan",
            "performance analysis",
            "code review",
        ];

        let can_parallel = parallel_indicators
            .iter()
            .any(|indicator| lower.contains(indicator));

        if can_parallel {
            let agents = self.extract_parallel_tasks(user_input);
            SupervisorDecision {
                should_spawn_parallel: true,
                agents,
                should_synthesize: true,
            }
        } else {
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
    }

    fn extract_parallel_tasks(&self, input: &str) -> Vec<AgentSpawnConfig> {
        let sentences: Vec<&str> = input
            .split(|c| c == '.' || c == '\n' || c == ',')
            .filter(|s| !s.trim().is_empty())
            .collect();

        let mut agents = Vec::new();
        let keywords = [
            ("security", &["vulnerabilities", "scan", "audit"][..]),
            ("performance", &["bottleneck", "optimize", "slow"][..]),
            ("quality", &["code smell", "refactor", "improve"][..]),
            ("test", &["testing", "unit test", "coverage"][..]),
            ("explain", &["documentation", "document", "describe"][..]),
        ];

        for (idx, sentence) in sentences.iter().enumerate() {
            let lower = sentence.to_lowercase();
            let mut found = false;

            for (label, tags) in keywords.iter() {
                if tags.iter().any(|tag| lower.contains(tag)) {
                    agents.push(AgentSpawnConfig {
                        label: label.to_string(),
                        task: sentence.trim().to_string(),
                        estimated_duration: 20 + (idx as u32 * 5),
                    });
                    found = true;
                    break;
                }
            }

            if !found && agents.len() < 4 {
                agents.push(AgentSpawnConfig {
                    label: format!("task_{}", idx + 1),
                    task: sentence.trim().to_string(),
                    estimated_duration: 20,
                });
            }
        }

        agents.truncate(5);
        agents
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
            "I need security scan AND performance analysis AND code quality check in parallel",
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
