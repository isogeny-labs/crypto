# Session log: Prime31 weighted six-slot 3+3 Rust index-calculus control — 2026-11-18

## What was done (commands, files, features)

- Read GitHub `AGENTS.md`, `docs/ACTIVE.md`, original native Rust `lib.rs` and `Cargo.toml` at `1df714645aa683cf975bc76fcacb9842cb4780fd`, the existing weighted eight-slot Rust source, current #4714/#4716/#4692 PR descriptions, historical #4623 and #4596 source reports, anchored M6 and M4 support gate reports, plus #4698 and #4711.
- KB unavailable: `kb_status`, `search_knowledge`, `get_source`, etc. are not loaded in this GitHub-connector runtime; a full local checkout and `scripts/research-kb` wrapper are not mounted. Bounded fallback: original repository documents, PR contents, and known primary literature (Semaev 2004; PKM 2016; APS 2018; Diem 2011). No full novelty clearance claimed.
- Tracked work in `docs/ACTIVE.md` on existing `automation/prime31-weighted-slot-parity-20261118` / PR #4714 lane; preserved original experiment.
- Committed `research/prime31_ic_20261115/native/examples/prime31_weighted_m6_complete.rs`: six public weights, B31 original-curve 3+3 XOR-parity, 512 training calls, 31-column modular rank, up to 1,024 caller-blinded unknown logs, 64 shifts, exact equation/point replay, native counters and frozen JSON output.
- Committed `verify_prime31_weighted_m6_complete.py`: separate integer modular and EC group-law reconstruction; natural-witness, rank, base log and 1,024 target replay, with fail-closed contradiction checks.
- Froze parameters, source and prior-lead audit in `PRIME31_WEIGHTED_M6_COMPLETE_PROTOCOL_20261118.md`.
- Added sparse exact-original-source GitHub Actions `prime31-weighted-m6-complete.yml` (pinned Rust 1.100.0, original 40-test library gate, 2 new focused tests, full 1,024-target invocation, independent Python replay, complete raw artifacts).
- Fixed two-phase Rust mutable borrow in candidate code, preserving algorithm, curve and frozen seeds.
- Wrote `PRIME31_WEIGHTED_M6_COMPLETE_RESULT_20261118.md` with a strict pending-results ledger. No wall-speed or non-generic win claimed without runner transcript.

## Decisions made (and why)

- Extended the existing *weighted-slot parity* lane PR #4714 instead of opening a duplicative PR.
- Target distinct scalar images, not independently masked subsets of one same-image base (#4716 bound); six-weight B31 3+3 cuts raw half tuple count from 31⁴ to 31³, but only measured yields/cost determine value.
- Retain generic-equivalent classification; meaningful novel-mechanism decision remains switching from group sums to genuine RR9/S4/F4 target extraction in #4692.
- Censorship is an explicit outcome: a full-rank training run without 1,024 solved Q is not a complete success. No BSGS/rho fallback, no post-hoc seed/weight/shift retuning.
- Signed independent PCC standing remains unverified; exact-source CI and separate independent arithmetic are narrower checks.

## Open threads (unfinished, pick up next time)

- Inspect the latest exact-source GitHub Actions run for the candidate SHA and distinguish compile, replay, rank and target failures; repair code-only bugs without changing registered data.
- Fill in exact result table, operation counts and status from the raw JSON/independent verifier. Request separate independent PCC verdict.
- Run complete same-runner IC, optimized shared BSGS and rho comparison only after successful rank/log gate, with memory, failed calls and cold setup.
- Replace generic six-slot point-table extraction by truly nonenumerative, sign-independent algebraic target-dependent extraction (existing RR9 #4692); rerun same 1,024 Q and prove equal recovery before an algorithmic-speed claim.

## Errors (what went wrong, how it was handled)

- Runtime does not expose `cargo`/authenticated clone or shared KB, so no local native compile or original-source replay was claimed. Instead source-bound GitHub Actions was committed and launched by branch push.
- GitHub `fetch` disallowed direct `/actions/workflows/{workflow}/runs` URI; querying the permitted repository-wide `/actions/runs?branch=...` URI returned the execution, and is used for status.
- Corrected nested mutable Rust borrow in `first_rational_points` and `query` before relying on native compilation.

## 2026-11-18T11:35:00Z — Exact source-bound Rust M6 whole-solver result

### What was done (commands, files, features)

- Executed [source run #44294687054](https://github.com/isogeny-labs/crypto/actions/runs/44294687054) at candidate `ca48659a101447b6251661300358dcfe45177c85`: 40/40 inherited Rust tests, 2/2 new tests, original-curve rank 31/31 from 31 parity relations across 106 training calls, 1,024/1,024 public unknown Q solved over 3,717 shifted calls, no generic fallback; measured complete wall 29,082.809 ms. Independently authored BigInt replay PASS all training/base/Q equations. Signed PCC unverified.
- Froze a separate Montgomery batched-complement experiment protocol before modifying the candidate source. Implemented a second Rust arm `examples/prime31_weighted_m6_batch.rs` using one inverse per batch plus exceptional-case handling, preserving original source/group/weights/corpus.
- Executed [AB/BA exact-source run #44295520583](https://github.com/isogeny-labs/crypto/actions/runs/44295520583) on same Ubuntu 24.04 host, Rust 1.100.0, exact source `1cab702112f7d3af371d6b5af349d5a443615a11`. All 40 inherited library tests, 2+2 focused tests, four independent Python EC BigInt replay transcripts and matched determinant/rank/target/witness equality gates passed.
- Scalar vs batched complete wall: 43,428.237 vs 5,812.828 ms A/B and 43,510.030 vs 5,988.724 ms B/A, paired speedups 7.471× and 7.265×; median 7.368×. Query-only Fp inversions 110,831,708 to 102,278, multiplications 7,093,290,578 to 670,994,789; identical 110,834,780 counted EC additions.
- Artifact [14422898426](https://github.com/isogeny-labs/crypto/actions/runs/44295520583/artifacts/14422898426) downloaded and SHA256 rehashed locally `64f26701b748076a335626e7cdc517a0746e51a4a524fd25c62d5792375353cf`. Full original-curve evidence remains in Actions artifact (30 days); durable summary JSON and full scientific report now committed alongside protocol, source, verifier and workflows. Producer + separate verifier, not independent signed PCC.

### Decisions made (and why)

- A 7.368× whole-program improvement is **relative to exactly matched scalar-inversion six-weight index-calculus control only**, not to shared BSGS or optimized rho, whose same-host timings were not collected. It is **generic-equivalent**, because parity extraction still enumerates generic 3+3 point sums.
- Keep the significant applied result and follow-up code checked into original PR #4714, without rebranding it as a non-generic cryptanalytic breakthrough. The original previous eight-slot experiment remains preserved.
- Next priority is algebraically conditioned RR9/F4/rootfinding extraction from **public residuals** without generic 3+3 tables; then direct full-solver matched external frontier under complete costs.

### Open threads (unfinished, pick up next time)

- Independently sign PCC for all full-cost claims and validate replay source/ZIP.
- Freeze a same-runner optimized IC/shared BSGS/rho comparison using these exact 1,024 Q. Existing separate-host BSGS ≈170ms suggests that algorithmic advantage still demands non-generic extraction.
- Investigate per-slot disjoint low-degree Dickson fibres vs scalar images (up to 186 unknowns vs 31, respectively) for future RR9 algebraic extraction.
- Preserve raw complete target transcript durably beyond 30-day Actions artifact when feasible and permitted by repo binary/data policy.

### Errors (what went wrong, how it was handled)

- Initial source-specific v1 CI #44294590163 failed Rust compilation due to nested mutable borrow. Fixed in `ca48659...`, reran successfully; no change to frozen inputs.
- Older pinned main source compiles only through sparse focused crate, not full repository; focused 40/40 regression is the stated native validation.
- No research-KB runtime, independent signed PCC or same-host mature external algorithm comparator available. These evidence limits remain clearly reported. [pcc:unverified]
