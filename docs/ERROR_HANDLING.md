# Error Handling Policy

Working policy for error types across the jcode workspace. Adopted incrementally; not every crate is migrated yet.

## TL;DR

| Where                       | What to use                                                                      |
| --------------------------- | -------------------------------------------------------------------------------- |
| Library crate (`crates/*`)  | A typed `enum`, derived with `thiserror`. Define `pub type Result<T>` per crate. |
| Binary crate (`src/`)       | `anyhow::Result<T>` at the boundary; convert typed errors via `?`.               |

## Rationale

Library callers may want to distinguish error variants (retry on transient I/O, surface specific UI for credential failures, etc.). `anyhow::Error` erases that information. `thiserror` keeps variants explicit and free of overhead, while still implementing `std::error::Error` so that `anyhow::Error` accepts them through `?`/`From`.

## Recipe for a leaf crate

1. Add `thiserror = "1"` to the crate's `Cargo.toml`. Drop the `anyhow` dependency if nothing else needs it.
2. Define the error enum at the top of `lib.rs` (or in a small `error.rs` if the surface grows):

   ```rust
   use thiserror::Error;

   #[derive(Debug, Error)]
   pub enum MyCrateError {
       #[error("failed to do X: {0}")]
       DoX(#[from] some_dep::Error),

       #[error(transparent)]
       Io(#[from] std::io::Error),
   }

   pub type Result<T> = std::result::Result<T, MyCrateError>;
   ```

3. Change public function signatures to return the crate-local `Result<T>`.
4. Where the crate previously called `anyhow::bail!` or `anyhow!`, switch to the appropriate variant. Prefer adding a variant over `#[error(transparent)]` if the failure has a name worth distinguishing.

## Caller-side migration

When a crate migrates, its existing call sites in `src/` continue to work with one of two small adjustments:

| Old pattern                              | New pattern                                                     |
| ---------------------------------------- | --------------------------------------------------------------- |
| `let v = my_crate::f(...)?;`             | unchanged — `?` lifts via `From<MyCrateError> for anyhow::Error`. |
| `my_crate::f(...)` returned directly     | `Ok(my_crate::f(...)?)` to bridge the return type.              |
| `match my_crate::f(...)` consuming `e`   | unchanged — `Display` impl is preserved.                        |

If a downstream caller needs to branch on the specific variant, downcast:

```rust
match err.downcast_ref::<MyCrateError>() {
    Some(MyCrateError::DoX(_)) => { ... }
    _ => { ... }
}
```

## What stays in `anyhow`

- The binary's top-level result types in `src/main.rs`, CLI dispatch, and tool execution.
- Code paths where the error is purely surfaced to the user as a string and no variant-specific behavior is needed.

## Migration tracking

Workspace crates that still use `anyhow::Result` directly (count of files):

- `jcode-build-support` (4)
- `jcode-desktop` (5)
- `jcode-embedding` (1)
- `jcode-mobile-core` (3)
- `jcode-mobile-sim` (4)
- `jcode-notify-email` (1)
- `jcode-protocol` (6)
- `jcode-provider-core` (1)
- `jcode-provider-gemini` (1)
- `jcode-selfdev-types` (1)
- `jcode-storage` (1)
- `jcode-tool-core` (1)
- `jcode-tui-mermaid` (2)
- `jcode-update-core` (1)
- `jcode-azure-auth` (1)

Already migrated:

- `jcode-pdf`
- `jcode-terminal-launch`
