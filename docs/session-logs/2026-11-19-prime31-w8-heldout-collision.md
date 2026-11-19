# Prime31 held-out signed-weight and EC collision audit — 2026-11-19 (PR #4807)

## 2026-11-19 12:18 UTC — What was done (commands, files, features)
- Inspected current `AGENTS.md`, `docs/ACTIVE.md`, #4692, #4728, #4805, #4803 and #4793 before selecting distinct task. Read existing #4805 Rust source, verified report, frozen JSON, session log, and original-source CI workflow. Existing W1 power-of-two halves each reported 65,496 vs formal maximum 65,536. Prior 256-target pilot 205 old vs 226 new is not held-out.
- Reviewed PKM 2016, APS 2018 and Kudo et al. prime-field index-calculus limitations; reviewed original GitHub previous directions listed in protocol. KB MCP/wrapper unavailable in connector runtime; bounded audit.
- Created feature branch, committed active-task line, opened draft PR #4807, linked PR in ACTIVE. Preregistered `W8_HELDOUT_COLLISION_PROTOCOL_20261119.md` with held-out seed `48454c444f555431`, 1024 public points, exact collision certificates, original pinned source and no generic/non-generic confusion.

## Decisions made (and why)
- Investigate **non-formal finite-group** collisions separately from pure-integer signature collisions and held-out hit frequency. Leave the actual RR9/F4 public-only extractor to #4692, avoiding duplicate candidate work.
- Reuse original Rust arithmetic; use generic 4+4 MITM **only as a grading oracle**. Do not promote whole-solver or ECDLP advantage from graded hits.

## Open threads (unfinished, pick up next time)
- Implement and run exact W1 collision witnesses + 1024 held-out natural public points in the original Rust crate on source-specific CI; independently replay an accident witness.
- Publish all actual counts, raw evidence, and next nonenumerative extractor gates.

## Errors (what went wrong, how it was handled)
- No local Rust toolchain or GitHub DNS in container. Source-specific GitHub Actions must run the new Rust example; distinguish lack of local test from CI success.
