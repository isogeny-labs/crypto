# EXP-R31-M: Rust-only B31 natural-target factorized S4 oracle calibration

Predeclared 2026-11-15, after the separately observed successful factorized S4 Rust development CI and **before** this new natural-target execution. Standing [pcc:unverified]. This is a relation-oracle experiment, not complete ECDLP evidence.

## Prior lead and literature review

- Current source: `research/prime31_ic_20261115/native/examples/prime31_s3_quotient_gcd.rs`; factorized-S4 predeclared protocol `M4_FACTORIZED_S4_RESULTANT_PROTOCOL_20261115.md`; exact m4 coverage `M4_NATURAL_YIELD_ARITY_GATE_20261115.md`.
- Live [PR #4188](https://github.com/isogeny-labs/crypto/pull/4188) and independently scoped quotient/subresultant [#4228](https://github.com/isogeny-labs/crypto/pull/4228), dense-F4 [#4275](https://github.com/isogeny-labs/crypto/pull/4275), m8 full-cost protocol [#4077](https://github.com/isogeny-labs/crypto/pull/4077), and label-free selector [#4338](https://github.com/isogeny-labs/crypto/pull/4338).
- Published prime-field algebraic IC prior art: Petit–Kosters–Messeng, PKC 2016, DOI 10.1007/978-3-662-49387-8_1; Amadori–Pintore–Sala, *Finite Fields and Their Applications* 51 (2018), DOI 10.1016/j.ffa.2018.01.009 (single factor-base relation technique). No priority or algorithmic novelty is claimed from classical resultants.
- This connector runtime cannot invoke the repository research-KB service/CLI, conductor or authenticated EKS; direct GitHub primary files and publication records are the bounded fallback. No new Python/C++.

## Evidence that motivated this protocol: **finished** prior development CI, not a future prediction

[GitHub Actions 43764153465](https://github.com/isogeny-labs/crypto/actions/runs/43764153465), Rust job 132965380902, **passed**, as inspected 2026-11-15. Ubuntu 24.04, pinned Rust 1.100.0, release, RAYON_NUM_THREADS=1. It passed 40/40 focused reused-library tests and 3/3 S3/m4 example tests including exact monic gcd equality on B4/B8/B16 and exceptional U=r branch. Matching Rust process results:

| B | Precomputation K | Planted target old quotient norm | Factorized short-resultant norm | Ratio (stage only) |
| -: | -: | -: | -: | -: |
| 4 | 0.008232 ms | 0.076954 ms | 0.026349 ms | 2.92x |
| 8 | 0.042663 ms | 3.179931 ms | 0.138345 ms | 22.99x |
| 16 | 0.200718 ms | 166.965025 ms | 0.917729 ms | 181.93x |
| 31 | 1.997539 ms | *not run/censored* | 10.122916 ms | *not comparable* |

At B16 independent public [17]G, 166.658650/0.905791 ms = 183.99x. B31 fast-only returned a signed planted four-point witness and no row for public [17]G. These are **single-run development stage measurements**; no cold ECDLP, no replayed independent PCC. Earlier branch notes saying CI was queued are now superseded for this specific run only.

## New Rust-only falsifiable experiment

Run on the same registered ordinary prime-order curve: `p=2^31-1, a=847514846, b=2087444724, n=2147446831, G=(625037715,2100627251)`.

1. Construct `Base::build(PRIME31,6)` and `K` **once**. Use `B=31`. Sample **4,096** distinct nonzero public fixture scalars using deterministic `splitmix64` with frozen seed `0x525553545f4d3401`; form Q=[scalar]G using existing Rust `Arithmetic::scalar`. The scalar is retained ONLY for reproducibility/after-the-fact replay and **must not** be supplied to elimination. Separate all target-generation group work from algebraic timings.
2. For each public Q use only its abscissa and the frozen `xs,K` in `m4_first_eliminant_factorized`. Charge every query, including negative gcds, and record completion/censoring/exceptional counts, gcd degree and target-dependent elapsed nanos. A nonzero eliminant is **not** a verified relation.
3. For every nonconstant gcd, use existing capped algebraic root extraction; afterward use group arithmetic solely for signed-witness verification against the **full** public point Q. Record roots, 4-index tuples, deduplicated signed rows, verification cost, and modular coefficient-vector rank. Any capped/failed extraction is **undetermined**, not a zero-relation negative.
4. Report a deterministic sample digest/fingerprint, count of distinct Q encodings, arithmetic costs, and split precompute, query, extraction and verifier intervals; include all positive and exceptional outcomes. A known planted four-summand fixture remains in the older unit test, not the natural population. **Do not select targets after seeing results.**
5. Expected successes under uniform target model: exact B31 m4 support 618,885/2,147,446,831 = 0.0002881957266955 per point; expected 1.1804 supports among 4096 (not a requirement or a statistical confidence claim). Any observed relation-yield estimate must state the tiny sample-success count/uncertainty.

## Explicit stop/acceptance gates

The new experiment succeeds as an **oracle-calibration experiment** only after exact-head Rust CI exits zero and all 4096 algebraic calls have definite complete/censored status. Any false signed witness or unreported undetermined result invalidates correctness/cost conclusions. A B31 m4 hit-rate measurement is not a cheap or complete ECDLP solver. A whole-solver claim still requires natural useful rank, complete logs of at least 12 held-out arbitrary public Q, all cold costs and same-host optimized rho/BSGS, and an independent replay/PCC. Only prioritize m8 target-dependent extraction if this m4 calibration closes without hidden errors.
