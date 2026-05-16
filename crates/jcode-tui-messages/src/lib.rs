//! Message-list rendering, wrap-width math, and the per-message line cache used
//! by the main TUI transcript view.

mod cache;
mod message;
mod prepared;
mod wrapped_line_map;

pub use cache::{
    MessageCacheContext, centered_wrap_width, get_cached_message_lines,
    left_pad_lines_for_centered_mode,
};
pub use message::DisplayMessage;
pub use prepared::{
    CopyTarget, EditToolRange, ImageRegion, PreparedChatFrame, PreparedMessages, PreparedSection,
    PreparedSectionKind,
};
pub use wrapped_line_map::WrappedLineMap;
