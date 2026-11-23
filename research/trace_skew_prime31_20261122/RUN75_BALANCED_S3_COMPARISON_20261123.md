# Run75 — paired balanced-rank-four versus Newton S3 extraction and m=4 exponent gate

**Date:** 2026-11-22 America/Los_Angeles / 2026-11-23 UTC.
**Repository:** `isogeny-labs/crypto`, existing draft #4924.
**Goal:** measure a genuine non-table **public-abscissa** algebraic elimination component on the unmodified prime-field curve, then promote it only if the full index-calculus qualification becomes possible.

## Verified pre-ablation evidence

Read actual completed original-Rust CI [#45089472283](https://github.com/isogeny-labs/crypto/actions/runs/45089472283), job 137415511884, success. Its eight target controls each at B=64,128,256,677 were compared **after the candidate returned** with exhaustive independent S3 pair checks. All 32 x-only cells passed; three positive single-pair cases and five exact negatives per B. There is **no** unknown-index signed-target DLP recovery in that run. Source-specific `online_ms` includes current Newton construction, D/N, GCD, rational-root extraction, validation, but *not* the post-hoc exhaustive reference enumeration.

| Factor base B | 8-cell median Newton extraction, ms | 8-cell mean, ms | `D` / `N` degree | Control |
|---:|---:|---:|---|---|
| 64 | 0.3315 | 0.3376 | 127 / 128 | all 8 exact |
| 128 | 1.0625 | 1.0666 | 255 / 256 | all 8 exact |
| 256 | 4.2545 | 4.2608 | 511 / 512 | all 8 exact |
| 677 | **29.3405** | **29.8543** | **1353 / 1354** | all 8 exact |

At B677, the independent enumerative **post-check** took around 4.83–5.49 ms per cell, versus 28.93–31.99 ms for algebraic online extraction. The scan is a **generic reference, disallowed as a qualifying non-generic solver**, and its lower stage time is a serious engineering warning. The previously checked rank-four balanced coefficient builder took roughly 11.2 ms at B677 but was not yet used by the actual candidate extractor; this may or may not overcome the GCD/root stage.

## Implemented new original-Rust experiment

Modified `examples/prime31_monic_s3_pair_oracle.rs` (original crate) with:
- `construct_tree_full`: take the existing independent balanced tree's H0,H1 directly, then perform the **same** eliminant D, norm N, gcd C and exact algebraic root extraction as the Newton arm.
- `extract_with_method`: a common post-elimination pipeline; **neither** arm receives pair indices, secret scalars or a precomputed pair-sum dictionary.
- `--compare`: every one of the original B64/128/256/677 × eight public-z cells; five timed repeats per arm, alternating order; report paired per-cell **median**, sum of per-cell medians and exact equality with an independently generated post-hoc S3 reference. The group-oriented seed generator has been preserved. All trials assert equality, including each size's 3 positive + 5 negative cases.
- Added a dedicated original-Rust CI step to `.github/workflows/prime31-trace-symmetric-rust-gate.yml` with a five-minute cap. This is an **x-only extraction ablation**. The matrix stage, signed-lift/group verification, true RHS and complete ECDLP are not implicitly covered.

Run:
```bash
cargo test --locked --release --example prime31_monic_s3_pair_oracle
cargo run --locked --release --example prime31_monic_s3_pair_oracle -- --scale
cargo run --locked --release --example prime31_monic_s3_pair_oracle -- --compare
```

**New Run75 execution status:** source committed; native `--compare` logs and CI verdict are still to be inspected. Do **not** use old #450894 timing as a measurement of the newly added tree-extractor arm.

## New analytical screening criterion for the *four-summand* candidate

A useful asymptotic hypothesis test can be written without pretending the binary pair oracle solves four-summand ECDLP.

Let group order `n ≈ p`, factor-base size `B`, four-leaf relation hit probability `Theta(B^4/n)` *only under a uniform-output and unsaturated-image heuristic*, `Theta(B)` **independent** relations needed, full relation-oracle time `T(B)=Theta(B^gamma)`, and sparse linear algebra cost `Theta(B^2)`. Ignore polylog factors, constants, failures and preprocessing for this conditional screening model.

The collection and LA model is:

```
C(B) = Theta((n / B^3) T(B) + B^2)
     = Theta(n B^(gamma-3) + B^2).
```

Balancing for `gamma < 3` gives `B = n^(1/(5-gamma))`, exponent `2/(5-gamma)`. **A sub-generic (< n^1/2) exponent requires gamma < 1**, *and* preprocessing / target recovery must not dominate and relation-rank heuristics must hold. A per-probe `~B` oracle only meets the 1/2 exponent under this model; an `~B^2` oracle is worse. If ordinary algorithmic m=4 root extraction cannot achieve **amortized o(B)** work (with setup accounted for), the proposed uniform four-summand framework cannot demonstrate the desired exponent gain on these hypotheses. This is a **conditional cost-model filter**, not an unconditional ECDLP lower bound or a security claim. Prove or falsify its individual assumptions before using it as an accepted complexity estimate.

Separate *m=2* warning: at B677 and group order 2147446831, a uniform random group target has at most `4*C(678,2)/n = 0.00042749` signed-pair support under a loose cardinality upper bound, so approximately **2339** uniform probes per potential success even optimistically. At the observed 29.34 ms x-only median, that is about 69 seconds of candidate work per putative relation; 677 independent hits would cost about **12.9 hours** in this simplistic, uniform, unchanged-cost extrapolation. This is *not* a measured 12.9-hour run, a bound on biased searches or an accepted full-ECDLP estimate.

## Prior-art and adjacent lanes audited (no duplicate solver claim)

- The original Semaev prime-field summation-polynomial method, [ePrint 2004/031](https://eprint.iacr.org/2004/031), and the 2014 Faugère–Huot–Joux–Renault–Vitse symmetrization: symmetric S3 itself is not novel.
- Existing **November 15 non-table S3 quotient/GCD in Rust**: `research/prime31_ic_20261115/native/examples/prime31_s3_quotient_gcd.rs`, [CI #43741710811](https://github.com/isogeny-labs/crypto/actions/runs/43741710811); it *already* supports signed target lifting, and the current experiment must compare directly before any priority claim.
- September 21 radical image DRL `research/radical_pair_image_20260921/RESULTS.md` includes independent small-field solver-stage/cold-batch benefits but no ECDLP-complete benefit.
- #4901 complete prime31 genuine-RHS/target quotient reference uses a **generic 916,469-key pair dictionary**; Run72 cold total remains **4.83–5.40x** same-host shared BSGS, outside the allowed non-generic qualification.
- #4920 signed chained S3, #4922 reusable elimination, #4692/#4812/#4822 RR9/Horner/F3 public-only larger-arity root lanes. Avoid stealing their ownership.
- `isogeny-labs/cryptanalysis/crates/prime-field-m3-cdclt/README.md`: m3 compact-selector screen claimed online exponent ~**1.33**, missing its **< 0.5** gate; representation reduction alone failed to move the algorithmic exponent.
- `isogeny-labs/crypto-autoresearcher/notes/g31-004-wp2-evalcost-20261120/RESULTS.md`: mode-sensitive S3/S4 evaluation charges span orders of magnitude; unpinned Python/M4 benchmarks must **not** be compared as Rust Prime31 server timings.

**Knowledge-base receipt:** The repository requests `kb_status` and `search_knowledge` before new mechanisms, but neither the research-KB MCP nor the checked-out CLI wrapper was available in this run. This was a bounded GitHub source/literature review, not an exhaustive lead atlas review.

## Prioritized experiment queue

1. Run75 exact-source native Rust `--compare`; if any mismatched positive/negative pair, stop, fix and rerun. Record median, min/max, full job URL, commit SHA. **No successful native run is claimed here.**
2. Reuse the November 15 original signed quotient/GCD implementation as an actual **matched third arm** at identical W and public Q, including `z=x_i` collision, diagonal, infinity and signed-target inversion branches. Require independent `P1+P2=±Q` certificates, not x-only S3.
3. Advance larger-arity non-table extraction and G+Q rank integration in existing #4692/#4812/#4822/#4901 lanes; account for all unsuccessful probes, dense/structured reductions, setup and LA. Use 46 planted positives/18 known negatives **then** uniform held-out public target cohorts. Require no secret-scalar inputs except independent checking after completion.
4. Freeze 1/32/128/1024-target **complete cold** panels on the unchanged 2³¹−1 ordinary curve and at least two other ordinary cofactor-one curves, against the strongest measured **index-calculus frontier**, optimized same-host negation rho and BSGS. Any performance claim requires independent source replay, wall, field/group ops, peak RSS, rank and verified final logs.

**Qualification status:** source check-in completed; native Run75 ablation pending; a cheaper **complete** qualifying solver and improved asymptotic exponent have not been established.
