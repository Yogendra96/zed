# Zed Multi-Agent IDE Implementation Plan

**Version:** 1.0  
**Date:** 2026-04-14  
**Status:** Draft - Requires Validation  

---

## Executive Summary

This plan outlines building a multi-agent coding IDE by extending Zed with:
1. **Hindsight** memory integration
2. **Parallel multi-agent** support  
3. **Shared workspace state**

## Architecture Overview

```
Zed Core (Rust)
    │
    ├── agent crate (existing)
    │   ├── thread.rs - conversation handling
    │   ├── tools/*.rs - file editing, shell, etc.
    │   └── spawn_agent_tool.rs - sub-agent spawning
    │
    ├── NEW: hindsight_client crate
    │   └── MCP client for Hindsight memory
    │
    ├── NEW: shared_state crate  
    │   └── WorkspaceState for multi-agent
    │
    └── NEW: orchestrator crate
        └── Supervisor + parallel execution
```

## What Zed Already Has

### EXISTING WORK (Uncommitted!) 
- `context_builder/` crate - Builds context from open files + git diffs
- `agent-panel/` extension - 4 built-in agents with slash commands

### Built-in Agents (Ready!)
- `/explain` - Explains code
- `/test` - Generates tests  
- `/audit` - Security audit
- `/refactor` - Code refactoring

### Official Agent Crates (Ready to use)
- `agent` - Core agent implementation (3126 lines)
- `agent_ui` - Agent panel, diff view, model selector
- `agent_settings` - Profile configuration
- `agent_servers` - Server management
- `language_models` - Multi-provider LLM (Anthropic, OpenAI, Gemini, Ollama, Groq)
- `copilot_chat` - GitHub Copilot integration

### Available Tools
- edit_file_tool.rs
- read_file_tool.rs
- terminal_tool.rs
- spawn_agent_tool.rs (already supports parallel delegation!)
- web_search_tool.rs
- grep_tool.rs
- find_path_tool.rs

### Spawn Agent Tool (Already supports parallel!)
From `spawn_agent_tool.rs`:
```
- Run multiple independent information-seeking subtasks in parallel
- Split implementation into disjoint codebase slices  
- When a plan has multiple independent steps, prefer delegating 
  those steps in parallel rather than serializing them
```

## What's Missing (Gap Analysis)

| Requirement | Zed Status | Work Needed |
|-------------|-----------|--------------|
| Security Agent | Tools exist | Agent wrapper |
| Architect Agent | Tools exist | Agent wrapper |
| Parallel Agents | Sub-agents exist | Only sequential |
| Shared Workspace | Per-thread only | New crate |
| LangGraph | None | Custom Rust impl |
| Hindsight Memory | None | MCP client |

## Implementation Proposal

### Phase 1: Foundation (Week 1-2)

#### Goals
- Fork Zed to QData-Octo ✅ DONE
- Set up Hindsight (NO DOCKER needed!)
- Create basic recall/store tool

#### Tasks

**Week 1:**
1. [x] Research Hindsight options (3 options found)
   - ✅ Option A: Hindsight Cloud (RECOMMENDED)
   - ✅ Option B: Python embedded (hindsight-all)
   - ✅ Option C: Rust MCP server

2. [ ] Decide Hindsight approach
   - **Chosen:** Option A - Hindsight Cloud + MCP
   - Sign up at https://hindsight.vectorize.io
   - Connect via MCP protocol (like Cursor uses)
   
2. [ ] Build `HindsightTool` for agent
   - Add as available tool in agent
   - Test single-agent memory

**Week 2:**
3. [ ] Create `ZedAgentState` struct
   - Define state schema
   - Add to thread state
   
4. [ ] Test: Single agent with memory
   - Recall past context
   - Store learned facts

**Deliverable:** Single agent with Hindsight memory

### Phase 2: Multi-Agent Core (Week 3-4)

#### Goals
- Parallel agent execution
- Shared workspace state
- Inter-agent communication

#### Tasks

**Week 3:**
1. [ ] Implement `WorkspaceState` struct
   - Shared file content
   - Pending diffs queue
   - Agent message bus
   
2. [ ] Extend spawn_agent_tool
   - Add parallel spawn mode
   - Add session_id reuse
   
3. [ ] Build Supervisor logic
   - Determine which agents to spawn
   - Route tasks to agents

**Week 4:**
4. [ ] Create Synthesis node
   - Merge agent findings
   - Conflict resolution
   
5. [ ] Implement diff queuing
   - Queue changes from agents
   - Apply in order

**Deliverable:** Multiple agents run in parallel

### Phase 3: UI (Week 5-6)

#### Goals  
- Agent crew management UI
- Visual state explorer

#### Tasks

**Week 5:**
1. [ ] Build MultiAgentPanel
2. [ ] Implement CrewView
   
**Week 6:**
3. [ ] Build SharedWorkspaceView
4. [ ] Add diff visualization

**Deliverable:** Full agent crew UI

### Phase 4: Advanced (Week 7-8)

#### Goals
- Intelligence features
- Polish

#### Tasks
1. Belief consistency checks
2. Performance metrics
3. Dynamic agents
4. Security audit

## Technical Decisions Needed

### 1. Hindsight Integration Approach

**Option A: Docker + MCP**
- Run Hindsight as Docker container
- Connect via MCP protocol
- PRO: Standard approach
- CON: Needs Docker

**Option B: Library (No Docker)**
- Study Hindsight's Rust API
- Call directly from crate
- PRO: No Docker needed
- CON: More implementation work

**Recommendation:** Start with Option B, fallback to A

### 2. State Management

**Option A: Custom Rust StateGraph**
- Build custom state machine
- Similar to LangGraph but Rust
- PRO: Full control
- CON: More code

**Option B: Extend existing thread state**
- Share state via Entity system
- Add to AgentHandle
- PRO: Less code
- CON: Less flexible

**Recommendation:** Start with Option B

### 3. Parallel Execution

Zed's spawn_agent_tool already supports:
- Multiple independent tasks in parallel
- Session reuse for follow-up
- Disjoint write scope spawning

**Plan:** Extend, don't rebuild

## Open Questions

1. **Docker required?** - Can we integrate Hindsight as library?
2. **State granularity** - How fine-grained should shared state be?
3. **Conflict resolution** - How to handle conflicting agent suggestions?
4. **Distribution** - How to ship custom Zed fork?

## Risks & Mitigations

| Risk | Impact | Mitigation |
|------|--------|-------------|
| Rust expertise needed | High | Hire contract dev |
| Build times | Medium | Use cargo check |
| Hindsight complexity | Medium | Start simple |

## Validation Checklist

Before Phase 1:

- [ ] Confirm Zed builds and runs ✅ (done)
- [ ] Confirm agent tools work via UI
- [ ] Verify spawn_agent_tool parallel capabilities
- [ ] Decide Hindsight approach (library vs Docker)
- [ ] Estimate Hindsight integration effort
- [ ] Identify Rust expertise needs

## Next Steps (Requires Validation)

1. ✅ Zed builds/runs - CONFIRMED
2. [ ] Test agent via UI
3. [ ] Validate spawn_agent parallel docs  
4. [ ] Choose Hindsight approach
5. [ ] Estimate Phase 1 timeline

---

**Once validated, proceed to Phase 1 implementation**

## References

- Existing Zed agent: `crates/agent/src/agent.rs`
- Spawn tool: `crates/agent/src/tools/spawn_agent_tool.rs`
- Thread: `crates/agent/src/thread.rs`
- This plan: `docs/multi_agent_plan.md`