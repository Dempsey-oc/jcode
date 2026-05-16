# jcode-protocol

_Type-only DTO crate_

Client-server protocol for jcode: newline-delimited JSON over a Unix socket, with the server streaming events back to clients during message processing.

Socket types:

- **Main socket** — TUI/client communication with the agent.
- **Agent socket** — inter-agent (AI-to-AI) communication.

## Layout

Source root: [`src/`](src/)

See [`docs/CRATE_OWNERSHIP_BOUNDARIES.md`](../../docs/CRATE_OWNERSHIP_BOUNDARIES.md) for the rules governing what does and does not belong here.
