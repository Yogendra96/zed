use super::workspace::{AgentState, PendingDiff};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SynthesisResult {
    pub combined_summary: String,
    pub key_findings: Vec<String>,
    pub recommendations: Vec<String>,
    pub merged_diffs: Vec<SynthesizedDiff>,
    pub conflicts: Vec<Conflict>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SynthesizedDiff {
    pub file_path: String,
    pub changes: String,
    pub source_agents: Vec<String>,
    pub priority: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conflict {
    pub file_path: String,
    pub agent_1: String,
    pub agent_2: String,
    pub description: String,
    pub resolution: Option<String>,
}

pub struct Synthesis {
    conflict_resolution_strategy: ConflictStrategy,
}

#[derive(Debug, Clone, Default)]
pub enum ConflictStrategy {
    #[default]
    FirstWins,
    LastWins,
    Merge,
    KeepBoth,
    UserDecision,
}

impl Synthesis {
    pub fn new() -> Self {
        Self {
            conflict_resolution_strategy: ConflictStrategy::Merge,
        }
    }

    pub fn synthesize(
        &self,
        agent_states: &[AgentState],
        diffs: &[PendingDiff],
    ) -> SynthesisResult {
        let mut all_findings = Vec::new();
        let mut recommendations = Vec::new();

        for agent in agent_states {
            for finding in &agent.findings {
                all_findings.push(finding.clone());
            }
            if agent.status == crate::orchestrator::workspace::AgentStatus::Completed {
                recommendations.push(format!("{} completed successfully", agent.label));
            }
        }

        let merged_diffs = self.merge_diffs(diffs);
        let conflicts = self.detect_conflicts(diffs);

        let combined_summary = self.generate_summary(agent_states);

        SynthesisResult {
            combined_summary,
            key_findings: all_findings,
            recommendations,
            merged_diffs,
            conflicts,
        }
    }

    fn merge_diffs(&self, diffs: &[PendingDiff]) -> Vec<SynthesizedDiff> {
        let mut file_groups: HashMap<String, Vec<&PendingDiff>> = HashMap::new();

        for diff in diffs {
            file_groups
                .entry(diff.file_path.to_string_lossy().to_string())
                .or_default()
                .push(diff);
        }

        let mut merged = Vec::new();
        for (path, group) in file_groups {
            if group.len() == 1 {
                let d = group[0];
                merged.push(SynthesizedDiff {
                    file_path: path,
                    changes: format!("Changed {} lines", d.new_content.lines().count()),
                    source_agents: vec![d.agent_id.clone()],
                    priority: 1,
                });
            } else {
                let changes = group
                    .iter()
                    .map(|d| d.new_content.clone())
                    .collect::<Vec<_>>()
                    .join("\n---\n");

                merged.push(SynthesizedDiff {
                    file_path: path,
                    changes,
                    source_agents: group.iter().map(|d| d.agent_id.clone()).collect(),
                    priority: group.len() as u32,
                });
            }
        }

        merged.sort_by(|a, b| b.priority.cmp(&a.priority));
        merged
    }

    fn detect_conflicts(&self, diffs: &[PendingDiff]) -> Vec<Conflict> {
        let mut file_map: HashMap<String, Vec<&PendingDiff>> = HashMap::new();
        for diff in diffs {
            file_map
                .entry(diff.file_path.to_string_lossy().to_string())
                .or_default()
                .push(diff);
        }

        let mut conflicts = Vec::new();
        for (path, group) in file_map {
            if group.len() > 1 {
                conflicts.push(Conflict {
                    file_path: path,
                    agent_1: group[0].agent_id.clone(),
                    agent_2: group[1].agent_id.clone(),
                    description: "Multiple agents modified the same file".to_string(),
                    resolution: Some("Changes merged sequentially".to_string()),
                });
            }
        }
        conflicts
    }

    fn generate_summary(&self, agent_states: &[AgentState]) -> String {
        let completed = agent_states
            .iter()
            .filter(|s| s.status == crate::orchestrator::workspace::AgentStatus::Completed)
            .count();

        let total = agent_states.len();

        format!("Completed {}/{} agent tasks. ", completed, total)
    }
}

impl Default for Synthesis {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_synthesis() {
        let synthesis = Synthesis::new();
        let agent_states = vec![AgentState {
            id: "1".to_string(),
            label: "security".to_string(),
            status: crate::orchestrator::workspace::AgentStatus::Completed,
            current_task: None,
            findings: vec!["Found SQL injection".to_string()],
        }];

        let result = synthesis.synthesize(&agent_states, &[]);

        assert_eq!(result.key_findings.len(), 1);
        assert!(result.combined_summary.contains("1/1"));
    }
}
