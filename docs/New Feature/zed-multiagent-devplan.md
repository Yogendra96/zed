# Zed Multi-Agent AI IDE — Full Development Plan
## ASCII Architecture · Step-by-Step Build · Auto-Recovery & Rollback (Canonical)

---

## MASTER ARCHITECTURE OVERVIEW

```
╔══════════════════════════════════════════════════════════════════════════════════════════════╗
║                           ZED MULTI-AGENT AI IDE — SYSTEM TOPOLOGY                         ║
╠══════════════════════════════════════════════════════════════════════════════════════════════╣
║                                                                                              ║
║  ┌────────────────────────────────────────────────────────────────────────────────────────┐  ║
║  │                             LAYER 1 — ZED HOST (IDE)                                   │  ║
║  │   Fast Rust editor · terminal · filesystem · project tree · native UI                  │  ║
║  │   [ NO heavy AI logic here — keeps editor stable and extension sandboxed ]              │  ║
║  └──────────────────────────────────┬─────────────────────────────────────────────────────┘  ║
║                                     │  Extension API (Rust/WASM manifest)                   ║
║                                     ▼                                                        ║
║  ┌────────────────────────────────────────────────────────────────────────────────────────┐  ║
║  │                          LAYER 2 — ZED EXTENSION (Thin Client)                         │  ║
║  │                                                                                        │  ║
║  │  ┌─────────────────┐  ┌──────────────────┐  ┌───────────────────┐  ┌───────────────┐  │  ║
║  │  │  UI Agent Panel  │  │  Command Router  │  │  Context Builder  │  │  Token Mgr    │  │  ║
║  │  │  - Agent chat    │  │  - slash cmds    │  │  - open files     │  │  - OS keychain│  │  ║
║  │  │  - Role switcher │  │  - action map    │  │  - git diff       │  │  - encrypted  │  │  ║
║  │  │  - Diff viewer   │  │  - validation    │  │  - terminal logs  │  │    store      │  │  ║
║  │  │  - Status bar    │  │  - error handler │  │  - project meta   │  │  - no commits │  │  ║
║  │  └────────┬─────────┘  └───────┬──────────┘  └────────┬──────────┘  └───────┬───────┘  │  ║
║  │           └──────────────────┬─┘                       │                    │           │  ║
║  │                              ▼                          ▼                    │           │  ║
║  │                   ┌──────────────────────────────────────────────────────┐   │           │  ║
║  │                   │             LOCAL PERSISTENT MEMORY (.ai_memory/)    │   │           │  ║
║  │                   │                                                      │   │           │  ║
║  │                   │   metadata.sqlite ─── embedding_cache.db            │   │           │  ║
║  │                   │   flow.lock        ─── session_state.json           │   │           │  ║
║  │                   │   snapshots/       ─── redaction_log.txt            │   │           │  ║
║  │                   │                                                      │   │           │  ║
║  │                   │  [ NEVER commit secrets · WAL mode · encrypted ]    │   │           │  ║
║  │                   └───────────────────────┬──────────────────────────────┘   │           │  ║
║  │                                           │ Secure RPC/REST/WebSocket         │           │  ║
║  └───────────────────────────────────────────┼──────────────────────────────────┘           ║
║                                              │                                               ║
║                                              ▼                                               ║
║  ┌────────────────────────────────────────────────────────────────────────────────────────┐  ║
║  │                     LAYER 3 — AGENT CONTROL PLANE (External Service)                   │  ║
║  │                                                                                        │  ║
║  │  ┌──────────────────────────────────────────────────────────────────────────────────┐  │  ║
║  │  │                        AGENT ORCHESTRATION ENGINE                                │  │  ║
║  │  │                                                                                  │  │  ║
║  │  │  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐  ┌──────────────────────┐ │  │  ║
║  │  │  │  LangGraph   │  │  OpenClaw /  │  │  Temporal.io │  │  Agent Role Registry │ │  │  ║
║  │  │  │  Deterministic│  │  NanoClaw   │  │  Durable WF  │  │  - Architect         │ │  │  ║
║  │  │  │  workflows   │  │  Swarm agent │  │  Retry/ckpt  │  │  - Coder             │ │  │  ║
║  │  │  └──────────────┘  └──────────────┘  └──────────────┘  │  - Auditor           │ │  │  ║
║  │  │                                                          │  - Tester            │ │  │  ║
║  │  │                                                          │  - Prompter          │ │  │  ║
║  │  │                                                          └──────────────────────┘ │  │  ║
║  │  └─────────────────────────────────────┬────────────────────────────────────────────┘  │  ║
║  │                                        │                                               │  ║
║  │                                        ▼                                               │  ║
║  │  ┌──────────────────────────────────────────────────────────────────────────────────┐  │  ║
║  │  │                          TOOL EXECUTION LAYER                                    │  │  ║
║  │  │                                                                                  │  │  ║
║  │  │  ┌─────────────┐ ┌──────────────┐ ┌────────────────┐ ┌──────────────────────┐   │  │  ║
║  │  │  │  MCP Servers │ │  Code Interp │ │  Git CLI Wrap  │ │  Sandbox Runner      │   │  │  ║
║  │  │  │  (connectors)│ │  (sandboxed) │ │  libgit2/CLI   │ │  container/firejail  │   │  │  ║
║  │  │  └─────────────┘ └──────────────┘ └────────────────┘ └──────────────────────┘   │  │  ║
║  │  └─────────────────────────────────────┬────────────────────────────────────────────┘  │  ║
║  │                                        │                                               │  ║
║  │                                        ▼                                               │  ║
║  │  ┌──────────────────────────────────────────────────────────────────────────────────┐  │  ║
║  │  │                        PROVIDER ABSTRACTION LAYER                                │  │  ║
║  │  │                                                                                  │  │  ║
║  │  │  ┌──────────┐ ┌───────────┐ ┌──────┐ ┌──────────┐ ┌────────┐ ┌──────────────┐  │  │  ║
║  │  │  │ Anthropic │ │  OpenAI  │ │ Groq │ │ DeepSeek │ │ Ollama │ │ Mock Provider │  │  │  ║
║  │  │  │ (Claude)  │ │  (GPT-4) │ │      │ │          │ │ (local)│ │ (for tests)   │  │  │  ║
║  │  │  └──────────┘ └───────────┘ └──────┘ └──────────┘ └────────┘ └──────────────┘  │  │  ║
║  │  │                                                                                  │  │  ║
║  │  │  Cross-cutting: cost tracker · rate limiter · fallback router · safety filters   │  │  ║
║  │  └─────────────────────────────────────┬────────────────────────────────────────────┘  │  ║
║  │                                        │                                               │  ║
║  │                                        ▼                                               │  ║
║  │  ┌──────────────────────────────────────────────────────────────────────────────────┐  │  ║
║  │  │                             MEMORY SYSTEM                                        │  │  ║
║  │  │                                                                                  │  │  ║
║  │  │   LOCAL                          REMOTE                                          │  │  ║
║  │  │   ─────                          ──────                                          │  │  ║
║  │  │   SQLite + sqlite_vss            Qdrant / Weaviate / Postgres+pgvector           │  │  ║
║  │  │   embedding cache                tree-sitter / ast-grep (code context)           │  │  ║
║  │  │   flow.lock snapshots            Sourcegraph Cody embeddings (optional)          │  │  ║
║  │  └──────────────────────────────────────────────────────────────────────────────────┘  │  ║
║  └────────────────────────────────────────────────────────────────────────────────────────┘  ║
║                                                                                              ║
╠══════════════════════════════════════════════════════════════════════════════════════════════╣
║  INFRASTRUCTURE LAYER                                                                        ║
║                                                                                              ║
║  Dev Env: Nix / Devcontainers / Docker   Workflow: Temporal / Prefect / Dagster             ║
║  CI/CD: GitHub Actions + secret scanner  Observability: OpenTelemetry + cost dashboard      ║
╚══════════════════════════════════════════════════════════════════════════════════════════════╝
```

---

## AGENT WORKFLOW SEQUENCE DIAGRAM

```
┌────────┐  ┌──────────────┐  ┌────────────────┐  ┌──────────────────┐  ┌──────────────┐
│  USER  │  │ ZED EXTENSION │  │ CONTROL PLANE  │  │  AGENT PIPELINE  │  │  MEMORY/GIT  │
└───┬────┘  └──────┬───────┘  └───────┬────────┘  └────────┬─────────┘  └──────┬───────┘
    │               │                  │                    │                   │
    │ /refactor X   │                  │                    │                   │
    ├──────────────>│                  │                    │                   │
    │               │ build context    │                    │                   │
    │               │ (files+diff+logs)│                    │                   │
    │               ├─ load memory ───────────────────────────────────────────>│
    │               │<─ prior context ────────────────────────────────────────-│
    │               │                  │                    │                   │
    │               │ POST /workflow   │                    │                   │
    │               ├─────────────────>│                    │                   │
    │               │                  │ spawn agents       │                   │
    │               │                  ├───────────────────>│                   │
    │               │                  │                    │ Architect agent   │
    │               │                  │                    ├──────────────┐    │
    │               │                  │                    │ plan created │    │
    │               │                  │                    │<─────────────┘    │
    │               │                  │                    │ Coder agent       │
    │               │                  │                    ├──────────────┐    │
    │               │                  │                    │ patch created│    │
    │               │                  │                    │<─────────────┘    │
    │               │                  │                    │ Auditor agent     │
    │               │                  │                    ├──────────────┐    │
    │               │                  │                    │ review passed│    │
    │               │                  │                    │<─────────────┘    │
    │               │                  │                    │ Tester agent      │
    │               │                  │                    ├──────────────┐    │
    │               │                  │                    │ tests passed │    │
    │               │                  │                    │<─────────────┘    │
    │               │                  │<── patch + report ─┤                   │
    │               │<── diff result ──┤                    │                   │
    │               │                  │                    │                   │
    │<── show diff ─┤                  │                    │                   │
    │               │                  │                    │                   │
    │ approve/reject│                  │                    │                   │
    ├──────────────>│                  │                    │                   │
    │               │ commit snapshot ──────────────────────────────────────── >│
    │               │ update memory ─────────────────────────────────────────->│
    │               │                  │                    │                   │
    │<── done ──────┤                  │                    │                   │
└───┴────┘  └──────┴───────┘  └───────┴────────┘  └────────┴─────────┘  └──────┴───────┘
```

---

## AUTO-RECOVERY & ROLLBACK POLICY (CANONICAL — APPLIES TO EVERY STEP)

```
╔═══════════════════════════════════════════════════════════════════════════╗
║             MANDATORY RECOVERY FRAMEWORK — ALL STEPS                     ║
╠═══════════════════════════════════════════════════════════════════════════╣
║                                                                           ║
║  BEFORE adding any function:                                              ║
║  ─────────────────────────────                                            ║
║  1. git commit --message "pre: <step-name> baseline"                      ║
║  2. snapshot .ai_memory/ → .ai_memory/snapshots/<timestamp>/              ║
║  3. record flow.lock state                                                ║
║                                                                           ║
║  AFTER adding any function:                                               ║
║  ─────────────────────────────                                            ║
║  1. run unit test suite for that function (must pass ≥ 95%)               ║
║  2. run integration smoke test                                            ║
║  3. verify no secrets in staged changes (pre-commit hook)                 ║
║  4. only then: git commit --message "feat: <function-name> [tested]"      ║
║                                                                           ║
║  ROLLBACK TRIGGERS:                                                       ║
║  ─────────────────────────────                                            ║
║  • Unit test failure → git reset --hard HEAD^ + restore snapshot         ║
║  • Integration failure → same as above                                    ║
║  • Secret detected in commit → block push + alert + reset                ║
║  • Memory corruption detected → restore from .ai_memory/snapshots/       ║
║  • Provider call failure > 3 retries → switch to mock + alert             ║
║  • Cost budget exceeded → pause all provider calls + alert                ║
║                                                                           ║
║  ROLLBACK COMMAND (templated):                                            ║
║  ─────────────────────────────                                            ║
║  git reset --hard <pre-step-commit-sha>                                   ║
║  cp -r .ai_memory/snapshots/<timestamp>/ .ai_memory/                     ║
║  cargo test --all                          # verify clean state           ║
║                                                                           ║
╚═══════════════════════════════════════════════════════════════════════════╝
```

---

## STEP-BY-STEP DEVELOPMENT PLAN

---

### STEP 0 — PROJECT SCAFFOLD & TOOLCHAIN LOCK

**Goal:** Establish a reproducible baseline before any code is written.

```
STEP 0 MODULE LAYOUT
─────────────────────────────────────────────────────────────────
  /
  ├── extension/                  ← Zed Extension (Rust crate)
  │   ├── Cargo.toml
  │   ├── extension.toml          ← Zed manifest
  │   └── src/
  │       └── lib.rs              ← entry point only (empty)
  │
  ├── control-plane/              ← External Agent Service
  │   ├── pyproject.toml / Cargo.toml
  │   └── src/
  │
  ├── .ai_memory/                 ← Local persistent memory
  │   ├── metadata.sqlite         ← WAL mode, encrypted
  │   ├── embedding_cache.db
  │   ├── flow.lock               ← toolchain snapshot
  │   ├── session_state.json
  │   └── snapshots/              ← rollback targets
  │
  ├── .gitignore                  ← includes *.secrets, .env*, tokens
  ├── .pre-commit-config.yaml     ← gitleaks / trufflehog
  ├── devcontainer.json           ← reproducible env
  └── flow.lock                   ← root toolchain version lock
─────────────────────────────────────────────────────────────────
```

**Features & Functions:**

| # | Function | Description |
|---|----------|-------------|
| 0.1 | `init_project_scaffold()` | Create directory structure, Cargo workspaces, extension.toml |
| 0.2 | `lock_toolchain()` | Write flow.lock: rustc, cargo, node, python, zed extension API version |
| 0.3 | `configure_gitignore()` | Block .env, *.pem, *secret*, ai_memory/secrets/ from git |
| 0.4 | `install_precommit_hooks()` | gitleaks + trufflehog scan on every commit |
| 0.5 | `create_devcontainer()` | devcontainer.json + Dockerfile for reproducible builds |
| 0.6 | `init_baseline_snapshot()` | First .ai_memory/snapshots/000-baseline/ checkpoint |
| 0.7 | `run_scaffold_tests()` | Verify build compiles, hooks fire, snapshot exists |

**Recovery & Rollback:**
```
AUTO-RECOVERY — STEP 0
────────────────────────────────────────────────────────
BEFORE: N/A (this is the baseline)
AFTER function 0.4: run `pre-commit run --all-files`
  → PASS: continue to 0.5
  → FAIL: fix hook config, re-run, do not proceed
AFTER function 0.7: cargo test + `git log` must show clean commit
  → FAIL: delete repo, re-scaffold from scratch
ROLLBACK TARGET: fresh directory (no prior state)
────────────────────────────────────────────────────────
```

---

### STEP 1 — ZED EXTENSION SKELETON (UI AGENT PANEL)

**Goal:** A working Zed extension that loads, shows an Agent Panel, and handles slash commands.

```
STEP 1 COMPONENT DIAGRAM
─────────────────────────────────────────────────────────────────
  extension/src/
  ├── lib.rs              ← Extension entry (ZedExtension impl)
  ├── panel.rs            ← Agent Panel UI
  ├── commands.rs         ← Slash command registry
  └── config.rs           ← agents.yaml reader / validator

  extension.toml
  ├── [extension]
  │   name = "multi-agent-ide"
  │   version = "0.1.0"
  ├── [[slash_commands]]
  │   name = "agent"
  │   description = "Invoke agent role"
  └── [[slash_commands]]
      name = "refactor"
      description = "Trigger refactor workflow"
─────────────────────────────────────────────────────────────────
```

**Features & Functions:**

| # | Function | Description |
|---|----------|-------------|
| 1.1 | `register_extension()` | Implement ZedExtension trait; register with host |
| 1.2 | `render_agent_panel()` | Sidebar panel: agent chat, role badge, status bar |
| 1.3 | `register_slash_commands()` | Register /agent, /refactor, /audit, /test, /rollback |
| 1.4 | `parse_agents_yaml()` | Read workspace agents.yaml; validate role definitions |
| 1.5 | `handle_slash_command(cmd, args)` | Dispatch commands to internal router |
| 1.6 | `display_diff_view(patch)` | Show AI-suggested diff in panel before user approval |
| 1.7 | `update_status_bar(state)` | Show current agent role + session status in Zed statusbar |

**Recovery & Rollback:**
```
AUTO-RECOVERY — STEP 1
────────────────────────────────────────────────────────
BEFORE STEP 1: git commit "pre: step-1 extension skeleton"
                snapshot .ai_memory/ → snapshots/001-pre-step1/

AFTER 1.1: `cargo build -p extension` must succeed with 0 errors
  → FAIL: revert lib.rs, check extension.toml manifest version

AFTER 1.3: install extension in Zed, type /agent → must show in command palette
  → FAIL: check extension API version in flow.lock matches Zed binary

AFTER 1.7: Full panel renders without Zed crash
  → FAIL: git reset --hard <pre-step1-sha>
           restore snapshots/001-pre-step1/

ROLLBACK COMMAND:
  git reset --hard $(cat .ai_memory/snapshots/001-pre-step1/commit.sha)
────────────────────────────────────────────────────────
```

---

### STEP 2 — COMMAND ROUTER & CONTEXT BUILDER

**Goal:** Route slash commands to the right workflow; collect rich project context before each call.

```
STEP 2 COMPONENT DIAGRAM
─────────────────────────────────────────────────────────────────
  extension/src/
  ├── router.rs
  │   ├── CommandRouter        ← maps commands → workflow IDs
  │   ├── ValidationLayer      ← schema check on args
  │   └── ErrorHandler         ← surfaces errors back to panel
  │
  └── context_builder.rs
      ├── FileContextCollector ← open buffers, file tree
      ├── GitDiffCollector     ← current uncommitted diff
      ├── TerminalLogCollector ← last N lines of terminal
      ├── MetadataCollector    ← project name, lang, framework
      └── ContextEnvelope      ← combined payload struct
─────────────────────────────────────────────────────────────────
```

**Features & Functions:**

| # | Function | Description |
|---|----------|-------------|
| 2.1 | `CommandRouter::register(cmd, workflow_id)` | Map each slash command to a workflow identifier |
| 2.2 | `CommandRouter::dispatch(cmd, args)` | Validate, look up, forward to control plane |
| 2.3 | `ValidationLayer::validate(cmd, args)` | Schema check: required fields, type safety |
| 2.4 | `FileContextCollector::collect()` | Gather open files + project tree (truncated safely) |
| 2.5 | `GitDiffCollector::collect()` | Run `git diff HEAD` via libgit2; include in context |
| 2.6 | `TerminalLogCollector::collect(n_lines)` | Capture last N terminal lines; run through redactor |
| 2.7 | `MetadataCollector::collect()` | Project lang/framework/version from config files |
| 2.8 | `ContextEnvelope::build()` | Assemble all collectors into signed payload struct |
| 2.9 | `ErrorHandler::surface(err)` | Show structured error in panel; log to .ai_memory/ |

**Recovery & Rollback:**
```
AUTO-RECOVERY — STEP 2
────────────────────────────────────────────────────────
BEFORE STEP 2: git commit "pre: step-2 router+context"
                snapshot → snapshots/002-pre-step2/

AFTER 2.3: unit test ValidationLayer with 10 valid + 10 invalid inputs
  → FAIL: revert validation.rs, do not proceed

AFTER 2.6: TerminalLogCollector must NEVER include tokens resembling
           API keys (regex: sk-*, Bearer *, AKIA*)
  → FAIL: fix redactor, re-test, do not proceed to 2.7

AFTER 2.8: ContextEnvelope serialises to JSON without panic
  → FAIL: git reset --hard to pre-step-2 sha

ROLLBACK COMMAND:
  git reset --hard $(cat .ai_memory/snapshots/002-pre-step2/commit.sha)
  cargo test -p extension -- router context
────────────────────────────────────────────────────────
```

---

### STEP 3 — LOCAL PERSISTENT MEMORY SYSTEM

**Goal:** Every AI interaction is stored, indexed, and retrievable. This is the foundation for replay and restore.

```
STEP 3 COMPONENT DIAGRAM
─────────────────────────────────────────────────────────────────
  .ai_memory/
  ├── metadata.sqlite          ← WAL mode, AES-256 at rest
  │   Tables:
  │   ├── sessions             (id, timestamp, project, agent_role)
  │   ├── interactions         (session_id, role, content_hash, vector_ref)
  │   ├── snapshots            (session_id, git_sha, flow_lock_hash)
  │   └── redaction_log        (timestamp, pattern_matched, action)
  │
  ├── embedding_cache.db       ← sqlite_vss vector index
  ├── flow.lock                ← toolchain versions (updated each session)
  └── snapshots/               ← full directory snapshots for rollback

  extension/src/
  └── memory/
      ├── store.rs             ← MemoryStore: CRUD on metadata.sqlite
      ├── embedder.rs          ← embed text → vector; cache locally
      ├── retriever.rs         ← semantic search over prior interactions
      ├── redactor.rs          ← strip secrets before indexing
      ├── snapshot.rs          ← create/restore directory snapshots
      └── flow_lock.rs         ← read/write/compare flow.lock
─────────────────────────────────────────────────────────────────
```

**Features & Functions:**

| # | Function | Description |
|---|----------|-------------|
| 3.1 | `MemoryStore::init(path)` | Create/open SQLite with WAL + encryption; run migrations |
| 3.2 | `MemoryStore::save_interaction(session, role, content)` | Persist each AI turn atomically |
| 3.3 | `MemoryStore::load_session_context(session_id)` | Retrieve full prior context for a session |
| 3.4 | `Redactor::scan_and_strip(text)` | Remove API keys, tokens, PII before storage |
| 3.5 | `Embedder::embed(text) -> vector` | Generate local embedding (e.g. via Ollama) |
| 3.6 | `Embedder::cache(hash, vector)` | Store vector in sqlite_vss; skip if hash exists |
| 3.7 | `Retriever::semantic_search(query, top_k)` | Return top-k prior interactions by relevance |
| 3.8 | `Snapshot::create(tag)` | Copy .ai_memory/ to snapshots/<timestamp>-<tag>/ |
| 3.9 | `Snapshot::restore(tag)` | Atomically replace .ai_memory/ from snapshot |
| 3.10 | `FlowLock::capture()` | Record rustc, zed API, python, node versions |
| 3.11 | `FlowLock::diff(prev, current)` | Alert on unexpected toolchain drift |

**Recovery & Rollback:**
```
AUTO-RECOVERY — STEP 3
────────────────────────────────────────────────────────
BEFORE STEP 3: git commit "pre: step-3 memory system"
                snapshot → snapshots/003-pre-step3/

AFTER 3.1: open SQLite, confirm WAL mode ON, tables exist
  → FAIL: delete .ai_memory/, re-init

AFTER 3.4: Redactor test suite — 50 inputs containing fake API keys
           ALL must be stripped, none must pass through
  → FAIL: do NOT proceed; memory store cannot be trusted

AFTER 3.7: SemanticSearch returns stable results on known test vectors
  → FAIL: revert embedder.rs, re-run from 3.5

AFTER 3.9: Restore snapshot then verify all tables intact
  → FAIL: restore from snapshots/003-pre-step3/ and alert

ROLLBACK COMMAND:
  git reset --hard $(cat .ai_memory/snapshots/003-pre-step3/commit.sha)
  rm -rf .ai_memory/  &&  mkdir .ai_memory/
  cp -r .ai_memory/snapshots/003-pre-step3/* .ai_memory/
────────────────────────────────────────────────────────
```

---

### STEP 4 — PROVIDER ABSTRACTION LAYER

**Goal:** A unified, testable interface to all LLM providers with cost tracking, rate limiting, and automatic fallback.

```
STEP 4 COMPONENT DIAGRAM
─────────────────────────────────────────────────────────────────
  control-plane/src/providers/
  ├── mod.rs                   ← ProviderAdapter trait definition
  ├── anthropic.rs             ← Claude (Sonnet/Haiku/Opus)
  ├── openai.rs                ← GPT-4o / GPT-4 Turbo
  ├── groq.rs                  ← Groq (Llama 3 etc)
  ├── deepseek.rs              ← DeepSeek V2/V3
  ├── ollama.rs                ← Local Ollama (Llama, Mistral)
  ├── mock.rs                  ← Deterministic mock for testing
  ├── cost_tracker.rs          ← Track tokens in/out per provider
  ├── rate_limiter.rs          ← Token bucket per provider
  ├── fallback_router.rs       ← Auto-switch on failure/budget
  └── safety_filter.rs         ← Block harmful outputs pre-response

  AgentResponse envelope:
  {
    request_id: uuid,
    provider: string,
    role: AgentRole,
    content: string,
    tokens_used: { input: u32, output: u32 },
    cost_usd: f64,
    latency_ms: u32,
    safety_passed: bool
  }
─────────────────────────────────────────────────────────────────
```

**Features & Functions:**

| # | Function | Description |
|---|----------|-------------|
| 4.1 | `ProviderAdapter::complete(prompt, config) -> AgentResponse` | Trait: all providers implement this interface |
| 4.2 | `AnthropicAdapter::complete()` | Claude API call with retry logic |
| 4.3 | `OpenAIAdapter::complete()` | OpenAI API call with retry logic |
| 4.4 | `GroqAdapter::complete()` | Groq inference API call |
| 4.5 | `OllamaAdapter::complete()` | Local Ollama HTTP call; no cost tracking needed |
| 4.6 | `MockAdapter::complete()` | Returns deterministic fixture; used in all tests |
| 4.7 | `CostTracker::record(provider, tokens_in, tokens_out)` | Accumulate cost; write to SQLite |
| 4.8 | `CostTracker::check_budget(provider) -> BudgetStatus` | Block calls if daily budget exceeded |
| 4.9 | `RateLimiter::acquire(provider) -> Result` | Token bucket: block or queue if limit hit |
| 4.10 | `FallbackRouter::route(request) -> Provider` | Try primary; fall back to secondary list on failure |
| 4.11 | `SafetyFilter::screen(response) -> Result` | Block responses containing harmful content patterns |

**Recovery & Rollback:**
```
AUTO-RECOVERY — STEP 4
────────────────────────────────────────────────────────
BEFORE STEP 4: git commit "pre: step-4 provider layer"
                snapshot → snapshots/004-pre-step4/

AFTER 4.6: ALL subsequent tests MUST use MockAdapter by default
  → Enforce: integration tests must not hit real endpoints without flag

AFTER 4.8: budget check returns BLOCKED when $0.01 test limit exceeded
  → FAIL: do not add real provider adapters yet

AFTER 4.10: FallbackRouter correctly routes through all 3 providers in sequence
  → FAIL: revert fallback_router.rs; re-test

AFTER 4.11: SafetyFilter blocks 100% of test harmful-pattern inputs
  → FAIL: do not proceed; safety is non-negotiable

RUNTIME AUTO-RECOVERY:
  On provider failure (HTTP 5xx or timeout) → FallbackRouter switches provider
  On budget exceeded → pause all real calls, switch to MockAdapter + alert
  On 3 consecutive fallback failures → stop workflow, save state, alert user

ROLLBACK COMMAND:
  git reset --hard $(cat .ai_memory/snapshots/004-pre-step4/commit.sha)
────────────────────────────────────────────────────────
```

---

### STEP 5 — AGENT ORCHESTRATION ENGINE

**Goal:** Multi-agent pipelines that are deterministic, retryable, and replay-safe.

```
STEP 5 COMPONENT DIAGRAM
─────────────────────────────────────────────────────────────────
  control-plane/src/orchestration/
  ├── planner.rs               ← LangGraph-style DAG planner
  ├── agent_roles.rs           ← Role definitions (Architect/Coder/Auditor/Tester)
  ├── session_state.rs         ← State machine per workflow run
  ├── workflow_engine.rs       ← Temporal/Prefect adapter for durable runs
  ├── handoff.rs               ← Agent-to-agent message passing
  ├── approval_gate.rs         ← Pause workflow; require user approval
  └── replay.rs                ← Re-run workflow from any saved checkpoint

  Agent Role State Machine:
  ┌──────────────┐     ┌──────────────┐     ┌──────────────┐
  │  IDLE        │────>│  PLANNING    │────>│  EXECUTING   │
  └──────────────┘     └──────────────┘     └──────┬───────┘
                                                   │
                             ┌─────────────────────┼─────────────────────┐
                             ▼                     ▼                     ▼
                       ┌──────────┐         ┌──────────┐         ┌──────────┐
                       │ APPROVED │         │ REJECTED │         │  FAILED  │
                       └────┬─────┘         └────┬─────┘         └────┬─────┘
                            ▼                    ▼                    ▼
                       ┌──────────┐         ┌──────────┐         ┌──────────┐
                       │ COMMITTED│         │ ROLLEDBACK│        │ RECOVERED│
                       └──────────┘         └──────────┘         └──────────┘
─────────────────────────────────────────────────────────────────
```

**Features & Functions:**

| # | Function | Description |
|---|----------|-------------|
| 5.1 | `Planner::build_dag(workflow_def) -> DAG` | Parse agents.yaml into a directed acyclic execution graph |
| 5.2 | `AgentRole::Architect::run(context) -> Plan` | Produce high-level design plan for the requested change |
| 5.3 | `AgentRole::Coder::run(plan, context) -> Patch` | Generate code patch implementing the plan |
| 5.4 | `AgentRole::Auditor::run(patch) -> AuditReport` | Review patch for correctness, style, security |
| 5.5 | `AgentRole::Tester::run(patch) -> TestResult` | Generate and run tests for the patch |
| 5.6 | `SessionState::save(checkpoint)` | Persist current workflow state to .ai_memory/ |
| 5.7 | `SessionState::load(session_id)` | Resume from any saved checkpoint |
| 5.8 | `Handoff::pass(from_role, to_role, payload)` | Typed message between agents with audit trail |
| 5.9 | `ApprovalGate::pause(workflow_id, diff)` | Halt workflow; send diff to extension for user review |
| 5.10 | `ApprovalGate::resume(workflow_id, decision)` | Continue or rollback based on user decision |
| 5.11 | `Replay::run(session_id, from_step)` | Re-execute workflow from given checkpoint |
| 5.12 | `WorkflowEngine::submit(dag) -> run_id` | Submit DAG to Temporal/Prefect for durable execution |

**Recovery & Rollback:**
```
AUTO-RECOVERY — STEP 5
────────────────────────────────────────────────────────
BEFORE STEP 5: git commit "pre: step-5 orchestration"
                snapshot → snapshots/005-pre-step5/

AFTER 5.1: DAG planner must detect and REJECT cyclic graphs
  → FAIL: block; cyclic agents would loop forever

AFTER 5.5: Tester agent must run in sandbox ONLY (no host filesystem writes)
  → FAIL: do not proceed; agent execution safety is non-negotiable

AFTER 5.9: ApprovalGate must block forward execution; verify via integration test
  → FAIL: revert approval_gate.rs

AFTER 5.11: Replay of a known session must produce identical patch
  → FAIL: non-determinism detected; investigate Coder agent prompt; re-test

RUNTIME AUTO-RECOVERY:
  Agent step failure → Temporal retries up to 3x with backoff
  3x retry exhausted → save SessionState checkpoint → alert user
  User rejects diff → ApprovalGate triggers Rollback → git reset to pre-patch sha

ROLLBACK COMMAND:
  git reset --hard $(cat .ai_memory/snapshots/005-pre-step5/commit.sha)
  cp -r .ai_memory/snapshots/005-pre-step5/* .ai_memory/
────────────────────────────────────────────────────────
```

---

### STEP 6 — GIT SYNC & SNAPSHOT MANAGER

**Goal:** Every approved AI workflow produces a reproducible, auditable Git commit with zero secret leakage.

```
STEP 6 COMPONENT DIAGRAM
─────────────────────────────────────────────────────────────────
  control-plane/src/git/
  ├── snapshot_manager.rs      ← create reproducible commits
  ├── branch_strategy.rs       ← ai/<session-id> branch per workflow
  ├── secret_scanner.rs        ← gitleaks integration pre-commit
  ├── flow_lock_writer.rs      ← append toolchain state to commit
  └── restore.rs               ← restore workspace from git sha + memory snapshot

  Commit message format:
  ─────────────────────
  ai(refactor): <summary> [session:<uuid>]

  Agent: Coder / Auditor / Tester
  Provider: anthropic/claude-sonnet-4-6
  Memory-ref: .ai_memory/snapshots/<timestamp>/
  Flow-lock: <hash>
  Cost: $0.0023
  Tokens: 1847 in / 512 out
─────────────────────────────────────────────────────────────────
```

**Features & Functions:**

| # | Function | Description |
|---|----------|-------------|
| 6.1 | `BranchStrategy::create(session_id)` | Create `ai/<session-id>` branch before applying patch |
| 6.2 | `SnapshotManager::stage(patch)` | Stage AI-generated changes; run secret scanner before commit |
| 6.3 | `SecretScanner::scan(staged_files) -> ScanResult` | Run gitleaks; BLOCK commit if secrets found |
| 6.4 | `FlowLockWriter::append(commit_meta)` | Record toolchain + agent + cost metadata in commit |
| 6.5 | `SnapshotManager::commit(message)` | Create structured commit with full audit trail |
| 6.6 | `SnapshotManager::tag(session_id)` | Tag commit as `ai-snapshot/<session-id>` for easy lookup |
| 6.7 | `Restore::from_sha(sha)` | Reset workspace to given commit + restore matching memory snapshot |
| 6.8 | `Restore::list_snapshots()` | List all ai-snapshot tags with session metadata |

**Recovery & Rollback:**
```
AUTO-RECOVERY — STEP 6
────────────────────────────────────────────────────────
BEFORE STEP 6: git commit "pre: step-6 git sync"
                snapshot → snapshots/006-pre-step6/

AFTER 6.3: SecretScanner MUST block 100% of test commits containing fake secrets
  → FAIL: do not proceed under any circumstance

AFTER 6.5: verify commit message contains all required fields (session, cost, flow-lock)
  → FAIL: revert SnapshotManager::commit, fix format

AFTER 6.7: Restore::from_sha must return workspace to byte-identical state
  → FAIL: restore from snapshots/006-pre-step6/ and re-investigate

RUNTIME AUTO-RECOVERY:
  Secret detected mid-commit → ABORT commit → git restore --staged .
                              → alert user with file+line detail
                              → save incident to .ai_memory/redaction_log.txt
  Restore fails (corrupt snapshot) → fall back to previous snapshot tag

ROLLBACK COMMAND:
  git reset --hard $(cat .ai_memory/snapshots/006-pre-step6/commit.sha)
────────────────────────────────────────────────────────
```

---

### STEP 7 — SANDBOX RUNNER & SAFETY LAYER

**Goal:** Agent-suggested code is NEVER executed on the host without user approval and sandboxing.

```
STEP 7 COMPONENT DIAGRAM
─────────────────────────────────────────────────────────────────
  control-plane/src/sandbox/
  ├── runner.rs                ← container / firejail executor
  ├── approval_ui.rs           ← present proposed execution to user
  ├── output_capture.rs        ← capture stdout/stderr; redact before storage
  ├── resource_limits.rs       ← CPU/mem/network caps per sandbox run
  └── audit_log.rs             ← every execution recorded immutably

  Execution flow:
  Agent suggests code to run
         │
         ▼
  ApprovalGate (user must explicitly approve)
         │
    ┌────┴────┐
   APPROVE   REJECT
    │             │
    ▼             ▼
  Sandbox       Skip +
  Runner        log reason
    │
    ▼
  Capture output → redact → store
    │
    ▼
  Return result to agent
─────────────────────────────────────────────────────────────────
```

**Features & Functions:**

| # | Function | Description |
|---|----------|-------------|
| 7.1 | `SandboxRunner::spawn(code, lang, limits)` | Launch container/firejail with resource caps |
| 7.2 | `ResourceLimits::apply(cpu_pct, mem_mb, net_disabled)` | Enforce strict OS-level limits |
| 7.3 | `ApprovalUI::present(code_to_run)` | Show code + risk rating to user; await decision |
| 7.4 | `OutputCapture::capture() -> Output` | Record stdout/stderr with timeout |
| 7.5 | `OutputCapture::redact(output)` | Strip secrets/tokens from captured output |
| 7.6 | `AuditLog::record(session, code_hash, decision, output_hash)` | Immutable execution record |
| 7.7 | `SandboxRunner::cleanup()` | Destroy container; verify no host filesystem mutations |

**Recovery & Rollback:**
```
AUTO-RECOVERY — STEP 7
────────────────────────────────────────────────────────
BEFORE STEP 7: git commit "pre: step-7 sandbox"
                snapshot → snapshots/007-pre-step7/

AFTER 7.1: sandbox must NOT be able to write to host filesystem
  → Test: attempt write to /tmp from inside sandbox → must FAIL
  → If sandbox write succeeds: STOP, fix resource limits, re-test

AFTER 7.3: ApprovalUI must BLOCK all runs if user does not explicitly APPROVE
  → Default must be REJECT, not approve
  → FAIL: critical safety violation; revert, re-test

AFTER 7.7: verify zero processes remain after cleanup
  → FAIL: sandbox leak; do not proceed

ROLLBACK COMMAND:
  git reset --hard $(cat .ai_memory/snapshots/007-pre-step7/commit.sha)
────────────────────────────────────────────────────────
```

---

### STEP 8 — COST TRACKING & OBSERVABILITY

**Goal:** Full operational visibility — cost, latency, errors, and agent performance in real time.

```
STEP 8 COMPONENT DIAGRAM
─────────────────────────────────────────────────────────────────
  control-plane/src/observability/
  ├── cost_dashboard.rs        ← real-time cost per provider/session
  ├── otel_exporter.rs         ← OpenTelemetry traces + metrics
  ├── alert_manager.rs         ← budget breach, error rate, latency alerts
  └── session_report.rs        ← per-session summary (cost, agents, duration)

  Metrics tracked:
  ├── provider.tokens.in       (counter, per provider)
  ├── provider.tokens.out      (counter, per provider)
  ├── provider.cost.usd        (gauge, per provider per day)
  ├── agent.latency.ms         (histogram, per role)
  ├── workflow.success.rate    (gauge, per workflow type)
  ├── memory.retrieval.ms      (histogram)
  └── sandbox.execution.ms     (histogram)
─────────────────────────────────────────────────────────────────
```

**Features & Functions:**

| # | Function | Description |
|---|----------|-------------|
| 8.1 | `CostDashboard::record(event)` | Write cost event to SQLite + emit OTel metric |
| 8.2 | `CostDashboard::daily_summary()` | Return total cost per provider for today |
| 8.3 | `OTelExporter::init(endpoint)` | Configure OpenTelemetry exporter (Jaeger/Grafana) |
| 8.4 | `OTelExporter::trace_workflow(workflow_id)` | Emit spans for each agent step |
| 8.5 | `AlertManager::set_budget(provider, daily_usd)` | Configure per-provider daily spend limit |
| 8.6 | `AlertManager::check_and_alert()` | Fire alert + pause if budget exceeded |
| 8.7 | `SessionReport::generate(session_id)` | Produce human-readable session summary |

**Recovery & Rollback:**
```
AUTO-RECOVERY — STEP 8
────────────────────────────────────────────────────────
BEFORE STEP 8: git commit "pre: step-8 observability"
                snapshot → snapshots/008-pre-step8/

AFTER 8.5: confirm alert fires when synthetic cost exceeds $0.01 test budget
  → FAIL: do not connect real providers until alerts work

AFTER 8.6: confirm real provider calls are paused after budget alert
  → FAIL: revert alert_manager.rs; cost blowup risk

ROLLBACK COMMAND:
  git reset --hard $(cat .ai_memory/snapshots/008-pre-step8/commit.sha)
────────────────────────────────────────────────────────
```

---

### STEP 9 — END-TO-END INTEGRATION & REPRODUCIBILITY TEST

**Goal:** Prove the full loop works: slash command → agents → patch → approval → commit → restore.

```
STEP 9 — INTEGRATION TEST MATRIX
─────────────────────────────────────────────────────────────────
  Test 1: Single-provider round trip
  /refactor dummy_module
  → MockAdapter responds
  → Patch applied to test file
  → SecretScanner passes
  → Commit created with audit trail
  → PASS

  Test 2: Provider fallback
  Primary provider returns 503
  → FallbackRouter switches to secondary
  → Workflow completes successfully
  → PASS

  Test 3: Budget enforcement
  Daily budget set to $0.001
  → First call allowed
  → Second call BLOCKED
  → MockAdapter substituted
  → PASS

  Test 4: Restore from snapshot
  Session S1 committed, memory saved
  Simulate filesystem change
  Restore::from_sha(S1.sha)
  → Workspace identical to S1
  → Memory identical to S1
  → PASS

  Test 5: Secret blocked
  Inject fake API key in context
  → Redactor strips it
  → SecretScanner blocks any that slip through
  → PASS (zero secrets in commit)

  Test 6: Sandbox isolation
  Agent requests dangerous shell command
  → ApprovalUI shown to user
  User rejects
  → Command not executed
  → AuditLog records rejection
  → PASS
─────────────────────────────────────────────────────────────────
```

**Recovery & Rollback:**
```
AUTO-RECOVERY — STEP 9
────────────────────────────────────────────────────────
ALL 6 tests must PASS before ANY real provider credentials
are added to the system.

If any test fails:
  → identify failing component
  → git reset --hard to that component's pre-step snapshot
  → fix, re-run unit tests for that component
  → re-run full integration matrix from Test 1

NOTHING ships to production until Test Matrix 100% green.
────────────────────────────────────────────────────────
```

---

### STEP 10 — PRODUCTION HARDENING & LICENSE COMPLIANCE

**Goal:** Legal sign-off, security audit, and production-ready deployment.

**Features & Functions:**

| # | Function | Description |
|---|----------|-------------|
| 10.1 | `license_audit()` | Map every Zed crate to AGPL/Apache/GPL; get legal sign-off |
| 10.2 | `security_audit()` | Third-party penetration test of control-plane API |
| 10.3 | `gdpr_policy()` | Document data retention, deletion, and PII handling |
| 10.4 | `ci_pipeline()` | GitHub Actions: test + secret scan + build + deploy gates |
| 10.5 | `extension_publish()` | Submit to Zed extension marketplace (only after legal clearance) |
| 10.6 | `runbook_write()` | Ops runbook: incident response, rollback procedure, budget resets |

**Recovery & Rollback:**
```
AUTO-RECOVERY — STEP 10
────────────────────────────────────────────────────────
Security audit findings must be resolved before publishing.
License issues blocking AGPL → do NOT publish until resolved.
CI pipeline must enforce: no merge without green tests + secret scan.
────────────────────────────────────────────────────────
```

---

## SUMMARY ROLLBACK REFERENCE TABLE

```
╔═══════════╦══════════════════════════════════════╦══════════════════════════════════════════╗
║  STEP     ║  PRE-SNAPSHOT TAG                    ║  ROLLBACK COMMAND                        ║
╠═══════════╬══════════════════════════════════════╬══════════════════════════════════════════╣
║  0        ║  000-baseline                        ║  rm -rf repo && re-scaffold              ║
║  1        ║  001-pre-step1                       ║  git reset --hard + restore snapshot     ║
║  2        ║  002-pre-step2                       ║  git reset --hard + restore snapshot     ║
║  3        ║  003-pre-step3                       ║  git reset --hard + rm -rf .ai_memory    ║
║  4        ║  004-pre-step4                       ║  git reset --hard + restore snapshot     ║
║  5        ║  005-pre-step5                       ║  git reset --hard + restore snapshot     ║
║  6        ║  006-pre-step6                       ║  git reset --hard + git restore staged   ║
║  7        ║  007-pre-step7                       ║  git reset --hard + kill sandbox procs   ║
║  8        ║  008-pre-step8                       ║  git reset --hard + restore snapshot     ║
║  9        ║  N/A (integration gate)              ║  revert to failing component's snapshot  ║
║  10       ║  N/A (hardening gate)                ║  do not publish until 100% resolved      ║
╚═══════════╩══════════════════════════════════════╩══════════════════════════════════════════╝

CANONICAL ROLLBACK TEMPLATE (copy-paste ready):
────────────────────────────────────────────────────────────────────────────────
STEP=<N>
SHA=$(cat .ai_memory/snapshots/00${STEP}-pre-step${STEP}/commit.sha)
git reset --hard $SHA
rm -rf .ai_memory/
cp -r .ai_memory/snapshots/00${STEP}-pre-step${STEP}/ .ai_memory/
cargo test --all
────────────────────────────────────────────────────────────────────────────────
```

---

## TECHNOLOGY STACK REFERENCE

```
┌─────────────────────────────┬──────────────────────────────────────────────┐
│  Component                  │  Technology Choice                            │
├─────────────────────────────┼──────────────────────────────────────────────┤
│  IDE Host                   │  Zed (Rust, open source, extension API)       │
│  Extension Language         │  Rust + WASM (zed_extension_api crate)        │
│  Agent Orchestration        │  LangGraph (deterministic) + Temporal (durable│
│  Agent Swarms               │  OpenClaw / NanoClaw (parallel tasks)         │
│  Workflow Engine            │  Temporal.io (retry, checkpoint, durable WF)  │
│  Local Memory DB            │  SQLite (WAL + AES-256) + sqlite_vss          │
│  Remote Vector DB           │  Qdrant (primary) / Weaviate (alternative)    │
│  Code Parsing               │  tree-sitter + ast-grep                       │
│  Local Inference            │  Ollama (Llama 3 / Mistral)                   │
│  Cloud Providers            │  Anthropic · OpenAI · Groq · DeepSeek         │
│  Git Automation             │  libgit2 / git CLI                            │
│  Secret Scanning            │  gitleaks + trufflehog (pre-commit + CI)      │
│  Sandbox Execution          │  Docker / firejail (resource-capped)          │
│  Dev Environment            │  Devcontainers + Nix (reproducible)           │
│  Observability              │  OpenTelemetry → Grafana / Jaeger             │
│  CI/CD                      │  GitHub Actions (test + scan + build gates)   │
└─────────────────────────────┴──────────────────────────────────────────────┘
```
