use gpui::SharedString;
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileState {
    pub path: PathBuf,
    pub content: String,
    pub modified: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentMessage {
    pub from_agent: String,
    pub to_agent: Option<String>,
    pub content: String,
    pub timestamp: u64,
    pub message_type: MessageType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageType {
    Request,
    Response,
    Broadcast,
    StatusUpdate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PendingDiff {
    pub file_path: PathBuf,
    pub original_content: String,
    pub new_content: String,
    pub agent_id: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceStats {
    pub active_agents: usize,
    pub total_messages: usize,
    pub pending_diffs: usize,
    pub files_tracked: usize,
}

pub struct SharedWorkspace {
    files: RwLock<HashMap<PathBuf, FileState>>,
    messages: RwLock<Vec<AgentMessage>>,
    diffs: RwLock<Vec<PendingDiff>>,
    agent_states: RwLock<HashMap<String, AgentState>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentState {
    pub id: String,
    pub label: String,
    pub status: AgentStatus,
    pub current_task: Option<String>,
    pub findings: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AgentStatus {
    Idle,
    Running,
    Completed,
    Failed,
    Waiting,
}

impl SharedWorkspace {
    pub fn new() -> Self {
        Self {
            files: RwLock::new(HashMap::new()),
            messages: RwLock::new(Vec::new()),
            diffs: RwLock::new(Vec::new()),
            agent_states: RwLock::new(HashMap::new()),
        }
    }

    pub fn track_file(&self, path: PathBuf, content: String) {
        let mut files = self.files.write();
        let path_clone = path.clone();
        files.insert(
            path,
            FileState {
                path: path_clone,
                content,
                modified: false,
            },
        );
    }

    pub fn update_file(&self, path: &PathBuf, content: String) {
        let mut files = self.files.write();
        if let Some(file) = files.get_mut(path) {
            file.content = content;
            file.modified = true;
        }
    }

    pub fn get_file(&self, path: &PathBuf) -> Option<String> {
        let files = self.files.read();
        files.get(path).map(|f| f.content.clone())
    }

    pub fn add_message(&self, from: &str, to: Option<&str>, content: &str, msg_type: MessageType) {
        let mut messages = self.messages.write();
        messages.push(AgentMessage {
            from_agent: from.to_string(),
            to_agent: to.map(|s| s.to_string()),
            content: content.to_string(),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            message_type: msg_type,
        });
    }

    pub fn broadcast_message(&self, from: &str, content: &str) {
        self.add_message(from, None, content, MessageType::Broadcast);
    }

    pub fn get_messages(&self) -> Vec<AgentMessage> {
        self.messages.read().clone()
    }

    pub fn add_diff(
        &self,
        file_path: PathBuf,
        original: String,
        new: String,
        agent_id: &str,
        description: &str,
    ) {
        let mut diffs = self.diffs.write();
        diffs.push(PendingDiff {
            file_path,
            original_content: original,
            new_content: new,
            agent_id: agent_id.to_string(),
            description: description.to_string(),
        });
    }

    pub fn get_diffs(&self) -> Vec<PendingDiff> {
        self.diffs.read().clone()
    }

    pub fn register_agent(&self, id: &str, label: &str) {
        let mut states = self.agent_states.write();
        states.insert(
            id.to_string(),
            AgentState {
                id: id.to_string(),
                label: label.to_string(),
                status: AgentStatus::Idle,
                current_task: None,
                findings: Vec::new(),
            },
        );
    }

    pub fn update_agent_status(&self, id: &str, status: AgentStatus) {
        let mut states = self.agent_states.write();
        if let Some(agent) = states.get_mut(id) {
            agent.status = status;
        }
    }

    pub fn add_agent_finding(&self, id: &str, finding: String) {
        let mut states = self.agent_states.write();
        if let Some(agent) = states.get_mut(id) {
            agent.findings.push(finding);
        }
    }

    pub fn get_agent_state(&self, id: &str) -> Option<AgentState> {
        self.agent_states.read().get(id).cloned()
    }

    pub fn get_all_agent_states(&self) -> Vec<AgentState> {
        self.agent_states.read().values().cloned().collect()
    }

    pub fn get_stats(&self) -> WorkspaceStats {
        WorkspaceStats {
            active_agents: self.agent_states.read().len(),
            total_messages: self.messages.read().len(),
            pending_diffs: self.diffs.read().len(),
            files_tracked: self.files.read().len(),
        }
    }

    pub fn clear(&self) {
        self.files.write().clear();
        self.messages.write().clear();
        self.diffs.write().clear();
        self.agent_states.write().clear();
    }
}

impl Default for SharedWorkspace {
    fn default() -> Self {
        Self::new()
    }
}
