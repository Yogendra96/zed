# Subagent Model Configuration - Implementation Plan

## Overview

Allow users to configure which model/subagent uses, with a UI in Zed settings. Subagents inherit from user's currently selected main agent model, but user can override per-task-type or globally.

---

## User Experience

### Scenario 1: Default Behavior
- User selects "Claude Sonnet" as main agent
- When spawning subagent → uses "Claude Sonnet" (inherits from main)
- User sees: "Using model: Claude Sonnet (inherited from main)"

### Scenario 2: Override with Preset
- User sets preset: "Security = Claude Opus", "Quality = GPT-4o"
- Spawn "security scan" subagent → uses "Claude Opus"
- Spawn "code quality" subagent → uses "GPT-4o"
- User sees: "Using model: Claude Opus (security preset)"

### Scenario 3: Custom Override
- User explicitly sets "Always use: Gemini 2.0" for all subagents
- Any subagent spawned → uses "Gemini 2.0"
- User sees: "Using model: Gemini 2.0 (override)"

---

## Data Model

### Settings Schema

```json
// In settings.json
"agent": {
  "subagent": {
    // Global default: use main agent's model
    "use_main_agent_model": true,
    
    // Override: specific model for all subagents
    "default_model": {
      "provider": "copilot_chat",
      "model": "gpt-4o"
    },
    
    // Per-task-type overrides (optional)
    "task_overrides": {
      "security": {
        "provider": "anthropic",
        "model": "claude-opus-4-5"
      },
      "performance": {
        "provider": "openrouter",
        "model": "openai/gpt-4o"
      },
      "quality": {
        "provider": "copilot_chat", 
        "model": "gpt-4o"
      },
      "test": {
        "provider": "anthropic",
        "model": "claude-sonnet-4-20250514"
      },
      "explain": {
        "provider": "openrouter", 
        "model": "deepseek/deepseek-chat"
      }
    }
  }
}
```

### Task Type Detection
Match user prompts to task types via keyword detection:
- **security** → "security", "vulnerability", "audit", "scan", "hack", "exploit"
- **performance** → "performance", "bottleneck", "optimize", "slow", "profile"
- **quality** → "quality", "code smell", "refactor", "improve", "lint"
- **test** → "test", "testing", "coverage", "unittest", "spec"
- **explain** → "explain", "document", "describe", "what does"
- **default** → fallback to main agent model or default_model

---

## UI Design

### Location
Settings → Agent → "Subagent" section

### UI Components

#### 1. Main Toggle
- "Use main agent's model for subagents"
- Toggle: ON (default) / OFF
- When ON: shows inherited model name
- When OFF: enables model selector below

#### 2. Model Selector (when toggle OFF)
- Dropdown: Provider selection
- Dropdown: Model selection based on provider
- "Test Connection" button

#### 3. Task-Type Overrides Section
- Collapsible panel "Override by task type"
- Table/grid showing:
  - Task Type | Model | Provider | Action
  - Security | Claude Opus | Anthropic | [Edit] [Clear]
  - Performance | GPT-4o | OpenRouter | [Edit] [Clear]
- "Add Override" button → opens model selector modal

#### 4. Presets (Future)
- "Save current config as preset"
- "Load preset" dropdown

---

## File Changes Required

### 1. Settings Definition
**File:** `crates/agent_settings/src/agent_settings.rs`

Add:
```rust
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SubagentSettings {
    pub use_main_agent_model: bool,
    pub default_model: Option<LanguageModelSelector>,
    pub task_overrides: HashMap<String, LanguageModelSelector>,
}
```

### 2. Settings UI
**File:** `crates/agent_settings/src/agent_settings_ui.rs` or new file

Add UI components for:
- Main toggle
- Model selector (re-use existing model picker)
- Task override table with add/edit/clear

### 3. Subagent Spawn Logic
**File:** `crates/agent/src/agent.rs` or `crates/agent/src/tools/spawn_agent_tool.rs`

Modify `create_subagent_thread`:
```rust
fn get_model_for_subagent(
    task_label: &str,
    subagent_settings: &SubagentSettings,
    main_agent_model: &LanguageModelSelector,
) -> LanguageModelSelector {
    // 1. Check task-specific override
    if let Some(override) = subagent_settings.task_overrides.get(task_label) {
        return override.clone();
    }
    
    // 2. Check global default
    if let Some(default) = &subagent_settings.default_model {
        return default.clone();
    }
    
    // 3. Fallback: use main agent's model
    if subagent_settings.use_main_agent_model {
        return main_agent_model.clone();
    }
    
    // 4. Ultimate fallback
    return get_default_model();
}
```

### 4. Task Label Detection
**File:** `crates/agent/src/orchestrator/supervisor.rs` (existing) or new

```rust
fn detect_task_type(prompt: &str) -> TaskType {
    let lower = prompt.to_lowercase();
    
    let keywords = [
        (TaskType::Security, &["security", "vulnerabilities", "scan", "audit", "hack", "exploit"][..]),
        (TaskType::Performance, &["performance", "bottleneck", "optimize", "slow", "profile"][..]),
        (TaskType::Quality, &["quality", "code smell", "refactor", "improve", "lint"][..]),
        (TaskType::Test, &["test", "testing", "coverage", "unittest", "spec"][..]),
        (TaskType::Explain, &["explain", "document", "describe", "what does"][..]),
    ];
    
    for (task_type, tags) in keywords {
        if tags.iter().any(|tag| lower.contains(tag)) {
            return task_type;
        }
    }
    
    TaskType::Default
}
```

### 5. Thread Creation Update
**File:** `crates/agent/src/thread.rs`

Update `Thread::new_subagent` to accept model parameter:
```rust
pub fn new_subagent(
    parent: &Entity<Thread>,
    model: LanguageModelSelector,
    cx: &mut App
) -> Entity<Thread>
```

### 6. Logs/Display
**File:** `crates/agent/src/tools/spawn_agent_tool.rs`

Update logging:
```rust
log::info!(
    "[orchestrator] Subagent spawned: label={}, session_id={}, model={}/{}",
    input.label,
    subagent.id(),
    selected_model.provider,
    selected_model.model
);
```

---

## Implementation Steps

### Phase 1: Settings & Schema (Priority: HIGH)
1. Add `SubagentSettings` struct to agent_settings.rs
2. Register in settings system
3. Add to default.json schema
4. Test: verify settings save/load

### Phase 2: UI (Priority: HIGH)
1. Create subagent settings panel in settings UI
2. Add toggle for "use main agent model"
3. Add model selector component
4. Add task override table
5. Test: verify UI renders and saves

### Phase 3: Integration (Priority: HIGH)
1. Modify spawn_agent_tool.rs to read settings
2. Implement task type detection
3. Implement model selection logic
4. Pass model to thread creation
5. Test: spawn subagent with different configs

### Phase 4: Polish (Priority: MEDIUM)
1. Add "current model" indicator in subagent card
2. Add logs showing which model used
3. Error handling for invalid models
4. Test: edge cases

---

## Testing Plan

### Test Cases
1. Default: subagent uses main agent's model
2. Override all: subagent uses specified model for all tasks
3. Task override: "security scan" uses security model, "test" uses test model
4. UI: toggle saves, model selector works, task override table works
5. Invalid model: graceful fallback to default
6. No config: uses main agent model (current behavior)

### Manual Test Prompts
```
1. "Run security scan AND code quality in parallel" 
   → should show different models per task

2. "Analyze this code"
   → should use default model

3. "Run performance analysis on main.py"
   → should use performance model if configured
```

---

## Risk Assessment

| Risk | Mitigation |
|------|------------|
| Model not available for provider | Add validation, fallback to default |
| UI too complex | Start simple, iterate |
| Breaking existing behavior | Default to "use main agent model" = true |
| Performance impact | Lazy load model list |

---

## Timeline Estimate

- **Phase 1**: 2-3 hours
- **Phase 2**: 3-4 hours  
- **Phase 3**: 2-3 hours
- **Phase 4**: 1-2 hours

**Total**: ~8-12 hours development

---

## Open Questions

1. Should there be a limit on max subagent depth? (already exists: MAX_SUBAGENT_DEPTH=5)
2. Should we show cost estimates per subagent?
3. Should subagent history be separate or shared with main thread?
4. How to handle API key issues for subagent models (different from main)?

---

## Related Files (Read First)

- `crates/agent_settings/src/agent_settings.rs` - Settings patterns
- `crates/agent/src/tools/spawn_agent_tool.rs` - Subagent spawn
- `crates/agent/src/agent.rs` - create_subagent_thread
- `crates/agent_ui/src/model_selector.rs` - Model picker UI
- `crates/language_model/src/language_model.rs` - Model types