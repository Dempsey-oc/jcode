# jcode-agent-runtime

_Workspace crate_

Agent runtime primitives: soft-interrupt signaling and shared run-state types
used by the conversation loop to coordinate cancellation and tool injection.

## Layout

Source root: [`src/`](src/)

## Dependencies

thiserror, tokio

See [`docs/CRATE_OWNERSHIP_BOUNDARIES.md`](../../docs/CRATE_OWNERSHIP_BOUNDARIES.md) for the rules governing what does and does not belong here.
