# Certicom 130-bit challenge workflow integration

**Recorded:** 2026-10-20; validation updated 2026-10-21. **Scope:** implementation and correctness checks, without a performance comparison or scientific state transition.

## Question and exact inputs

Expose the published Certicom [ECCp-131 and ECC2K-130 challenge groups](https://www.certicom.com/en/curves-list) to the ECDLP and index-calculus command paths. ECCp-131 is the prime-field curve identified by the full `p`, `a`, `b`, `G`, `n`, cofactor and published `Q` in the [curve constructor](../../src/ecc/curve.rs) and [IC parameters](../../src/bin/ic/params.rs). ECC2K-130 is the distinct binary Koblitz group already carried by the [binary curve implementation](../../src/binary_ecc/curve.rs) and the fixed-parameter file in `aburan28/cryptanalysis` (`suite/docs/ic/params/ecc2k130-fixed.json`). A matching bit count does not identify a curve.

## Implemented paths and limits

The [workflow diagram](workflow.svg) records the execution boundary; its editable source is [workflow.dot](workflow.dot).
Green marks exact-parameter checks or a bounded solver, yellow marks the existing scaled or staged IC workflow, and the red box marks the unvalidated exact prime IC path.

![Exact identities and command paths, with the open prime IC path marked separately](workflow.svg)

| Path | Scope in this change |
| --- | --- |
| `ecdlp-nist solve ECCp-131 --challenge` | Imports the published prime-field point, with the solver's subgroup and result verification. |
| `ecdlp-nist solve ... --method bsgs --interval-bits N` | Uses exact ECCp-131 or ECC2K-130 group operations for a bounded interval; the in-memory table holds at most 1,048,576 baby-step entries. Every 64-bit point-key collision retains its candidate indices. |
| `ic inspect --curve eccp-131` and `icx inspect eccp-131` | Inspect the exact prime-field parameters and target. The `cryptanalysis` `ca-ic` inspector performs curve-equation and `[n]P = O` checks on both published points. |
| `crax ecdlp --curve eccp-131 --q X,Y --analyze-only` | Accepts the exact prime group and supplied point for structural weak-curve analysis. |
| `ca-ic prime --curve eccp-131` | Reports exact-curve structure, then runs its existing prime IC experiment on a generated smaller curve. |
| `ca-ic fixed --params docs/ic/params/ecc2k130-fixed.json` | Uses the existing staged ECC2K-130 fixed-parameter IC engine. This integration changes no relation, rank, or descent algorithm in that engine. |

The exact ECCp-131 relation-to-scalar path is an open [implementation problem](../../problems/eccp131-full-parameter-index-calculus.md). A bounded BSGS check needs a justified interval; the full 131-bit subgroup exceeds the table cap. This change makes no attack-speed, asymptotic, or full-parameter IC cost claim.

## Validation record

The source revisions are `crypto` `07dd1d97fa` (implementation commit) and `cryptanalysis` `3ac3f59363` (PR [#1011](https://github.com/aburan28/cryptanalysis/pull/1011)). The local validation host is Darwin arm64 with Rust/Cargo 1.93.1. The test points are generated from known scalars; they are separate from the published challenge point. The published point is checked for curve and subgroup membership without assuming its logarithm.

| Check | Exact command or action | Observed status |
| --- | --- | --- |
| Cryptanalysis release unit checks | From `cryptanalysis/suite`: `CARGO_TARGET_DIR=/private/tmp/codex-eccp131-build-cryptanalysis CARGO_BUILD_JOBS=2 cargo test --release --lib eccp131 -- --nocapture` | 2 passed, 0 failed; 3,102 other tests filtered. The sole compiler warning was in untouched `f4_fp_tower.rs`. |
| Cryptanalysis `ca-ic` | From `cryptanalysis/suite`: `CARGO_TARGET_DIR=/private/tmp/codex-eccp131-build-cryptanalysis CARGO_BUILD_JOBS=2 cargo run --release --bin ca-ic -- inspect --curve eccp-131 --json` | Exit 0, `checks_passed`; both published points passed curve-equation and subgroup checks. General prime-field point counting remained `not_checked`. |
| Cryptanalysis `ca-icx` | `ca-icx inspect eccp-131 --json`; `ca-icx estimate eccp-131 --json` | Exit 0; inspection `checks_passed`; estimate reports `regime: scaled`. |
| Cryptanalysis `crax` | `crax ecdlp --curve eccp-131 --q 0x03AA6F004FC62E2DA1ED0BFB62C3FFB568,0x009C21C284BA8A445BB2701BF55E3A67ED --analyze-only` | Exit 0; structural analysis accepted the published point. |
| ECC2K-130 fixed status | `ca-ic fixed --params docs/ic/params/ecc2k130-fixed.json --dir /private/tmp/codex-ecc2k130-status-recorded-20261020 --stage status --json` | Exit 0, `status: stopped`, zero saved relations, no logarithm database. Raw [JSON](ecc2k130-status.json), [SQLite state](ecc2k130-status.sqlite3), and [stderr](ecc2k130-status.stderr) retained. |
| Crypto bounded BSGS unit check | From `crypto`: `CARGO_TARGET_DIR=/private/tmp/codex-eccp131-build-crypto CARGO_BUILD_JOBS=2 cargo test --release --lib certicom_131_prime_and_130_binary_bsgs_use_exact_groups -- --nocapture` | 1 passed, 0 failed; 4,574 filtered. The test verifies planted scalars on both exact groups, the published ECCp-131 point's subgroup relation, and refusal of an oversized BSGS interval. |
| Crypto full release library suite | `CARGO_TARGET_DIR=/private/tmp/codex-eccp131-build-crypto CARGO_BUILD_JOBS=2 cargo test --release --lib` | Initial run: 4,445 passed, 11 failed, 119 ignored in 1,351.83 s. Seven failures were missing frozen CryptoPro-B JSON files in the sparse checkout; four were unchanged `jv_cover` cases with 120-second per-run limits under host contention. The run is retained as a failed validation, not rewritten as a pass. |
| Sparse fixture reruns | Add tracked `research/cryptopro_b_chain_sweep_20261006/frozen_inputs/` to the sparse checkout; rerun `cargo test --release --lib cryptopro_b_chain_set -- --test-threads=1` and `cargo test --release --lib endo_generic_chain_set -- --test-threads=1` | 5 and 4 passed respectively, 0 failed. This covers all seven fixture-related failures. |
| Timed `jv_cover` reruns | Run each failed test by name with `cargo test --release --lib <test-name> -- --test-threads=1` | `nagao_finds_planted_six_sums_and_agrees_with_the_oracle`, `stopped_f4_agrees_with_the_full_solver_and_the_oracle`, and `replayed_f4_agrees_with_the_full_solver_and_the_oracle` passed. `hilbert_stop_agrees_with_the_full_solver_and_the_oracle` first failed with 28 qualifying samples against 30, then passed on the next isolated rerun. These tests are outside the implementation diff. |
| Crypto CLI checks | Release builds of `ecdlp-nist` and `ic`, followed by exact-curve command checks | Build in progress; command results pending. |

Initial cryptanalysis compiler attempts stopped on omitted sparse-checkout files (`include/`, both CUDA directories, `suite/fixtures/`, and `challenges/elliptic/`). The files were materialized before the successful release unit run; those setup failures provide no result about the ECCp-131 implementation.
The first crypto compile likewise stopped at omitted committed research and documentation paths. Those paths were materialized before the passing bounded BSGS release test. The frozen CryptoPro-B fixture omission in the initial full-suite run was repaired separately, as shown in the rerun rows.

The crypto release compiler took 96 minutes of wall time under concurrent host builds before the targeted test ran. The initial full-suite failures are separated by cause above. The `jv_cover` source and its time-limit settings are unchanged by this branch; the observed per-run timeout and the passing isolated reruns support a host-concurrency explanation, without proving that every future full-suite run will pass.

The retained ECC2K-130 status JSON has SHA-256 `0a2285955dbf9417c846c743603bd607e3925e8948a584a3f37c44b70f1fd967`; its SQLite state has SHA-256 `0a368c404d092a5099f397e23a89ddcd67e58e3ce0c2658c1266d48adf813e4e`. An earlier unarchived status smoke in a separate temporary directory returned the same stopped stage and is excluded from evidence and comparison. The retained command is a workflow check, not a relation-collection or speed experiment; its operation counters are uncalibrated for an attack-cost comparison.

## Graph and evidence scope

The existing `docs/index-calculus-scoreboard.html` and `docs/ic/progress-timeline.json` were checked for ECC2K-130 coverage. Neither receives a new result or measurement from this integration, so their canonical data and rendered copies remain current. The diagram distinguishes exact-parameter validation, bounded interval solving, the existing scaled or staged IC paths, and the missing prime-field path. Its arrows assert command flow only; they do not claim a transferred attack or a measured improvement.
