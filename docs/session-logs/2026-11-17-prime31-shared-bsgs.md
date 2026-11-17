# Session handoff — 2026-11-17 prime31 shared precomputation benchmark

## 2026-11-17 ~06:25 UTC
### What was done (commands, files, features)
Reviewed AGENTS.md, docs/ACTIVE.md, newest 2026-11-17 session log and PR #4513's exact-source Rust source/report, #4524/#4525 concurrent work, #4503/#4466 and prior BSGS source. Prior work confirmed: full 1024-target mixed IC 2684.798ms vs native unfolded rho32 3017.608ms (producer-only), but no shared-advice generic comparator. Created tracking branch and draft PR #4532. Literature review: Shanks BSGS reuse of baby table, published APS 2018 and repo precomputed-rho correction. KB MCP is unavailable here, fallback used.
### Decisions made (and why)
Freeze matched reusable BSGS with m=46341,262144,1048576 and same 1024 Q; original Rust Jacobian arithmetic, full setup/memory charged, no selection based on favorable timings. Generic cost frontier determines whether earlier apparent IC advantage survives a fair comparator.
### Open threads
Implement exact-source append-only BSGS suffix and CI workflow; execute checks, retain raw output, independently replay answers; report comparison and try nonenumerative RR9/S9 next.
### Errors
No Rust toolchain in this runtime; source-specific GitHub Actions required for compilation and actual measurements. No fresh result or speedup yet. [pcc:unverified]
