# Prime31 run63 — original Rust affine-coordinate query floor

**Date:** 2026-11-22 UTC. **Evidence:** source-specific Rust 1.100.0 GitHub Actions [#45026992197](https://github.com/isogeny-labs/crypto/actions/runs/45026992197), original source/workflow commit `75169e5c14c522178650d2d501db9254e1f069f2`. First execution job `137222066807` **SUCCESS**. All 40 original-Rust-reference initial scalar chains × 192 group increments, 7,680 steps, passed for both X and XY normalization, including O / G / -G / order wrap controls. Five 250000-step original-scalar crosschecked endpoints passed per arm. Signed PCC: unverified.

## Question tested

Run62's complete fixed-G mixed Jacobian cost (21.56–21.87 ns in *other* source-specific jobs) excluded the mandatory affine-coordinate extraction for an implementation that needs the public abscissa of **every** Q+kG query. Run63 reuses the original `prime31_algebraic_ic::Arithmetic` as the correctness reference and compares **same-host, same-process** complete projective +G against batched x-only and x/y materialization using Montgomery's one-inversion-per-256-points trick. These are **component measurements**, not an ECDLP solver. No root selection, relation rank, factor-base setup, lookup, or target logs occur.

Exact ordinary curve `p=2147483647, a=847514846, b=2087444724, n=2147446831, G=(625037715,2100627251)`, cofactor 1. Group law includes the previously missing exceptional branches.

## First source-specific native release CI result

All times ns per public Q+kG increment, **250000 consecutive increments per repetition, five rotated arm repetitions**, endpoint crosschecked against original Rust scalar multiplication.

| Arm | Five raw ns/probe | Median ns/probe |
|---|---|---:|
| Complete projective shift; coordinates not affine | 16.6655, 16.4228, 16.4223, 16.2460, 16.4051 | **16.4223** |
| Complete shift + batch-256 **every affine x** | 21.8773, 21.8684, 21.9820, 21.7745, 21.5624 | **21.8684** |
| Complete shift + batch-256 every affine (x,y) | 23.4431, 23.1132, 23.0823, 22.7670, 23.0222 | **23.0823** |

The measured paired overheads are +5.4461ns for x-only normalization and +1.2139ns for materializing y after x, on this single runner. The x-only normalized chain is 1.3316× the projective-only chain. The actual group-shift primitive is faster on this run than in run62; the run62 values must **not** be pooled as if same-host.

**Sensitivity only:** Run59's *synthetic* hybrid quotient-rank model predicted 4,699,949 uniform queries for a 1024-target cohort. Using the historical **different-host** #4532 170.113ms shared-BSGS result, any affine-x-per-query design has a nominal 36.19465ns total budget per query. The run63 x-only component at 21.8684ns projects 102.7803ms, leaving 67.3327ms / **14.3262ns per query** for **all lookup / polynomial relation extraction, failed probes, setup, rank and target work combined**. XY materialization would project 108.4856ms and leave 61.6274ms. These are intentionally **cross-run sensitivity calculations**, not a measured winner or theorem lower bound.

**Unexpected positive engineering result:** Projective + x-only batch inversion still fits the primitive-only 36.2ns sensitivity on this GitHub runner; the previous concern that affine materialization would unavoidably consume the whole budget is falsified at this component scope. This does **not** imply that a public-only non-generic relation extractor fits in the leftover 14.3ns.

## Prior work and red lines

- #4391 already implements complete Jacobian and batch inversion in original APS Rust; this ablation does not claim to invent it.
- #4901 runs55–60: 677-point Dickson/y^7 hybrid and exact 916469 two-sum support, 88 free relations, quotient-rank partial target stopping; all modeled query counts assume an idealized relation oracle.
- #4692, #4812 and #4822 own public-only algebraic blinded S3/RR9/F3 extraction; #4891 reports multi-second low-B F4 costs. Neither this fast shift nor a hash of precomputed two sums qualifies as a novel non-generic relation extractor.
- Amadori–Pintore–Sala, *On the discrete logarithm problem for prime-field elliptic curves*, Finite Fields and Their Applications 51 (2018) 168–182, DOI https://doi.org/10.1016/j.ffa.2018.01.009, establishes one-relation prime-field prior art. Repository KB MCP unavailable in this runtime; reviewed prior PRs and paper metadata.
- The per-probe floor applies only to **this** fixed-step affine-coordinate implementation. Homogeneous Semaev evaluation in projective coordinates or entirely different algorithms need not materialize x every step.

## Next experiments, ranked by decisive value

1. **P0 blind algebraic S3/RR9 extraction:** run the existing original-Rust #4822 four-variable RR9/Krylov and #4812 14-variable Horner/F3 on frozen held-out natural Q and negatives under equal wall/RSS/F4-degree caps. Verify complete original-ideal root recovery and signed group-law relation; count timeouts as censored, not UNSAT.
2. **P1 strict leftover-cost gate:** on same runner as x-only normalization, time *actual* non-generic per-query S3 test/root extraction including negative attempts and independent rows, not precomputed group pair-lookup. If measured non-generic query stage + rest exceeds 14.3ns/query under the run59 rate model, demote this particular 1024-target hybrid/two-point proposal against the historical BSGS sensitivity.
3. **P2 complete end-to-end:** recover independent blind logs, original-rust [k]G=Q, 1/32/128/1024 cohorts. Charge setup, F4 misses, memory, rank and verification. Compare paired same-host with strongest optimized shared BSGS, rho and existing index-calculus frontier. Change conclusions only when all costs and CI/replay are measured.

**Classification:** independently original-Rust-checked, exact-source Rust microbenchmark with a measured lower-level x-coordinate materialization improvement. Not a new non-generic index-calculus solver and not a complete ECDLP speedup.
