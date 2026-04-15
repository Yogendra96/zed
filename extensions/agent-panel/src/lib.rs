mod panel;
mod router;
mod ai_client;
mod agent;
mod context;
mod history;
mod approval;
mod config;

use zed_extension_api::{self as zed, SlashCommand, SlashCommandOutput, Worktree};

struct AgentPanelExtension;

impl zed::Extension for AgentPanelExtension {
    fn new() -> Self {
        Self
    }

    fn run_slash_command(
        &self,
        command: SlashCommand,
        args: Vec<String>,
        worktree: Option<&Worktree>,
    ) -> Result<SlashCommandOutput, String> {
        router::handle_command(&command.name, &args, worktree)
    }
}

zed::register_extension!(AgentPanelExtension);
