# Zed Multi-Agent System Implementation Plan
## Provider-Agnostic Parallel Agent Orchestration

**Version:** 1.0  
**Date:** 2026-04-14  
**Status:** Ready for Implementation

---

## Executive Summary

Build a provider-agnostic multi-agent system in Zed that works with **any LLM provider** (OpenRouter, Anthropic, OpenAI, Ollama, etc.). The system leverages Zed's built-in agent infrastructure with parallel execution, shared workspace state, and result synthesis.

---

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                     ZED CORE (EXISTING)                      │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│   Provider Layer (Agnostic)                                 │
│   ┌─────────┐ ┌──────────┐ ┌─────────┐ ┌─────────┐         │
│   │OpenRouter│ │Anthropic │ │ OpenAI  │ │ Ollama  │         │
│   └─────────┘ └──────────┘ └─────────┘ └─────────┘         │
│         │           │            │           │               │
│         └───────────┴────────────┴───────────┘               │
│                         │                                     │
│                         ▼                                     │
│   ┌─────────────────────────────────────────────────────┐    │
│   │            Agent Tool Layer (Built-in)              │    │
│   │  - spawn_agent      - read_file                     │    │
│   │  - edit_file        - terminal                       │    │
│   │  - grep             - web_search                    │    │
│   └─────────────────────────────────────────────────────┘    │
│                                                              │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│              NEW: Multi-Agent Orchestrator                   │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│   ┌────────────────┐    ┌─────────────────┐                  │
│   │  Supervisor   │───▶│ Parallel Agent │                  │
│   │    Node       │    │    Spawner     │                  │
│   └───────┬────────┘    └────────┬────────┘                  │
│           │                      │                            │
│           │        ┌────────────┼────────────┐              │
│           │        ▼            ▼            ▼              │
│           │   ┌────────┐  ┌──────────┐  ┌────────┐          │
│           │   │ Agent 1│  │ Agent 2  │  │ Agent 3│          │
│           │   └────┬───┘  └────┬─────┘  └────┬───┘          │
│           │        │           │            │                │
│           │        └───────────┼────────────┘                │
│           │                    ▼                             │
│           │        ┌─────────────────────┐                    │
│           │        │  Shared Workspace  │                    │
│           │        │  - File State       │                    │
│           │        │  - Agent Messages   │                    │
│           │        │  - Diff Queue       │                    │
│           │        └──────────┬──────────┘                    │
│           │                   ▼                              │
│           │        ┌─────────────────────┐                    │
│           │        │  Synthesis Node    │                    │
│           │        │  (Merge Results)    │                    │
│           │        └─────────────────────┘                    │
│           │                                                   │
│           └────────────────┬────────────────────────────────┤
│                            ▼                                  │
│                    ┌─────────────────┐                        │
│                    │  Final Output   │                        │
│                    │  to User        │                        │
│                    └─────────────────┘                        │
└─────────────────────────────────────────────────────────────┘
```

---

## Implementation Phases

### Phase 1: Supervisor Agent (Week 1)

**Goal:** Create a supervisor that decides when to spawn parallel agents

**Tasks:**
1. [ ] Create `supervisor.rs` - Analyzes user requests
2. [ ] Detect parallelizable tasks from natural language
3. [ ] Generate agent configurations dynamically
4. [ ] Route tasks to spawn_agent tool

**Location:** `crates/agent/src/orchestrator/supervisor.rs`

### Phase 2: Shared Workspace State (Week 2)

**Goal:** Enable agents to share state and communicate

**Tasks:**
1. [ ] Create `workspace_state.rs` - Shared state manager
2. [ ] Implement message bus for inter-agent communication
3. [ ] Build diff queue for change aggregation
4. [ ] Add state persistence

**Location:** `crates/agent/src/orchestrator/workspace.rs`

### Phase 3: Synthesis Node (Week 3)

**Goal:** Combine results from multiple agents

**Tasks:**
1. [ ] Create `synthesis.rs` - Merge agent outputs
2. [ ] Handle conflicting suggestions
3. [ ] Generate unified response
4. [ ] Apply diffs to workspace

**Location:** `crates/agent/src/orchestrator/synthesis.rs`

### Phase 4: UI Integration (Week 4)

**Goal:** Make it visible in Zed's UI

**Tasks:**
1. [ ] Add agent status panel
2. [ ] Show parallel agent progress
3. [ ] Display workspace state viewer
4. [ ] Add controls for agent spawning

**Location:** `crates/agent_ui/src/`

---

## Technical Details

### Supervisor Logic
```rust
pub fn analyze_task(task: &str) -> SupervisorDecision {
    // 1. Parse task for parallel hints
    // 2. Check if task is "AND" decomposable
    // 3. Return spawn config
}
```

### Workspace State
```rust
pub struct SharedWorkspace {
    pub files: HashMap<PathBuf, FileState>,
    pub messages: Vec<AgentMessage>,
    pub diffs: Vec<PendingDiff>,
}
```

### Agent Spawning
```rust
// Built-in spawn_agent tool already supports parallel!
SpawnAgentToolInput {
    label: "security".to_string(),
    message: "scan for vulnerabilities".to_string(),
    // session_id for follow-up
}
```

---

## Files to Create/Modify

### New Files
- `crates/agent/src/orchestrator/mod.rs`
- `crates/agent/src/orchestrator/supervisor.rs`
- `crates/agent/src/orchestrator/workspace.rs`
- `crates/agent/src/orchestrator/synthesis.rs`

### Modify
- `crates/agent/src/agent.rs` - Register orchestrator
- `crates/agent/src/thread.rs` - Add shared state to threads

---

## Testing Strategy

1. **Unit Tests** - Each component independently
2. **Integration Tests** - Supervisor → Spawn flow
3. **E2E Tests** - Full parallel agent execution

---

## Success Criteria

1. Supervisor correctly identifies parallelizable tasks
2. Multiple agents run in parallel (tested with 3+ agents)
3. Shared workspace allows agent communication
4. Synthesis merges results correctly
5. UI shows agent status

---

## Dependencies

- Built-in: `spawn_agent_tool`, `agent`, `agent_ui`
- External: None required (all internal)

---

## Future Enhancements

- Memory integration (Hindsight)
- Dynamic agent templates
- Conversation export/import
- Agent performance metrics