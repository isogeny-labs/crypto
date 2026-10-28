## 2026-10-28T17:25:00Z — classical APS and guess-and-determine archive

### What was done (commands, files, features)

- Preserved 19 original classical investigation files byte for byte under `research/classical_ecdlp_20261028/historical/`, with `HISTORICAL.sha256`. The 14-record source catalogue retains access gaps; third-party papers and author code are linked rather than redistributed.
- Added `examples/classical_ecdlp_archive.rs`, reusing the existing W12 engine. Native replay verifies 60 public known-answer cases, 120 recorded arm answers, six full matrix/signature/minor/point-sum witnesses and six exact curve orders. Six native trial-zero solver checks and 84 small APS S3/sign checks pass.
- `cargo test --release --locked --example classical_ecdlp_archive -- --test-threads=2` passes all 14 tests, including three witness-corruption tests and seven inherited engine tests. Clippy with `-D warnings`, scoped rustfmt and the historical SHA-256 manifest checks pass. The `check`, `summary` and `solve` CLI receipts are saved in `validation/`.
- Added a source-linked report, editable SVG contribution map and finite-count graph, rendering source and two-page PDF. Visually inspected both pages; final page renders match the inspected bytes. Artifact-policy planning reports zero files requiring the store.
- Started required `cargo test --release --lib --locked -- --test-threads=4`. Nineteen tests initially lacked tracked runtime inputs in the sparse checkout; all 19 pass in exact focused reruns after materializing their original inputs. One unchanged end-to-end library test remains active at this entry; no clean full-suite pass is claimed.
- Fast-forwarded the isolated branch to `7be0960b07b222b54b32c5a7813e649d9563071a`. Compared it with tested base `6187036c049b6de2b1521b014f575952e4f1705c`: library, Cargo files, reused engine, crates and tools are unchanged. Upstream changes concern another study and `src/bin/icprog`.

### Decisions made (and why)

- Kept Python implementations and measurements as frozen historical evidence; the active correctness/replay path is Rust. Python is used only to render recorded counts and document layout.
- Kept all research leads classical. Distinguished APS's one-relation construction from the adjacent matrix/minor method, which still uses matrix algebra. Reused the earlier repository W12 implementation and audit rather than claiming a new solver or counting theorem.
- Added no benchmark/leaderboard row, `VALIDATION.json` or progress metric. Historical Python timings lack host/isolation metadata and the strongest matched rho reference; this import runs no new performance experiment and changes no admitted result or incumbent.

### Open threads (unfinished, pick up next time)

- Publish the ready PR from `codex/classical-ecdlp-replication-20261028`, enable auto-merge, and follow current-head `gate`, `size`, `artifacts` and review feedback through merge.
- Record the eventual outcome of the still-running unchanged `jv_isogeny_walk` end-to-end library test on the PR. Its isolated diagnostic also remains pending; neither run has been represented as passed.
- Full CANS chapter and Lanteri thesis access remain open; a qualified native performance comparison would need its own protocol, required environment and matched rho evidence.

### Errors (what went wrong, how it was handled)

- Conductor checks could not reach the local control plane (`localhost:8080`, connection refused). No lease is claimed; changes stayed in the isolated study, new example and append-only session log.
- Sparse checkout initially omitted a path dependency and compile-time fixtures, then 19 runtime-fixture tests failed. Materialized the tracked inputs; focused reruns of every affected test pass. No source assertion was changed, skipped or quarantined.
- Matplotlib emitted trailing whitespace in SVG paths; the rendering source now normalizes that formatting. Final staged diff checks pass. The original historical files remain unchanged.
- Automatic approval review rejected uploading the shared daily session log, citing unrelated infrastructure details in its existing contents. That file is excluded from the PR; this task-only extract preserves the new work record without republishing unrelated content.

## 2026-10-28T17:38:00Z — library validation and ready PR

### What was done (commands, files, features)

- Published ready PR #2797 with the 33-file classical package. The GitHub tree matched the reviewed local index exactly; the shared daily log is absent from the PR.
- The original full library command completed: 4,968 passed, 26 failed, 122 ignored, 2,576.51 seconds. The formerly outstanding `jv_isogeny_walk` test passed. All 26 failures were missing tracked inputs in the sparse checkout.
- Restored the remaining frozen CryptoPro constants and ran every affected test exactly. The initial 19 and later seven all pass. All 4,994 non-ignored library tests therefore pass across the full run and focused replays; no clean full-suite invocation is claimed. Stopped the duplicate isolated diagnostic after the original test passed.
- Merged current `main` at `bd58c75eb86efe9bec36868ac01b9a152bd298e3` without conflicts. Its four workflow/script changes leave the library, Cargo files and reused solver unchanged.

### Decisions made (and why)

- Kept the failed full-run counts and successful fixture replays explicit. Restoring missing tracked data needed no test or source changes.
- The connector's auto-merge action requested squash, which the repository's rules forbid. Continue with a merge commit after the current-head merge gates and review are satisfied; no alternative merge method or protection bypass is authorized.

### Open threads (unfinished, pick up next time)

- Publish the base update and final development receipts, update the PR description, and follow current-head `gate`, `size`, `artifacts` and review through merge.
- The historical source-access and qualified-performance gaps recorded above remain unchanged.

### Errors (what went wrong, how it was handled)

- Seven additional missing CryptoPro-fixture failures appeared when the full suite finished. Restoring the committed constants made all seven exact reruns pass.
- Auto-merge could not be enabled through the connector because it selected the disallowed squash method. No check or repository rule was weakened.
