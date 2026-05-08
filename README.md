# Merix

*The self-extending Agentic OS*

This project aims to merge the following into a single, comprehensive agentic operating system that is capable of self-extension:

- llama.cpp
- whisper.cpp
- llama.vscode
- ollama
- hermes-agent
- hermes-workspace

We will eventually want to extend Merix to have the following capabilities:

```text
Merix/
├── adaptation/             # Real-time adjustments for dynamic strategy switching, fallback behaviors and performance tuning mid-task for agents.
├── agents/                 # Provides identity, permission scopes, capability ownership and audit trails
├── alignment/              # Ensures agents act within user goals, constraints, and system objectives.
├── app_connect/            # Connects cloud apps for AI use (Cloud AI, Google, APIs, etc...)
├── attention/              # Provides context prioritization, tool selection filtering, dynamic context compression
├── capabilities/           # Allows for skill generation via LLM codegen, validation loop and versioning
├── cli/                    # Minimal Control Interface
├── communication/          # A2A Protocol Layer - handles structured agent messaging, negotiation protocols, shared context exchange
├── compatibility/          # Handles model differences, tool schema evolution and backwards compatibility.
├── context/                # Handles context assembly, prioritization and truncation strategies. See note.
├── coordination/           # Similar to orchestration, but handles conflict resolution, resource arbitration, multi-agent synchronization (see note)
├── core/                   # Task execution + LLM runtime (llama.cpp)
├── deployment/             # Allows the packaging of agents, distributing skills, updating components and version rollbacks
├── economics/              # Handles model selection, tool cost budgeting, execution optimization
├── environment/            # Abstracts OS interaction, browser automation and file system control.
├── executor/               # Defines execution graphs (DAGs), step scheduling, retry/failure handling, determinism boundaries. See note.
├── experience/             # Stores trajectories (task -> actions -> outcomes) and enables reuse of successful patterns and powers learning systems.
├── evaluation/             # Scoring outputs, benchmarking tasks and regression detection for when agents modify their own behavior and skills evolve.
├── events/                 # Provides a pub/sub event bus for system-wide signals for observability, decoupling, real-time adaptation. See note for examples.
├── governance/             # Policy enforcement, risk thresholds, approval systems constraint validation
├── intent/                 # Parses user goals into constraints, success criteria, sub-goals to prevent vague execution and enables verification
├── interop/                # Normalizes external interfaces and prevents pollution of core logic when using Apps
├── interface/              # Allows for voice, real-time streams, multimodal input interactions with the agent(s)
├── knowledge/              # Structured knowledge reasoning using knowledge graphs, entity linking and semantic relationships.
├── llama/                  # Provides the API, management and optimizations for the llama/oxillama backend.
├── memory/                 # Persistent (SurrealDB) + Ethereal (Dashmap) — (separated into 2 implementations)
├── observability/          # Provides traces, spans, decision logs, replay systems
├── orchestrator/           # Handles agent-to-agent protocols (A2A), role assignment, concensus / arbitration (see note)
├── persona/                # Handles "what the agent becomes" - personality/behavior profiles, long-term preferences, communication style, role specialization.
├── personalization/        # Allows LLM personalization based on user profile (auto + manual).
├── planner/                # Prompts LLM for planning, validates plan and provides cost/complexity estimation. See note.
├── rag/                    # File embedding provider (choose "data location" and it will create embeddings & add them to the DB) for ingestion.
├── recovery/               # Structured error handling, retry strategies, fallback planning to prevent hallucinations, tool errors, timeouts.
├── reflection/             # History analysis, what worked, what failed, performance per skill/tool. See note.
├── registry/               # Provides a registry for tools, skills, agents, and anything else added with versioning, capability metadata and search
├── relationships/          # Tracks agent-to-agent relationships, trust scores / reliability, delegation history.
├── resources/              # Provides CPU/GPU allocation, memory pressure and concurrency limits.
├── sandbox/                # Security + Execution Isolation. Isolates execution, resource limits and failure containment.
├── schemas/                # Database & In-Memory data structures (Session, Task, Checkpoint, Skill, etc.)
├── security/               # System-wide protection and handles secrets, auth, encryption, attack prevention for both incoming and outgoing
├── server/                 # Provides VPN-like and E2EE capabilities for various communication apps and direct communications to a `Merix-Server`.
├── skills/                 # Skills registry & loading
├── simulation/             # Allows agents to simulate outcomes, test plans, estimate risk within a sandbox for planning validation and "what-if" reasoning
├── state/                  # Provides the current system snapshot. Handles active tasks, agent states, execution context.
├── time/                   # Schedulers, delayed execution and recurring tasks. Allows agents to schedule tasks, revisit goals, maintain long-term objectives.
├── utilities/              # Utility implementations such as logging.
├── verification/           # The "Trust Layer" - gives output validation, plan verification, constraint checking
├── workflow/               # Provides long-running workflows (hours/days/weeks), checkpointing, resumability, distributed execution across agents
└── world_model/            # Enables planning accuracy, simulation quality, long-term reasoning. See note.
```

> The executor's responsibility is to convert plans -> executable graphs. Those graphs run independent of LLM.

> The planner allows us to swap planning strategies and add symbolic planners

> Examples of the events crate are TaskStarted, ToolFailed, MemoryUpdated, SkillLearned

> The context crate is essential for growing context windows and for multi-agent coordination

> The reflection crate also tracks outcomes, adjusts routing decisions and evolves behavior over time.

> The world_model builds internal representations (entities, systems, dependencies), tracks causal relationships, supports reasoning beyond text.

> Orchestration handles planning/control.

> Coordination handles real-time interactions.
