# Qbit AI Context Entry Point

## Repository role

`qbit-cli` is the CLI runtime in the Qbit multi-repository workspace. Current source, tests, configuration, explicit contracts, and committed local engineering decisions remain authoritative for implementation facts owned by this repository.

<!-- qbit-toolkit:ai-context:start -->
## Zero-touch lifecycle

Before substantive Codex work, the agent automatically runs `.ai/context/context.ps1 start` and reads `.ai-bridge/context-runtime.md`. The launcher clones or safely refreshes the central context into the ignored `.ai/context/cache/project-context` cache.

After a substantive validated milestone that changes durable continuity state, the agent creates `.ai-bridge/context-checkpoint.json` and runs `.ai/context/context.ps1 checkpoint`. Checkpoints are milestone-driven, not per-message.

The canonical context source is `https://github.com/qbit-click/qbit-ai-context.git` on branch `main`.

## Authority and safety

AI context is coordination evidence, never implementation authority. Current source, tests, schemas/migrations, explicit contracts, and committed canonical decisions outrank stored context according to claim type. Preserve pre-existing uncommitted work, never store secrets or raw chat transcripts in context, and do not use destructive Git recovery to resolve context failures.
<!-- qbit-toolkit:ai-context:end -->
