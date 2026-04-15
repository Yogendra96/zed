use crate::agent::Agent;
use crate::ai_client::{AiClient, ChatMessage};
use crate::config::Config;
use crate::context::Context;
use zed_extension_api::{SlashCommandOutput, SlashCommandOutputSection, Worktree};

pub fn handle_command(
    name: &str,
    args: &[String],
    worktree: Option<&Worktree>,
) -> Result<SlashCommandOutput, String> {
    let config = Config::load().map_err(|e| e.to_string())?;
    let provider = config
        .providers
        .first()
        .ok_or("No AI providers configured")?;

    let client = AiClient::new(
        provider.api_key.clone(),
        provider.base_url.clone(),
        provider.default_model.clone(),
    );

    // First check custom agents from config, then fall back to builtins
    let agent = if let Some(custom_config) = config.get_agent(name) {
        Agent::load_dynamic(name, custom_config)
    } else {
        Agent::load_builtin(name).unwrap_or_else(|_err| Agent {
            name: "Generic".to_string(),
            description: "Generic AI Assistant".to_string(),
            system_prompt: "You are a helpful AI assistant.".to_string(),
            tools: vec![],
            max_iterations: None,
        })
    };

    let prompt = args.join(" ");
    if prompt.is_empty() {
        return Err("Please provide a prompt".to_string());
    }

    // Gather Context
    let context = worktree.and_then(|wt| Context::collect_from_worktree(wt).ok());
    let mut final_prompt = prompt;
    if let Some(ctx) = context {
        final_prompt.push_str(&ctx.to_prompt());
    }

    let messages = vec![
        ChatMessage {
            role: "system".to_string(),
            content: agent.system_prompt,
        },
        ChatMessage {
            role: "user".to_string(),
            content: final_prompt,
        },
    ];

    let mut output_text = String::new();
    let chunks = client.stream_chat(messages).map_err(|e| e.to_string())?;

    for chunk in chunks {
        match chunk {
            Ok(text) => output_text.push_str(&text),
            Err(e) => return Err(format!("AI Stream Error: {}", e)),
        }
    }

    Ok(SlashCommandOutput {
        sections: vec![SlashCommandOutputSection {
            range: (0..output_text.len()).into(),
            label: format!("{} Response ({})", agent.name, provider.name),
        }],
        text: output_text,
    })
}
