# Implementation Plan: Zed Multi-Agent AI IDE Connector

This plan outlines the step-by-step process required to build the Universal AI Connector and Agent Orchestration Engine as a Zed extension, incorporating the architecture and gap fixes identified in the provided documentation.

## User Review Required
> [!IMPORTANT]
> **Git Configuration Action Required:** 
> I checked your git remote configuration (`git remote -v`). You have cloned directly from `https://github.com/zed-industries/zed.git`. Since you do not have direct push access to the main repository, **you must fork the `zed` repository** to your personal GitHub account and add it as a remote to push your branches.
> Run: `git remote rename origin upstream` and then `git remote add origin https://github.com/YOUR_USERNAME/zed.git`.

> [!NOTE]
> Please review this phased development plan and let me know if you approve this sequence, so we can begin execution.

## Proposed Changes

We will build the feature as an out-of-tree extension interacting with Zed via the Extension API.

### Phase 1: Setup & UI Skeleton (Steps 0 & 1)
- **Fork & Setup:** Switch to a personal fork as noted above.
- **Scaffold:** Create the `extension` and `control-plane` directories. Set up `.ai_memory` structure.
- **Extension UI:** Implement the Zed Agent Panel side-bar view, slash command registration (`/agent`, `/refactor`), and streaming UI text output using the Zed rust extension API framework.

### Phase 2: Context & State (Steps 2 & 3)
- **Context Builder:** Build logic to collect open files, terminal logs (redacted), and `git diff` states into a context envelope.
- **Persistent Memory:** Implement SQLite (with WAL) to store AI session states, history, and embeddings.
- **Session Continuity:** Add serialization (`thread_state.json`) to allow long-running chat threads to survive editor restarts.

### Phase 3: Universal AI Bridge & Orchestration (Steps 4 & 5)
- **Universal Provider Trait:** Adapt standard traits for connecting to `Anthropic`, `OpenAI`, `Ollama` (local), and high-context APIs (like Kimi) using Server-Sent Events (SSE).
- **Tool Protocol:** Implement tool execution wrappers (file read/write, git diff, ask user).
- **Agent Orchestrator:** Create the state machine for Architect -> Coder -> Auditor -> Tester workflows, complete with `ApprovalGate` interactive breakpoints for the user.

### Phase 4: Safety & Production Hardening (Steps 6 to 10)
- **Snapshots & Rollbacks:** Implement auto-snapshotting of `.ai_memory` and `git commit` wrappers that bundle metadata (cost, tokens, flow lock) after each approved AI patch.
- **Sandbox Execution:** Create the shell runner with resource limiters before any AI-generated commands are executed.
- **Metrics/Telemetry:** Track tokens and cost to a local DB budget to ensure APIs do not overspend.

## Verification Plan

### Automated Tests
- For the Control Plane logic, we will write Rust `cargo test` suites focusing heavily on the Memory, Validation, and Router layers.
- We will use the defined `MockAdapter` to dry-run provider workflows and verify the DAG orchestration state transitions seamlessly.

### Manual Verification
- **Build Verification:** Currently, the initial `cargo build` on the `zed` repository is running in the background to ensure your environment has all the requisite WebAssembly and Rust compilation targets ready.
- **UI Testing:** We will compile the extension, load it locally in your Zed editor, and manually invoke `/agent` to verify the sidebar interaction and token streaming responses.
