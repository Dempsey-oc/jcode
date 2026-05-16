# jcode-tool-core

_Domain core_

Async `Tool` trait and shared tool-execution scaffolding consumed by every
built-in tool. Defines the contract between the runtime and tool impls.

## Layout

Source root: [`src/`](src/)

## Dependencies

anyhow, async-trait, jcode-agent-runtime, jcode-message-types, jcode-tool-types, serde_json, tokio

See [`docs/CRATE_OWNERSHIP_BOUNDARIES.md`](../../docs/CRATE_OWNERSHIP_BOUNDARIES.md) for the rules governing what does and does not belong here.
