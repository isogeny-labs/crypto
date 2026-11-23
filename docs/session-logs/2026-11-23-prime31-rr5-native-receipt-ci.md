# Prime31 RR5 native receipt qualifier — 2026-11-23 UTC

## 2026-11-23 UTC — initial source-bound Rust test run and compiler repair

### What was done (commands, files, features)

- Read live `AGENTS.md`, `docs/ACTIVE.md`, the large shared daily log availability, and the dated `2026-11-23-prime31-compact-dual.md` handoff. Prior merged PRs [#4969](https://github.com/isogeny-labs/crypto/pull/4969), [#5020](https://github.com/isogeny-labs/crypto/pull/5020), [#5033](https://github.com/isogeny-labs/crypto/pull/5033) were not reopened: #5033 merged at 2026-11-23T17:37:31Z. Frozen original verifier blob: `98b760116e03c91461bf33741cb7b76c7d86a5ac`. `main` observed `b2dd12810c063c5eb3733d5db2a9d0f96cf5fcac` at branch creation.
- Began a fresh isolated branch `automation/prime31-rr5-native-receipt-gate-20261123`; ACTIVE-only tracking commit `2d8ea8716610343087a642ff61685bfeda019579`, draft tracking [PR #5053](https://github.com/isogeny-labs/crypto/pull/5053).
- Added `.github/workflows/prime31-rr5-receipt-check.yml`: opt-in narrow-path original-Rust runner on GitHub's ubuntu-24.04, Rust 1.100.0, `CARGO_BUILD_JOBS=2`, `RUST_TEST_THREADS=1`, 20-minute job cap, no scientific benchmark; records source SHA, toolchain, dependency lock, compiler JSON messages, executable digest, test logs/exit, checked source hash and artifacts. Sparse-checkout isolates the original checker, original native Rust library and shared F4 source, avoiding the full 100k-file project.
- First [Actions #45238680831](https://github.com/isogeny-labs/crypto/actions/runs/45238680831) on `29f64a3ec519d120a2d051081dfd6c61ab45f126` failed at the library compile because the initial sparse checkout omitted the original `src/cryptanalysis/f4_fp_tower.rs` path used through `#[path]`. This is a runner-materialization error, not an algebraic checker test result. Error artifact: [#14851355243](https://github.com/isogeny-labs/crypto/actions/runs/45238680831/artifacts/14851355243). Fixed sparse include plus `sig_fp_tower.rs`; workflow commit `5d3b9546ead57f36935cb627ee32cf71504624de`.
- Second [Actions #45238957031](https://github.com/isogeny-labs/crypto/actions/runs/45238957031) on exact source `5d3b954` compiled the original native adapter and reached the previously uncompiled helper. **Found actual Rust compiler error E0382** in merged #5033 source: `DualPivot { row, ... }` moves `row` on one branch, but `row[base_size]` is accessed after the loop; the borrow checker cannot use `base_pivot_installed` to rule this out. Build artifact is [#14849198852](https://github.com/isogeny-labs/crypto/actions/runs/45238957031/artifacts/14849198852). No tests executed in this failed attempt.
- Repaired the original checker on this isolated branch in commit `f681021df26e402a30522af8a49b7f6eaa349af3`, new blob `d3ebbe79b09266acd541c3804c215c8d8d830a2e`: `row: std::mem::take(&mut row)` leaves an empty Vec in scope, safely transferring the normalized pivot buffer without O(B) clone work. First subsequent CI [#45239200514](https://github.com/isogeny-labs/crypto/actions/runs/45239200514) is the required source-exact native test gate; its result is not asserted until read.

### Decisions made (and why)

- This lane fixes **compiler correctness of an inherited verifier**, not an RR5 extraction, a graph/S3/transfer primitive, or a complete solver speedup. Keep the existing 19 authored Rust test cases untouched until observed compiling/running. Preserve failed CI attempts and the exact failure distinction.
- Reuse current Rust arithmetic, factor-base input and project curve `p=2^31-1`, `n=2147446831` (prime order). Do not alter extracted B8 evidence, B677 producer, frozen PCC claims, maintained baseline results or benchmarking reservations.
- Literature/prior-results review: `research/cheap_ecdlp_cold_gate_20261123/README.md`; merged #4969/#5020/#5033; Semaev 2004 (https://eprint.iacr.org/2004/031); Petit–Kosters–Messeng PKC 2016 (https://www.iacr.org/archive/pkc2016/96140156/96140156.pdf). These identify the unproven prime-field relation-extraction bottleneck; a verifier maintenance fix makes no novelty claim. **Research-KB/Conductor MCP and native subagent launcher absent** in this runtime; fallback was source-exact GitHub connector reads and the prior report.
- Observed prior same-scope negative control remains 24-target ordinary Prime31 APS 156.909 ms vs fresh BSGS 122.565 ms (separate frozen panel). These times are not rerun or compared with new helper CI.

### Open threads (unfinished, pick up next time)

- Obtain an actual source-frozen **Rust** PASS/FAIL with full test stdout and archived executable/lock/toolchain digests on the repaired source; inspect all failures, fix within this lane, retain error receipts, and do not report model-only or authored tests as executed.
- Freeze the package-specific Cargo.lock if needed for exact cross-run reproducibility. Current workflow resolves a lock per run and archives its bytes; that alone does not establish cross-run identical dependency resolution.
- Afterwards run independent receipt replay and original-arithmetic target recovery for an authentic **blind unknown-index B677** accepted relation stream if supplied by RR5 owner; then charge the checker, extraction, setup, target rank and baseline reference as cold cost under resource reservation. No authorized blind B677 stream is currently evidenced.
- Review main/PR state before any push or merge; preserve existing review holds, failed and censored runs, and PCC standing. Draft until source-bound gate is qualified.

### Errors (what went wrong, how it was handled)

- No native `rustc`/`cargo`, Conductor, research-KB, subagent launcher, EKS credentials or local GitHub clone in this execution container; GitHub Actions source-bound validation was substituted. Tool calls read repository and run records concurrently; **one actual coordinator, zero native workers**, no independent signed validator. No scientific benchmark or EKS paid compute was initiated.
- The current canonical shared `docs/session-logs/2026-11-23.md` is >1 MB, `fetch_file` returned empty content and earlier handoff identified unresolved conflict markers. Did not overwrite it; wrote this append-only supplemental dated log.
- No pass is asserted from the first two CI runs. The second failure is an actionable inherited source compiler defect, not evidence against ECDLP algorithm correctness or performance. [pcc:unverified]


## 2026-11-23 UTC — source-bound execution qualified after repair (two green CI runs)

### What was done (commands, files, features)

- Read exact workflow-run job steps and downloaded complete preserved result artifacts for [#45239291528](https://github.com/isogeny-labs/crypto/actions/runs/45239291528) and [#45239466847](https://github.com/isogeny-labs/crypto/actions/runs/45239466847). Both run the original existing Rust checker with Rust 1.100.0 and complete successfully; **19/19 Rust tests passed, 0 failed, 0 ignored** in the actual `test.stdout`. The second run used git SHA `0297bc50c3b986bec1fe3fb451fd6092d7fc5e71`. Parent `test.exit=0`, and the source SHA256 replay returned OK for all eight source/fixture/dependency files. New compiled checker blob `d3ebbe79b09266acd541c3804c215c8d8d830a2e` retains the E0382 move fix.
- Frozen second green artifact [#14850584102](https://github.com/isogeny-labs/crypto/actions/runs/45239466847/artifacts/14850584102) includes executable hash `784492368dbd25cd4fd10b0e53776b24686bee90a80bc60eb5bdae25ea503ec0`, resolved helper lock hash `a6123e8bb420a0ae63745b7a9b285fea0e70930058dd6dd02fe1ccff5c4570f1`, `rustc 1.100.0 (0ef1bc386 2026-11-09)`, compile JSON and stderr, test stdout/stderr, and source hashes. Evidence summary checked into [qualification report](../research/cheap_ecdlp_cold_gate_20261123/RUST_NATIVE_QUALIFICATION_20261123.md).
- Preserved both earlier failures: #45238680831 sparse checkout missing F4 file; #45238957031 inherited Rust E0382 move error (before `std::mem::take` fix). #45239200514 was canceled by serial workflow concurrency; no false pass assigned to it.

### Decisions made (and why)

- Report **original-Rust native test pass at two source-bound revisions** only, not blind relation recovery, full cold speedup or signed PCC. The native test suite checks fixtures and failure admissions but does not validate extracted unknown-index B677 production streams. No competing benchmark, extra automation, producer change, EKS compute, merge or leaderboard promotion.
- Preserve the draft PR until its independent-validation/merge requirements are resolved. Do not touch ownership of RR5 B677, graph, S3, transfer or maintained rho.

### Open threads (unfinished, pick up next time)

- Independently attest the compiled original-Rust run and its known-answer/mutation scope via PCC as permitted; record signed verdict rather than self-declaring it.
- Freeze helper-specific Cargo.lock across runs if mandated by qualification (current exact locks and hashes are archived **per run**, but no lock is committed).
- Obtain actual **blind unknown-index** accepted B677 relation stream from owning producer lane, verify its target separation weights and complete recovery, then charge full cold cost under reserved matched best-IC / optimized-rho / BSGS protocol. Negative/censored rows must remain.

### Errors (what went wrong, how it was handled)

- Earlier sparse-source and moved-value compiler failures are preserved with exact run IDs and artifacts, not overwritten by green runs. One coordinator, **zero spawned native subagents**; independent read operations overlapped but no independently signed validation or elapsed-time speed gain is inferred. [pcc:unverified]
