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

## 16:28–16:30 UTC — exact-source validation and outcome

### What was done (commands, files, features)
- Read failed [Actions #44125685729](https://github.com/isogeny-labs/crypto/actions/runs/44125685729): original **40/40 Rust library** tests passed; new example E0499 single nested borrow at `ar.sqrt(ar.rhs(..))`. Committed narrow source-only borrow fix `6b485f3`, same frozen protocol and fixtures; no post-data tuning.
- Final [Actions #44125986122](https://github.com/isogeny-labs/crypto/actions/runs/44125986122) **SUCCESS**: native library 40/40, comparator 1/1, release JSON assertions, pinned Rust 1.100, original arithmetic ref `1df7146`, bounded virtual memory. Downloaded [artifact #14346464226](https://github.com/isogeny-labs/crypto/actions/runs/44125986122/artifacts/14346464226); independently SHA256-hashed ZIP in the container: `e5fe9fcb11dab0e15acde7a12644f3d13cb5935a859b2fe46702658b53d61212`, exactly GitHub's artifact digest.
- Replayed **256/256** planted signed masks (256 distinct group points), B8 three frozen fixtures match existing chart outcome. Precompute 16+16 points, 128 original group additions, 27,161 ns. One favorable fixed positive generic query 311ns (1 probe); two off-support queries 4,066ns and 4,076ns (16 probes). Existing RR9 chart on same three R: 139,740ns positive and 136,144/136,204ns negatives (256 sign masks each). 1,024 distinct blinded public Q: zero hits, 16,384 query group additions, 4,197,002ns online.
- Wrote `RR9_B8_GENERIC_MITM_RESULT_20261117.md` with exact provenance, historical 1024 full-solver frontier separated, bounded literature and previous-lead review, prioritized next experiments. Retained earlier failure and avoided any complete-solver speedup claim.

### Decisions made (and why)
- Simple B8 256-sign interpolation is a correct but generic-equivalent phase and is not an attractive performance lead relative to 16+16 precomputed point-law MITM. The group-law exact support predicate is more permissive than the RR9 *normalized chart* on exceptional cases, so the comparison claims fixed-fixture equality only.
- Next decisional test: sign-independent public-R seven-equation sparse F4/quotient with negative unit certificate and independently replayed positive, then B31 all-multiplicity natural-Q extraction. No full ECDLP measured in this run.

### Open threads (unfinished, pick up next time)
- Build and preregister seven RR9 quadrics quotient/dimension/negative-unit F4 experiment on original native Rust, no sign-mask inputs; perform B31 scaling only if extraction works. Recheck base/current source before editing.
- Complete B31 unknown-target solver comparisons including shared BSGS and external IC frontier. Independent signed PCC unavailable.

### Errors (what went wrong, how it was handled)
- Initial compiler E0499 documented and fixed without algorithm/parameter changes. Signed independent PCC not available; all new measurements are producer CI. No other failures on final source.

## 16:30 UTC — review publication

### What was done (commands, files, features)
- Updated PR #4575 with explicit numeric source-CI results and the audit receipts, then marked it **ready for review**. GitHub reported the branch as mergeable.

### Decisions made (and why)
- Leave the validated PR open rather than bypass the normal approval/CI review route via a forced manual merge.

### Open threads (unfinished, pick up next time)
- Repository maintainer can merge with an allowed method after remaining reviews/checks; preserve producer-only validation status.

### Errors (what went wrong, how it was handled)
- GitHub `enable_auto_merge` rejected the integration's selected method: **"Merge method squash merging is not allowed on this repository"**. This is a merge-method configuration blocker, not a Rust test failure; no merge or status claim made.
