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
| BSGS point-key collision regression | Run the same release library test binary with filter `bsgs_retains_all_indices_when_point_keys_collide` | 1 passed, 0 failed; all baby-step indices sharing a deliberately constant point key remained eligible for full-point verification. |
| Crypto full release library suite | `CARGO_TARGET_DIR=/private/tmp/codex-eccp131-build-crypto CARGO_BUILD_JOBS=2 cargo test --release --lib` | Initial run: 4,445 passed, 11 failed, 119 ignored in 1,351.83 s. Seven failures were missing frozen CryptoPro-B JSON files in the sparse checkout; four were unchanged `jv_cover` cases with 120-second per-run limits under host contention. The run is retained as a failed validation, not rewritten as a pass. |
| Sparse fixture reruns | Add tracked `research/cryptopro_b_chain_sweep_20261006/frozen_inputs/` to the sparse checkout; rerun `cargo test --release --lib cryptopro_b_chain_set -- --test-threads=1` and `cargo test --release --lib endo_generic_chain_set -- --test-threads=1` | 5 and 4 passed respectively, 0 failed. This covers all seven fixture-related failures. |
| Timed `jv_cover` reruns | Run each failed test by name with `cargo test --release --lib <test-name> -- --test-threads=1` | `nagao_finds_planted_six_sums_and_agrees_with_the_oracle`, `stopped_f4_agrees_with_the_full_solver_and_the_oracle`, and `replayed_f4_agrees_with_the_full_solver_and_the_oracle` passed. `hilbert_stop_agrees_with_the_full_solver_and_the_oracle` first failed with 28 qualifying samples against 30, then passed on the next isolated rerun. These tests are outside the implementation diff. |
| Crypto prime BSGS CLI | `ecdlp-nist solve ECCp-131 --secret 12345 --interval-bits 16 --method bsgs --json` | Exit 0; planted scalar 12345 returned, `verified: true`, table size 183. Raw [JSON](bsgs-eccp131.json) retained. |
| Crypto binary BSGS CLI | `ecdlp-nist solve ECC2K-130 --secret 12345 --interval-bits 16 --method bsgs --json` | Exit 0; planted scalar 12345 returned, `verified: true`, table size 183. Raw [JSON](bsgs-ecc2k130.json) retained. |
| Published prime target table-cap check | `ecdlp-nist solve ECCp-131 --challenge --interval-bits 64 --method bsgs --json` | Exit 1 as an intentional refusal; imported target coordinates match the published point, `failure: BSGS interval exceeds the 1048576-entry table limit`, `group_ops: 0`, `table_size: 0`. Raw [JSON](bsgs-eccp131-challenge-limit.json) retained. |
| Crypto IC CLI | `/private/tmp/codex-eccp131-build-crypto/release/ic inspect --curve eccp-131 --json` | Exit 0, `checks_passed`; published generator and target passed curve-equation and subgroup checks. General prime-field point counting is explicitly `not_checked`, and the prime/order screens are not primality proofs. Raw [JSON](ic-eccp131-inspect.json) retained. |

Initial cryptanalysis compiler attempts stopped on omitted sparse-checkout files (`include/`, both CUDA directories, `suite/fixtures/`, and `challenges/elliptic/`). The files were materialized before the successful release unit run; those setup failures provide no result about the ECCp-131 implementation.
The first crypto compile likewise stopped at omitted committed research and documentation paths. Those paths were materialized before the passing bounded BSGS release test. The frozen CryptoPro-B fixture omission in the initial full-suite run was repaired separately, as shown in the rerun rows.

The crypto release compiler took 96 minutes of wall time under concurrent host builds before the targeted test ran. The initial full-suite failures are separated by cause above. The `jv_cover` source and its time-limit settings are unchanged by this branch; the observed per-run timeout and the passing isolated reruns support a host-concurrency explanation, without proving that every future full-suite run will pass.

The retained ECC2K-130 status JSON has SHA-256 `0a2285955dbf9417c846c743603bd607e3925e8948a584a3f37c44b70f1fd967`; its SQLite state has SHA-256 `0a368c404d092a5099f397e23a89ddcd67e58e3ce0c2658c1266d48adf813e4e`. An earlier unarchived status smoke in a separate temporary directory returned the same stopped stage and is excluded from evidence and comparison. The retained command is a workflow check, not a relation-collection or speed experiment; its operation counters are uncalibrated for an attack-cost comparison.

The retained prime and binary BSGS CLI JSON files have SHA-256 `b1b0fb866204604ecbc18b06c74a9163faf25ff4cc998a22949c2d7b8c962ec4` and `836d5c373efcd75a595df21b4a4c721aad9689b0165f1ac447c8fdc3b46efb3a`, respectively. The published-prime table-cap JSON has SHA-256 `bae405a4fcec45ac14fd0659208569f8eca36e71e78daae4897486f343cc3007`. These correctness checks are not timing comparisons.
The crypto IC inspection JSON has SHA-256 `8d704ab6f3d8f53e90e086e6230f2e9fc4a6b1f1bd5eadc9ceb505ac1e4b3d80`.

## Graph and evidence scope

The existing `docs/index-calculus-scoreboard.html` and `docs/ic/progress-timeline.json` were checked for ECC2K-130 coverage. Neither receives a new result or measurement from this integration, so their canonical data and rendered copies remain current. The diagram distinguishes exact-parameter validation, bounded interval solving, the existing scaled or staged IC paths, and the missing prime-field path. Its arrows assert command flow only; they do not claim a transferred attack or a measured improvement.
