# jcode-desktop

_Desktop binary_

GUI/desktop entry point for jcode. Wraps the TUI surfaces into a windowed shell with single-session and multi-session modes, model picker, session switcher, power-inhibit handling, and desktop-specific preferences.

This is a binary crate; it has no public library API. See [`docs/DESKTOP_APP_ARCHITECTURE.md`](../../docs/DESKTOP_APP_ARCHITECTURE.md), [`docs/DESKTOP_CODEBASE_ARCHITECTURE.md`](../../docs/DESKTOP_CODEBASE_ARCHITECTURE.md), and [`docs/DESKTOP_SINGLE_SESSION_DESIGN.md`](../../docs/DESKTOP_SINGLE_SESSION_DESIGN.md) for the design.

## Layout

Source root: [`src/`](src/)

See [`docs/CRATE_OWNERSHIP_BOUNDARIES.md`](../../docs/CRATE_OWNERSHIP_BOUNDARIES.md) for the rules governing what does and does not belong here.
