# Merix-V2 Full Development Plan

*(Pure-Rust 2024 + Tauri v2 + SurrealDB vectors + Dashmap + oxillama/oxiwhisper + Rig agent)*

This checklist takes us from zero to a fully working, installable desktop application that combines the functionality of **llama.cpp**, **whisper.cpp**, **llama.vscode**, **ollama**, **hermes-agent**, and **hermes-workspace** into one native Rust app.

---

## Phase 1: Project Setup & Scaffolding

- [x] Create a new empty folder named `Merix-V2` (or use the PowerShell script below)
- [x] Save and run the latest `Merix-V2-scaffold.ps1` from our previous message (it creates the entire modular crate structure under `crates/`)
- [x] Replace the root `Cargo.toml` with the exact workspace version shown at the end of the scaffold output (the one that declares all 7 members and re-exports workspace dependencies)
- [x] Run `cargo check` in the root to verify the workspace builds cleanly
- [x] Run `cargo tauri dev` once (it will show the placeholder window and greet command)

## Phase 2: Core Shared Crate (`merix-core`)

- [x] Complete `crates/core/src/types.rs` (add `Config`, `ModelInfo`, `AgentState`, `Skill`, `MemoryEntry`, `ToolCall`, etc.)
- [x] Expand `crates/core/src/error.rs` with proper error variants for every module
- [x] Add `crates/core/src/config.rs` (TOML-based config with model paths, SurrealDB options, default agent settings)
- [x] Re-export everything cleanly in `lib.rs`
- [x] Add unit tests for serialization

## Phase 3: Database Layer (`merix-db`) - this is a db abstration layer for surreal db

- [x] Implement `crates/db/src/connection.rs` — embedded SurrealDB (RocksDB + in-memory fallback) with proper startup/shutdown
- [x] Create `crates/db/src/vector.rs` — define tables (`memory`, `skills`, `sessions`, `trajectories`, `user_profile`) and create HNSW/MTree vector indexes on embeddings
- [x] Add helper methods: `store_memory()`, `vector_search()`, `hybrid_search()`, `store_skill()`, `get_user_profile()`
- [x] Write integration tests (use `#[tokio::test]` with temporary RocksDB)
- [x] Expose a clean `Db` struct that other crates can depend on

## Phase 4: Vector Embedding (`merix-embed`) - this is how we embed vectors for semantic search

- [ ] Implement `crates/inference/src/embed.rs` - Candle-based embedding for vector search

## Phase 5: Cache Layer (`merix-cache`) - this is a RAM abstraction layer for dashmap

- [x] Finish `crates/cache/src/lib.rs` with Dashmap-based cache
- [x] Add TTL/eviction helpers and atomic operations (e.g., `upsert_context`)
- [x] Add `global_caches()` singleton pattern for easy access from Tauri commands

## Phase 6: Storage Layer (`merix-storage`) - this is a unified API for the db and cache layers

The storage layer is for handling both persistent and ephemeral storage

- [ ] Finish `crates/storage/src/lib.rs`

## Phase 7: Memory Layer (`merix-memory`) - LLM memory

The memory layer is where the actual "LLM Memory" lives, it is a policy + retrieval + transformation layer and decides:

- what is important
- what gets stored
- what gets retrieved
- what gets embedded
- what gets summarized

## Phase 8: Inference Engine (`merix-inference`)

- [ ] Implement `crates/inference/src/llm.rs` — load GGUF models via oxillama, streaming generation, FIM completions (for code)
- [ ] Implement `crates/inference/src/stt.rs` — real-time Whisper streaming + VAD via oxiwhisper
- [ ] Implement `crates/merix-inference/src/server.rs` — embedded OpenAI-compatible server (localhost:11434) so existing tools still work
- [ ] Add Tauri commands: `chat_stream`, `transcribe_mic`, `load_model`, `list_models`
- [ ] Add model auto-download logic (similar to Ollama) into `models/` folder

## Phase 9: Tools Layer (`merix-tools`)

- [ ] Implement `crates/tools/src/file.rs` (read/write, search, tree view)
- [ ] Implement `crates/tools/src/terminal.rs` (PTY via `tauri-plugin-shell` or `portable-pty`)
- [ ] Implement `crates/tools/src/code_exec.rs` (safe sandboxed Rust/Python/JS execution) -> Rename to `code_sandbox.rs`
- [ ] Add more MCP-style tools (browser, git, search, etc.)
- [ ] Expose a unified `ToolRegistry` that Rig can call

## Phase 10: Agent Runtime (`merix-agent`)

- [ ] Implement `crates/agent/src/runtime.rs` — full HermesAgent using Rig + memory from SurrealDB + tools
- [ ] Implement `crates/agent/src/skills.rs` — skill creation, refinement, vector retrieval, self-improvement loop
- [ ] Implement `crates/agent/src/swarm.rs` — multi-agent orchestration (like hermes-workspace swarm view)
- [ ] Add long-term memory nudges, trajectory saving, and cron-style background tasks
- [ ] Connect everything to Dashmap for real-time state

## Phase 11: Tauri Backend Integration (`tauri`)

- [ ] Add all Tauri commands in `tauri/src/main.rs` (or better: create a `commands/` module)
- [ ] Initialize DB + caches + inference + agent in the `setup` hook
- [ ] Add state management (`tauri::State`) for shared services
- [ ] Enable devtools and logging
- [ ] Add window events (resize, close → graceful shutdown)

## Phase 12: Frontend (Workspace UI)

- [ ] Set up a modern frontend in `src/` (Vite + React + TypeScript or Svelte — whichever you prefer)
- [ ] Install and configure Monaco Editor, XTerm.js, shadcn/ui or Tailwind
- [ ] Port the key screens from the workspace:
  - Chat interface (streaming + tool calls)
  - Terminal panel
  - Memory / Skills browser (with vector search)
  - File explorer + inline editor
  - Dashboard + Swarm Kanban
- [ ] Call Tauri commands via `@tauri-apps/api`
- [ ] Add voice input button (STT) and voice output (TTS placeholder)

## Phase 13: Testing & Polish

- [ ] Write end-to-end tests for chat → agent → tool → memory flow
- [ ] Add error boundaries and graceful fallbacks
- [ ] Implement model switching UI
- [ ] Add settings window (config, model management)
- [ ] Theme support (dark/light) + splash screen
- [ ] Performance tuning (token streaming, vector search latency)

## Phase 14: Packaging & Distribution

- [ ] Run `cargo tauri build` for Windows, macOS, Linux
- [ ] Create platform-specific installers (`.msi`, `.dmg`, `.deb`)
- [ ] Add auto-updater (Tauri plugin)
- [ ] Bundle default small models or create a first-run downloader
- [ ] Write final README with screenshots and usage guide
- [ ] (Optional) Publish to GitHub Releases
