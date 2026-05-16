# MCP & Skills Plan Status

Tracks the four phases in [`PLAN_MCP_SKILLS.md`](../PLAN_MCP_SKILLS.md) against what currently exists in the codebase.

| Phase                              | Status      | Reference                                                                                                  |
| ---------------------------------- | ----------- | ---------------------------------------------------------------------------------------------------------- |
| Phase 1 — Hot-reload skills        | Done        | `SkillRegistry::reload`, `reload_all`, `reload_all_for_working_dir` in [`src/skill.rs`](../src/skill.rs)   |
| Phase 2 — Dynamic tool registry    | Done        | `Registry::register`, `unregister`, `unregister_prefix`, `register_mcp_tools` in [`src/tool/mod.rs`](../src/tool/mod.rs) |
| Phase 3 — MCP client               | Done        | [`src/mcp/`](../src/mcp/) — `protocol.rs`, `client.rs`, `manager.rs`, `pool.rs`, `tool.rs`                 |
| Phase 4 — Agent self-configuration | Done (unified shape) | `McpManagementTool` in [`src/tool/mcp.rs`](../src/tool/mcp.rs) — single `mcp` tool with `action ∈ {list, connect, disconnect, reload}` |

## Deviations from the original plan

- The plan proposed four separate tools (`mcp_list`, `mcp_connect`, `mcp_disconnect`, `mcp_reload`). The implementation ships a single `mcp` tool with an `action` discriminator instead. Functionally equivalent; reduces tool-surface noise for the agent.
- The plan listed `src/tool/reload_skills.rs` as a dedicated tool. Skill reload happens through the skill registry directly today; there is no standalone agent-facing tool for it. Add one only if an agent workflow needs it.

## Open follow-ups

- Document the configured MCP servers in user-facing docs (currently only `.claude/mcp.json` and the in-tool description). Track separately.
- No automated integration test exercises a real MCP server end-to-end; protocol-level tests live in [`src/mcp/protocol_tests.rs`](../src/mcp/protocol_tests.rs).
