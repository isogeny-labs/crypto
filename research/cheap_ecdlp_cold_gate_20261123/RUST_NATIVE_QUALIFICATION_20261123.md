# Prime31 RR5 public-receipt Rust native qualification — 2026-11-23

**Scope:** original-Rust `rr5_receipt_check` public mixed-RHS checker; no relation extraction, blind unknown-index ECDLP result, elapsed-time acceleration, independent signed PCC, or newly established asymptotic prime-field attack. [pcc:unverified]

## Source and chain of attempts

- Upstream [merged #5033](https://github.com/isogeny-labs/crypto/pull/5033) introduced triangular source-row certificate provenance at Rust Git blob `98b760116e03c91461bf33741cb7b76c7d86a5ac`. Its 19 authored tests had **never run** in that handoff. Upstream [#5020](https://github.com/isogeny-labs/crypto/pull/5020) and [#4969](https://github.com/isogeny-labs/crypto/pull/4969) provide the previously implemented primal/dual checker.
- [Native CI attempt #45238680831](https://github.com/isogeny-labs/crypto/actions/runs/45238680831) at `29f64a3ec519d120a2d051081dfd6c61ab45f126` failed compiling because the workflow's sparse checkout omitted original shared `f4_fp_tower.rs`. Its [artifact #14851355243](https://github.com/isogeny-labs/crypto/actions/runs/45238680831/artifacts/14851355243) preserves the failure. Hydrated the required F4 source and its sibling `sig_fp_tower.rs`.
- [Native CI attempt #45238957031](https://github.com/isogeny-labs/crypto/actions/runs/45238957031) at `5d3b9546ead57f36935cb627ee32cf71504624de` exposed the actual **Rust E0382** compiler defect in the *previously merged* helper: `DualPivot { row, ... }` moves `row`, but a later branch may inspect `row[base_size]`. Tests did not run; [artifact #14849198852](https://github.com/isogeny-labs/crypto/actions/runs/45238957031/artifacts/14849198852) retains the compile diagnostic.
- In [fix commit `f681021df26e402a30522af8a49b7f6eaa349af3`](https://github.com/isogeny-labs/crypto/commit/f681021df26e402a30522af8a49b7f6eaa349af3), replaced the transfer with `row: std::mem::take(&mut row)`, preserving the Vec's allocated buffer in the stored pivot and an empty in-scope Vec for the other branch; **no `O(B)` clone is added**. Fixed checker blob `d3ebbe79b09266acd541c3804c215c8d8d830a2e`.
- [Native CI #45239200514](https://github.com/isogeny-labs/crypto/actions/runs/45239200514) at fix-only `f681021` was canceled by the workflow's one-at-a-time concurrency policy after a documentation push; **not treated as a pass**.
- [Native CI #45239291528](https://github.com/isogeny-labs/crypto/actions/runs/45239291528) at `01f44176a19e4d5b0400a457a330fd74d92e98c3` **passed build and all 19 original-Rust tests**; [source-bound artifact #14851190934](https://github.com/isogeny-labs/crypto/actions/runs/45239291528/artifacts/14851190934).
- [Native CI #45239466847](https://github.com/isogeny-labs/crypto/actions/runs/45239466847) at `0297bc50c3b986bec1fe3fb451fd6092d7fc5e71` also **passed build and all 19 original-Rust tests** with **0 failures, 0 ignored**, and zero measured scientific benchmarks; [source-bound artifact #14850584102](https://github.com/isogeny-labs/crypto/actions/runs/45239466847/artifacts/14850584102). The unchanged source blob under this exact git SHA is the repaired original checker.

## Frozen native execution receipt (second completed green run)

| Field | Source-bound recorded value |
|---|---|
| Git source revision | `0297bc50c3b986bec1fe3fb451fd6092d7fc5e71` |
| Rust toolchain | `rustc 1.100.0 (0ef1bc386 2026-11-09)`, x86_64-unknown-linux-gnu |
| Original checker blob | `d3ebbe79b09266acd541c3804c215c8d8d830a2e` |
| Runner / experiment | GitHub ubuntu-24.04; `CARGO_BUILD_JOBS=2`, `RUST_TEST_THREADS=1`; no timed benchmark |
| Test command | `cargo +1.100.0 test --release --locked --bin rr5_receipt_check --manifest-path research/cheap_ecdlp_cold_gate_20261123/Cargo.toml -- --test-threads=1 --nocapture` |
| Observed result | `19 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out`, parent test exit `0` |
| Source-tree and input checks | Recorded SHA256 replay reports **OK** on helper Cargo.toml, Rust checker, B8 JSON public fixture, original native library and lock, original `f4_fp_tower.rs` and `sig_fp_tower.rs`. Tracked Git status empty before and after |
| Test executable SHA256 | `784492368dbd25cd4fd10b0e53776b24686bee90a80bc60eb5bdae25ea503ec0` |
| Generated helper Cargo.lock SHA256 | `a6123e8bb420a0ae63745b7a9b285fea0e70930058dd6dd02fe1ccff5c4570f1` |
| Executable evidence | GitHub artifact #14850584102 contains `compile.jsonl`, `compile.stderr`, `executable.sha256`, `test.stdout`, `test.stderr`, `test.exit`, source/tree identities and resolved `Cargo.lock` |

**Interpretation:** source-exact compiler and Rust functional test gate now passes at two CI revisions after repairing a genuine inherited compiler defect. This is native **development** validation, **not an independent cryptographic certificate or signed PCC**; cannot infer the behavior of a blind Prime31 B677 relation stream not yet supplied. The helper-local Cargo lock is pinned by its archived bytes for each successful execution; it is **not yet committed**, so cross-run dependency-resolver reproducibility remains a separate follow-up. [pcc:unverified]

## Prior art, neighboring ownership and next gate

The algebraic target-separation witness `sum_i w_i [C_i|-b_i|a_i] = [0,...,0|1|d]`, checked by `[d]G=Q`, was already studied in existing Run69 target-rank controls. It is **not a novel ECDLP relation extractor**. Sources consulted: [Semaev 2004](https://eprint.iacr.org/2004/031); Petit–Kosters–Messeng [PKC 2016](https://doi.org/10.1007/978-3-662-49387-8_1); [lab Prime31 baseline report](README.md), and merged #4970 RR5 producer / #4974 B677 circuit, with existing negative Run72/75 and APS controls. Research-KB/Conductor native tools were unavailable; repository source review was the fallback. Neighboring graph, S3 and transfer source and frozen science evidence were not touched.

A real blind, original prime-field B677 public extraction stream **must first be delivered by its owning lane** with full statuses, RHS replay, independent rank and target recovery. Then reserve shared resources and measure **complete cold T=1** (including relation generation, matrix/replay, extraction and checker overhead) against best maintained index calculus, strongest admissible optimized negation-map rho and matched BSGS on the same target panel. Preserve negative/censored results and obtain independent PCC validation. No such benchmark ran in this PR.

## Parallel work account

**One coordinator, zero spawned native workers** (no native worker launcher). Parallel repository/CI reads supported the baseline, source and validation audit; only the coordinator wrote GitHub files. No paid EKS/Spot job, scientific timed experiment, extra automation, producer/graph/S3/transfer edit, benchmark reservation overrun or merge was initiated. Source-only CI work was within the workflow's own two compiler jobs and single-thread test cap. [pcc:unverified]
