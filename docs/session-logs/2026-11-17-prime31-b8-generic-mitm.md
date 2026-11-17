# 2026-11-17 — prime31 B8 generic MITM comparison (#4575)

## 16:21–16:29 UTC — tracking, literature and code registration

### What was done (commands, files, features)
- Read current `AGENTS.md`, `docs/ACTIVE.md`, and recent prime31 B8 barycentric session log; checked shared 2026-11-17 log's last substantive entry. Fetched #4558/#4556 merged results, #4532 cached BSGS complete 1024-Q benchmark, #4550 D9 theorem, #3301 lead atlas, #3305 Semaev-alternatives audit, and existing native `prime31_s9_b31_half_sum_oracle.rs` to avoid duplicating its B31 exact eight-sum oracle.
- Research-KB MCP `kb_status` and `search_knowledge` tools absent from this tool catalog. No checked-out repo or catalog wrapper available; **KB unavailable**, bounded GitHub repository and primary-publication fallback. Checked Petit–Kosters–Messeng PKC 2016, Amadori–Pintore–Sala FFA 2018, Shoup EUROCRYPT 1997. No new mathematical novelty claim.
- Created branch `automation/prime31-b8-mitm-control-20261117`, tracked task in `docs/ACTIVE.md` first, opened draft [PR #4575](https://github.com/isogeny-labs/crypto/pull/4575), registered protocol `RR9_B8_GENERIC_MITM_PROTOCOL_20261117.md` before the code.
- Added independent native Rust example `prime31_rr9_b8_generic_mitm.rs`: precompute 16 left and up to 16 deduplicated right signed half sums using original curve group law; exact public residual membership; original-curve witness replay; all 256 planted signed masks; 1024 distinct deterministic scalar-blinded public Q; fixed B8 positive/two off-support negatives; same-run call to merged sign-enumerative RR9 chart controls. Measurements separate construction and online group additions/wall time, no full B31 logs.
- Added source-pinned Rust 1.100.0 Actions workflow `.github/workflows/prime31-rr9-b8-generic-mitm.yml`, reusing immutable native arithmetic commit `1df714645aa683cf975bc76fcacb9842cb4780fd`. Two CI workflows started from same exact code SHA `cbb0a38d90b793487ab29ef6d8eefb3aaded07df`: #44125685729, #44125672033. Source-specific results pending at log creation.

### Decisions made (and why)
- A 4+4 point-law search is by construction generic, never non-generic IC; use it solely as a contemporaneous correctness and phase-cost falsifier for the B8 256-sign-mask RR9 code. B31 4+4 generic oracle already exists; do not claim this duplicates or improves it.
- Compare **fixed monic RR9 charts only on its preregistered three nonexceptional residuals**, and use generic full-signed-sum membership on 256 planted cases and 1024 random public Q. Some generic positives can lie on excluded RR9 charts; no broad equality claim.
- Keep the complete 1024-Q cost barrier from merged #4532 as historical evidence, not the cost of this component.
- No paid compute or infrastructure changes. GitHub-hosted source-pinned CI provides producer measurements only.

### Open threads (unfinished, pick up next time)
- Read CI result, correct any compilation defects on same fixed protocol, preserve failure and final-success evidence, read back JSON counters.
- Document exact source run, separate measured phase timing from unmeasured solver extrapolation; update task checklist and move PR to ready only after passing tests.
- Next genuinely non-generic experiment: public-R-only seven-of-eight sparse quotient/D9 gate without selecting signed leaves, then B31 repeated-root, chart-completeness and full unknown-log recovery with shared-BSGS and strongest index-calculus competitors.

### Errors (what went wrong, how it was handled)
- Tool environment has GitHub connector but no `cargo` binary and no local checkout; delegated native test/build to source-specific GitHub Actions instead of claiming local compilation.
- Research-KB MCP unavailable; fallback scope recorded above. No experimental numeric outcome asserted before runner completion.
