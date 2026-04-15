use serde::{Deserialize, Serialize};
use crate::ai_client::ChatMessage;
use anyhow::Result;
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct Conversation {
    pub id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub messages: Vec<ChatMessage>,
}

impl Conversation {
    pub fn new(messages: Vec<ChatMessage>) -> Self {
        Self {
            id: Uuid::new_v4(),
            timestamp: Utc::now(),
            messages,
        }
    }

    pub fn save(&self) -> Result<()> {
        // In a real implementation, we would save to .zed/ai/history/{id}.json
        // For now, this is a stub.
        Ok(())
    }
}
