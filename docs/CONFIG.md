# Configuration Reference

jcode is configured via `~/.jcode/config.toml` (or `$JCODE_HOME/config.toml`). Environment variables override values from this file. The file is optional; every field has a default.

The authoritative shape lives in [`crates/jcode-config-types/src/lib.rs`](../crates/jcode-config-types/src/lib.rs). The root `Config` struct that ties it all together is in [`src/config.rs`](../src/config.rs).

## Top-level sections

| Section          | Purpose                                              |
| ---------------- | ---------------------------------------------------- |
| `[keybindings]`  | Key bindings for scrolling, model switching, etc.    |
| `[dictation]`    | External speech-to-text integration                  |
| `[display]`      | Diff rendering, animation, mouse capture, UI options |
| `[features]`     | Feature toggles (memory, swarm, update channel)      |
| `[auth]`         | Trusted external auth sources                        |
| `[provider]`     | Default provider/model and provider-specific options |
| `[providers.*]`  | Named provider profiles (self-hosted, OpenRouter)    |
| `[agents]`       | Agent-specific model overrides (swarm, memory)       |
| `[ambient]`      | Ambient mode (autonomous background work)            |
| `[safety]`       | Notifications: ntfy, email, Telegram, Discord        |
| `[gateway]`      | WebSocket gateway for iOS/web clients                |
| `[compaction]`   | Context compaction strategy and thresholds           |
| `[autoreview]`   | End-of-turn code review                              |
| `[autojudge]`    | End-of-turn execution judging                        |

## Minimal example

An empty `config.toml` is valid. A more useful starting point:

```toml
[provider]
default_provider = "claude"
default_model    = "claude-opus-4-7"

[display]
diff_mode = "inline"
centered  = false

[features]
memory          = true
swarm           = true
update_channel  = "stable"
```

## `[provider]`

Selects the default provider and tunes provider-specific behavior.

```toml
[provider]
default_provider = "claude"            # claude | openai | copilot | openrouter | <named profile>
default_model    = "claude-opus-4-7"
openai_reasoning_effort = "low"        # none | low | medium | high | xhigh
openai_transport        = "auto"       # auto | websocket | https
openai_service_tier     = "priority"   # priority | flex
openai_native_compaction_mode = "auto" # auto | explicit | off
openai_native_compaction_threshold_tokens = 200000
cross_provider_failover = "countdown"  # see CrossProviderFailoverMode
same_provider_account_failover = true
copilot_premium = "normal"             # normal | one | zero
```

## `[providers.<name>]` (named profiles)

Define one or more self-hosted or third-party OpenAI-compatible endpoints. Each table becomes a selectable provider in the picker.

```toml
[providers.my-gateway]
type          = "openai-compatible"    # openai-compatible | open-router
base_url      = "https://llm.example.com/v1"
auth          = "bearer"               # bearer | header | none
api_key_env   = "MY_GATEWAY_API_KEY"
default_model = "llama-3.3-70b"
provider_routing = false
model_catalog    = false

[[providers.my-gateway.models]]
id = "llama-3.3-70b"
context_window = 128000
```

## `[display]`

```toml
[display]
diff_mode       = "inline"   # off | inline | full-inline | pinned | file
diagram_mode    = "pinned"   # none | margin | pinned
markdown_spacing = "compact" # compact | document
queue_mode       = false
auto_server_reload = true
mouse_capture    = true
centered         = false
show_thinking    = false
pin_images       = true
idle_animation   = true
prompt_entry_animation = true
disabled_animations = []     # e.g. ["donut", "orbit_rings"]
diff_line_wrap   = true
performance      = ""        # "" (auto) | full | reduced | minimal
animation_fps    = 60        # 1-120
redraw_fps       = 60        # 1-120
prompt_preview   = true
```

## `[features]`

```toml
[features]
memory             = true
swarm              = true
message_timestamps = true
update_channel     = "stable"  # stable | main
```

## `[compaction]`

```toml
[compaction]
mode = "reactive"              # reactive | proactive | semantic
lookahead_turns = 15
ewma_alpha = 0.3
proactive_floor = 0.40
min_samples = 3
stall_window = 5
min_turns_between_compactions = 10
topic_shift_threshold = 0.45
relevance_keep_threshold = 0.65
goal_window_turns = 5
```

Thresholds for when compaction triggers (80% by default, 95% critical) are constants in [`crates/jcode-compaction-core`](../crates/jcode-compaction-core/src/lib.rs).

## `[ambient]`

```toml
[ambient]
enabled               = false
provider              = "claude"        # auto-select if unset
model                 = "claude-opus-4-7"
allow_api_keys        = false           # if false, only OAuth providers run ambient
api_daily_budget      = 1000000         # tokens/day; unset = unlimited
min_interval_minutes  = 5
max_interval_minutes  = 120
pause_on_active_session = true
proactive_work        = true
work_branch_prefix    = "ambient/"
visible               = true
```

## `[safety]`

Push notifications and reply-to-agent integrations. All are off by default.

```toml
[safety]
ntfy_topic            = "my-jcode-topic"
ntfy_server           = "https://ntfy.sh"
desktop_notifications = true

email_enabled       = false
email_to            = "you@example.com"
email_smtp_host     = "smtp.gmail.com"
email_smtp_port     = 587
email_from          = "agent@example.com"
# prefer setting JCODE_SMTP_PASSWORD env var instead of email_password
email_imap_host     = "imap.gmail.com"
email_imap_port     = 993
email_reply_enabled = false

telegram_enabled       = false
telegram_bot_token     = "..."
telegram_chat_id       = "..."
telegram_reply_enabled = false

discord_enabled       = false
discord_bot_token     = "..."
discord_channel_id    = "..."
discord_bot_user_id   = "..."
discord_reply_enabled = false
```

## `[gateway]`

WebSocket gateway for iOS/web clients.

```toml
[gateway]
enabled    = false
port       = 7643
bind_addr  = "0.0.0.0"
```

## `[keybindings]`

All keybinding strings accept the format documented in `KeybindingsConfig::default()`. Example overrides:

```toml
[keybindings]
scroll_up         = "ctrl+k"
scroll_down       = "ctrl+j"
scroll_page_up    = "alt+u"
scroll_page_down  = "alt+d"
model_switch_next = "ctrl+tab"
model_switch_prev = "ctrl+shift+tab"
workspace_left    = "alt+h"
workspace_down    = "alt+j"
workspace_up      = "alt+k"
workspace_right   = "alt+l"
session_picker_enter = "new-terminal"   # new-terminal | current-terminal
```

## `[agents]`

```toml
[agents]
swarm_model            = "claude-sonnet-4-6"
memory_model           = "claude-haiku-4-5"
memory_sidecar_enabled = true
```

## `[autoreview]` and `[autojudge]`

```toml
[autoreview]
enabled = false
model   = "claude-opus-4-7"

[autojudge]
enabled = false
model   = "claude-opus-4-7"
```

## `[auth]`

```toml
[auth]
trusted_external_sources       = ["claude-code", "codex"]
trusted_external_source_paths  = ["/path/that/may/read/credentials"]
```

## `[dictation]`

```toml
[dictation]
command      = "say-something --transcribe"   # must write transcript to stdout
mode         = "send"                          # see TranscriptMode
key          = "off"                           # or e.g. "ctrl+space"
timeout_secs = 90
```

## Environment overrides

Several environment variables override config values at startup. Notable ones:

| Variable                      | Purpose                                              |
| ----------------------------- | ---------------------------------------------------- |
| `JCODE_HOME`                  | Override config and state directory                  |
| `JCODE_NO_TELEMETRY` / `DO_NOT_TRACK` | Disable telemetry (see [TELEMETRY.md](../TELEMETRY.md)) |
| `JCODE_SMTP_PASSWORD`         | SMTP password (preferred over `[safety].email_password`) |
| `JCODE_*_API_KEY`             | Provider API keys (see provider docs)                |

For the complete list, search `crates/jcode-core/src/env.rs` and `src/config/env_overrides.rs`.
