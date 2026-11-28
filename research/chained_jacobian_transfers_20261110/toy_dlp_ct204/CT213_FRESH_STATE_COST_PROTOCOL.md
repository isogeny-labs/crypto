# CT213 — preregistered native paired full-arm toy cost probe (2026-11-18)

Status: **committed source; native timing and independent PCC not yet obtained**. [pcc:unverified]

## Purpose and prior evidence

CT212 [exact-head CI](https://github.com/isogeny-labs/crypto/actions/runs/44351488002) actually recovered all 43 toy logarithms on both E and Jac(C) with generic BSGS, but its 0.02s two-test duration was **not** a complete cold comparison. CT213 closes that specific *toy-only* measurement gap with Rust source `tests/prime_cover_toy_cold_cost.rs`. It does not substitute for authenticated original-model 695/701-record physical C169/C13a/C13b/N2_actual/D pricing, complete destination index calculus, or optimized rho/BSGS/IC panels. [pcc:unverified]

Internal sources reviewed: crypto PRs [#3877](https://github.com/isogeny-labs/crypto/pull/3877) (merged), [#4215](https://github.com/isogeny-labs/crypto/pull/4215) (frozen 48-counter transformation, **prediction not implementation**), [#4697](https://github.com/isogeny-labs/crypto/pull/4697) (draft arithmetic), [#4715](https://github.com/isogeny-labs/crypto/pull/4715) (native recovery); cryptanalysis [#1523](https://github.com/isogeny-labs/cryptanalysis/pull/1523) (merged frozen scope), [#1766](https://github.com/isogeny-labs/cryptanalysis/pull/1766) (independent full-cost gate), [#1770](https://github.com/isogeny-labs/cryptanalysis/pull/1770) (arithmetic mirror); [Gaudry, EUROCRYPT 2000](https://doi.org/10.1007/3-540-45539-6_2), [GHS/Galbraith-Hess-Smart, EUROCRYPT 2002](https://doi.org/10.1007/3-540-46035-7_3). The prior literature addresses Jacobian IC / Weil descent, not a claim that this equal-base-field toy BSGS implementation is faster. The research-KB MCP is not loaded here and its repository wrapper is not executable in this connector-only runtime, so KB review is **unavailable**, not an empty novelty search.

## Frozen before observation: exact protocol

- Curve and cover: `E/F31: y^2=x^3+3`, `G=(1,2)`, order 43; `C: v^2=u^6+3`.
- Public target roster: each of the **43 distinct** points `Q=[k]G` for `k=0..42`, generated **outside** each timed arm. The solver receives only `G,Q,43`, never fixture `k`; test assertions use `k` after the solve. This is a known-answer toy, not cryptographically blind targets.
- Exactly seven rounds, one complete 43-point roster per round (301 paired samples); cyclic 17-step roster offset and alternating AB/BA ordering. Both arms solve the **same Q** in each pair; there is no between-target solver table reuse. Repeat-target rounds are explicitly allowed *only* for a toy microbenchmark, not for independent cold ECDLP proof.
- Direct arm times complete BSGS table creation, search, and `[answer]G=Q` verification using the provided source curve. Jacobian arm times **fresh** genus-two cover construction, pullback of `G,Q`, order assertion, fresh BSGS table/search, image verification, explicit norm and source re-verification.
- The identical generic BSGS routine is used in both groups and reports separately the additions and baby-table entries. It prices neither addition equivalently nor any assumed component gain. The test fails for wrong answers, non-distinct source targets, or different source/Jacobian BSGS addition counts.
- `std::time::Instant` records each full-arm elapsed duration in nanoseconds on the same hosted runner. The test emits a `CT213_TOY_COST_JSON` line with total nanoseconds, median per-arm nanoseconds, operation counts and the *diagnostic* direct/Jacobian ratio. It never asserts that the ratio is above 1. Only Rust test execution is timed; source/CI compilation is not counted.
- Common public-curve fixture construction and public target generation are not timed. Jacobian cover construction **is** charged in the transferred arm. The result is an asymmetrical (but documented) fixed-curve toy comparison, **not** a complete optimized cryptographic baseline.
- Memory is **unmeasured**. The GitHub shared hosted runner is not an isolated/affinity-pinned benchmark host, has no calibration A/A or confidence interval, does not run optimized rho or prime-field IC, and has no independently signed PCC. Hence this run **cannot** satisfy the owner complete-solve advantage gate, whatever the numerical ratio.

## Execution

Bounded existing workflow: `.github/workflows/ct205-toy-transfer.yml` now includes the new source in sparse checkout and executes its release test after the successful CT212 integration test.

```sh
cargo test --locked --release --test prime_cover_toy_complete_dlp -- --nocapture
cargo test --locked --release --test prime_cover_toy_cold_cost -- --nocapture
```

**Current state:** Source and CI wiring committed; Rust is not installed in this runtime, so this report records **no claimed timing, no new native pass, and no measured negative/positive ratio** until exact-head CI logs can be inspected. This is a fresh-state toy *comparison design* and executable implementation, not original-model custody.

## Admission and next decisive test

1. Inspect exact-head workflow logs, compilation, full 301-pair sample roster, 670×7 addition parity and JSON. Retain any failure, even if only a compile error, as negative engineering evidence.
2. If Jac(C) is slower on the toy, retain measured negative evidence as toy-only; avoid further polishing BSGS on this no-descent genus-two cover and concentrate on existing genuine even-degree Jacobian IC / other field-reduction leads.
3. Pursue **authorized** original S3 materialization, immutable 695/701 native replay and physically priced C169 vs C13a+C13b with real N2_actual and D, independently replaying all records. Reuse frozen counter/preregistration evidence rather than creating another branch.
4. Only admit a speedup with blind fresh public targets, full *single-target* setup/transfer/relation collection/LA/recovery/verification, resident and peak memory, paired same-host strongest rho/BSGS/IC baselines, uncertainty with lower confidence bound ratio >1, and signed PCC.

No EKS/Spot or paid compute was launched, no protected merge attempted. [pcc:unverified]
