//! Cross-domain primitives shared across the workspace: env access, filesystem
//! helpers, id generation, panic utilities, stdin detection, and small utils.
//!
//! See `docs/CRATE_OWNERSHIP_BOUNDARIES.md` for what does and does not belong here.

pub mod env;
pub mod fs;
pub mod id;
pub mod panic_util;
pub mod stdin_detect;
pub mod util;
