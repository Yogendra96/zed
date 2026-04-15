use serde::{Deserialize, Serialize};
use zed_extension_api::http_client::{self, HttpMethod, HttpRequest};
use anyhow::{Result, anyhow};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

#[derive(Serialize, Debug)]
struct ChatRequest {
    model: String,
    messages: Vec<ChatMessage>,
    stream: bool,
}

#[derive(Deserialize, Debug)]
struct ChatResponseChunk {
    choices: Vec<ChatChoiceChunk>,
}

#[derive(Deserialize, Debug)]
struct ChatChoiceChunk {
    delta: ChatDelta,
}

#[derive(Deserialize, Debug)]
struct ChatDelta {
    content: Option<String>,
}

pub struct AiClient {
    pub api_key: String,
    pub base_url: String,
    pub model: String,
}

impl AiClient {
    pub fn new(api_key: String, base_url: String, model: String) -> Self {
        Self { api_key, base_url, model }
    }

    pub fn stream_chat(&self, messages: Vec<ChatMessage>) -> Result<impl Iterator<Item = Result<String>>> {
        let request_body = serde_json::to_vec(&ChatRequest {
            model: self.model.clone(),
            messages,
            stream: true,
        })?;

        let request = HttpRequest::builder()
            .method(HttpMethod::Post)
            .url(format!("{}/chat/completions", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .body(request_body)
            .build()
            .map_err(|e| anyhow!("Failed to build request: {}", e))?;

        let stream = request.fetch_stream()
            .map_err(|e| anyhow!("HTTP request failed: {}", e))?;

        Ok(SseIterator { stream, buffer: Vec::new() })
    }
}

struct SseIterator {
    stream: http_client::HttpResponseStream,
    buffer: Vec<u8>,
}

impl Iterator for SseIterator {
    type Item = Result<String>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            // Try to parse a complete SSE message from the buffer
            if let Some(line_end) = self.buffer.iter().position(|&b| b == b'\n') {
                let line_bytes = self.buffer.drain(..line_end + 1).collect::<Vec<u8>>();
                let line = String::from_utf8_lossy(&line_bytes);
                
                if line.starts_with("data: ") {
                    let data = line["data: ".len()..].trim();
                    if data == "[DONE]" {
                        return None;
                    }
                    
                    if let Ok(chunk) = serde_json::from_str::<ChatResponseChunk>(data) {
                        if let Some(choice) = chunk.choices.first() {
                            if let Some(content) = &choice.delta.content {
                                return Some(Ok(content.clone()));
                            }
                        }
                    }
                }
                continue;
            }

            // If no complete line, fetch more from the stream
            match self.stream.next_chunk() {
                Ok(Some(chunk)) => {
                    self.buffer.extend(chunk);
                }
                Ok(None) => return None,
                Err(e) => return Some(Err(anyhow!("Stream error: {}", e))),
            }
        }
    }
}
