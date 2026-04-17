# QData Multi-Agent & Subagent Configuration Master Plan

This document represents the comprehensive, merged state of the QData Multi-Agent implementation and the Subagent Model Configuration plan. It outlines what has been built, the exact architecture for subagent routing, testing procedures, and future improvements.

---

## 1. System Overview & What We Did So Far

### Orchestrator Modules Created
- `crates/agent/src/orchestrator/supervisor.rs` - Task decomposition for parallel agents and keyword detection.
- `crates/agent/src/orchestrator/workspace.rs` - Shared state between agents.
- `crates/agent/src/orchestrator/synthesis.rs` - Result merging logic.

### System Prompt Updates (`crates/agent/src/templates/system_prompt.hbs`)
Added explicit rules for multi-agent delegation:
- **EXPLICIT USER REQUEST ONLY**: Agent only spawns sub-agents when the user explicitly asks for parallel agents.
- **SHOW PLAN FIRST**: Always shows the exact plan before spawning (number of agents, what each does).
- **WAIT FOR APPROVAL**: Waits for user to confirm before spawning.
- **SYNTHESIZE RESULTS**: Merges results from multiple agents into a coherent response.

### Git Setup
- **Branch**: `qdata-multi-agent-v1`
- **Pushed to**: https://github.com/Yogendra96/zed/tree/qdata-multi-agent-v1

---

## 2. Subagent Model Configuration Plan

Allow users to configure which AI model subagents use via a UI in Zed settings. Subagents inherit from the user's currently selected main agent model by default, but the user can override this globally or per-task-type.

### User Experience Scenarios
1. **Default Behavior**: Main agent is "Claude Sonnet". Subagents spawn using "Claude Sonnet". UI shows: *"Using model: Claude Sonnet (inherited from main)"*.
2. **Task Override**: User sets preset "Security = Claude Opus", "Quality = GPT-4o". Spawning a security subagent uses Opus; a quality subagent uses GPT-4o. UI shows: *"Using model: Claude Opus (security preset)"*.
3. **Custom Global Override**: User sets "Always use: Gemini 2.0" for all subagents. UI shows: *"Using model: Gemini 2.0 (override)"*.

### Data Model & Settings Schema

```json
// In settings.json
"agent": {
  "subagent": {
    "use_main_agent_model": true,
    "default_model": {
      "provider": "copilot_chat",
      "model": "gpt-4o"
    },
    "task_overrides": {
      "security": { "provider": "anthropic", "model": "claude-opus-4-5" },
      "performance": { "provider": "openrouter", "model": "openai/gpt-4o" },
      "quality": { "provider": "copilot_chat", "model": "gpt-4o" },
      "test": { "provider": "anthropic", "model": "claude-sonnet-4-20250514" },
      "explain": { "provider": "openrouter", "model": "deepseek/deepseek-chat" }
    }
  }
}
```

### Task Type Detection
Match user prompts to task types via keyword detection (located in `supervisor.rs`):
- **security** → "security", "vulnerability", "audit", "scan", "hack", "exploit"
- **performance** → "performance", "bottleneck", "optimize", "slow", "profile"
- **quality** → "quality", "code smell", "refactor", "improve", "lint"
- **test** → "test", "testing", "coverage", "unittest", "spec"
- **explain** → "explain", "document", "describe", "what does"
- **default** → fallback to main agent model or `default_model`

### UI Design
Located in **Settings → Agent → "Subagent" section**:
1. **Main Toggle**: "Use main agent's model for subagents" (ON/OFF).
2. **Model Selector**: Provider/Model dropdowns + "Test Connection" (Visible when toggle is OFF).
3. **Task-Type Overrides Section**: Collapsible panel with a table showing Task Type | Model | Provider | Action [Edit/Clear]. "Add Override" button to open modal.
4. **Presets (Future)**: Save/Load configuration presets.

---

## 3. Required File Changes for Subagent Config

1. **Settings Definition** (`crates/agent_settings/src/agent_settings.rs`):
   - Add `SubagentSettings` struct with `use_main_agent_model`, `default_model`, and `task_overrides`.
2. **Settings UI** (`crates/agent_settings/src/agent_settings_ui.rs`):
   - Implement the Main toggle, Model selector, and Task override table.
3. **Subagent Spawn Logic** (`crates/agent/src/agent.rs` or `crates/agent/src/tools/spawn_agent_tool.rs`):
   - Implement `get_model_for_subagent()` to route via (1) Task-override, (2) Global default, (3) Main agent, (4) Ultimate fallback.
4. **Task Label Detection** (`crates/agent/src/orchestrator/supervisor.rs`):
   - Implement `detect_task_type(prompt: &str) -> TaskType` keyword matcher.
5. **Thread Creation Update** (`crates/agent/src/thread.rs`):
   - Update `Thread::new_subagent` to accept the `LanguageModelSelector` parameter.
6. **Logs/Display** (`crates/agent/src/tools/spawn_agent_tool.rs`):
   - Inject log: `[orchestrator] Subagent spawned: label={}, session_id={}, model={}/{}`.

### Implementation Phases & Timeline (~8-12 hours total)
- **Phase 1: Settings & Schema (High Priority)** (2-3h): Define structs, register in settings, update `default.json`, verify save/load.
- **Phase 2: UI (High Priority)** (3-4h): Create subagent settings panel, toggles, selectors, and override tables.
- **Phase 3: Integration (High Priority)** (2-3h): Modify `spawn_agent_tool.rs`, implement detection and routing, pass model to thread creation.
- **Phase 4: Polish (Medium Priority)** (1-2h): Add "current model" UI indicators in subagent cards, add logging, and error handling.

---

## 4. Testing & Verification Plan

### High Priority Multi-Agent Integration Tests
1. **Prompt the agent**: "Run security scan AND code quality check in parallel using spawn_agent"
   - *Expected*: Shows plan, waits for approval, THEN spawns agents using the respective overridden models.
2. **Verify the system prompt was applied**: Ask "Show me your system prompt".
   - *Expected*: Shows the new multi-agent rules section.
3. **Test parallel spawning**: Ask for multiple independent tasks.
   - *Expected*: Sees sub-agent cards in UI.
4. **Test Tool Permissions**: Set all tool permissions to `Allow` mode and verify terminal commands bypass confirmation prompts.

### Subagent Config Test Cases
1. **Default**: Subagent correctly inherits the main agent's model.
2. **Override all**: Subagent uses specified global override model for all tasks.
3. **Task override**: "Analyze this code" uses default. "Run performance analysis on main.py" triggers the Performance override model.
4. **UI Validation**: Toggles save state correctly, table CRUD operations work.
5. **Invalid model**: Graceful fallback to default if provider API fails.

---

## 5. Future Improvements & Open Questions

### Immediate (Next Sprint)
1. **Add supervisor visibility UI**: Show an active agents panel displaying status (running/idle/completed) and allow cancellation from UI.
2. **Tool Permission Default**: Consider changing default from `Confirm` to `Allow` in settings (Skipped currently due to risk).
3. **Logging Enhancement (DONE)**: Log sub-agent spawns to `Zed.log` (added via `log::info!` in `spawn_agent_tool.rs`).

### Medium Term
1. **Dynamic Agent Configuration**: User-configurable agent roles, custom labels, and task definitions.
2. **Result Caching**: Cache results from similar tasks to avoid re-running identical analyses.
3. **Build Hardware**: Ensure a 16GB+ RAM machine or CI pipeline is used for release builds.

### Long Term
1. **Custom Orchestrator Panel**: A dedicated UI panel showing orchestrator state with drag-and-drop task distribution.
2. **Presets**: Loadable user configurations for subagent deployments.

### Open Questions (Subagent Logic)
1. Should there be a limit on max subagent depth? *(Currently exists: `MAX_SUBAGENT_DEPTH=5`)*
2. Should we show cost estimates per subagent given the multi-model architecture?
3. Should subagent history be completely separate or visually merged with the main thread?
4. How to robustly handle API key authentication issues for subagent models that differ from the main agent?

### Risk Assessment (Subagent Configuration)
| Risk | Mitigation |
|------|------------|
| Model not available for provider | Add validation, fallback to default |
| UI too complex | Start simple, iterate |
| Breaking existing behavior | Default to "use main agent model" = true |
| Performance impact | Lazy load model list |

---

## 6. Technical Notes & Related Files

### Files Affected
- **Agent core**: `crates/agent/src/agent.rs`, `crates/agent/src/thread.rs`
- **Orchestrator**: `crates/agent/src/orchestrator/*`
- **Settings & UI**: `crates/agent_settings/src/agent_settings.rs`, `crates/agent_ui/src/model_selector.rs`
- **Tools**: `crates/agent/src/tools/spawn_agent_tool.rs` and 15+ tool files affected by permission states (`terminal_tool.rs`, `edit_file_tool.rs`, etc.)
- **Prompts**: `crates/agent/src/templates/system_prompt.hbs`
- **Models**: `crates/language_model/src/language_model.rs`

### Build Requirements
- 16GB+ RAM is mandatory for a successful release build.
- Expected build time: ~15-20 minutes.
- **Next Step**: Clone `qdata-multi-agent-v1` branch, run `cargo build --release`, and execute the test prompts.
