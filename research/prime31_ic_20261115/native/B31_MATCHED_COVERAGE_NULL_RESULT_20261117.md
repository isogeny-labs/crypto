# Prime31 B31 matched 256-target signed eight-leaf coverage: actual-curve null control

**Measured 2026-11-17 UTC; source-specific Rust evidence [pcc:unverified].** PR [#4626](https://github.com/isogeny-labs/crypto/pull/4626), stacked on completed generic-equivalent 1,024-log baseline [#4623](https://github.com/isogeny-labs/crypto/pull/4623). This is a research diagnostic, not a new non-generic ECDLP solver.

## Decisive observation

On the actual ordinary cofactor-one prime-field curve over **p = 2³¹ − 1 = 2,147,483,647**, group order **n = 2,147,446,831**, with `E: y² = x³ + 847514846 x + 2087444724` and `G=(625037715,2100627251)`, the original depth-six **Dickson31 did not outperform ordinary B31 factor bases** in the preregistered same-256-public-target first-shift-zero signed eight-leaf membership comparison.

| Fixed 31-point base | Exact hits / 256 | Exhaustive misses | Distinct 4A sums | 4A duplicates of formal net signatures | 4A setup | All 256 queries |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| Actual Dickson depth six | 237 | 19 | 618,885 | 0 | 296.246 ms | 2,334.013 ms |
| First 31 rational distinct x | **244** | 12 | 618,885 | 0 | 298.370 ms | 1,775.268 ms |
| Seeded random x-distinct A | 237 | 19 | 618,765 | 120 | 284.483 ms | 1,913.497 ms |
| Seeded random x-distinct B | 237 | 19 | 618,765 | 120 | 295.059 ms | 2,089.998 ms |

The first-x base exceeded Dickson by seven successful residuals on this one predetermined cohort (+2.73 percentage points), while both seeded random bases matched Dickson's 237/256 hit count exactly. With 31 paired disagreements against first-x (Dickson-only 12; first-x-only 19), this one-cohort contrast is not evidence of a robust improvement. Neither random seeded 4A collision deficit (120 collisions of formal signatures) yielded an advantage in measured 8A membership coverage. **These are not independent cohorts or controlled isolated runtime speed comparisons.**

Pairwise public Q contingency counts against Dickson:

| Comparator | Both hit | Dickson only | Comparator only | Neither |
| --- | ---: | ---: | ---: | ---: |
| First rational x31 | 225 | 12 | 19 | 0 |
| Random A | 222 | 15 | 15 | 4 |
| Random B | 219 | 18 | 18 | 1 |

The generic `4A+4A` oracle recovers a signed eight-point relation by constructing group sums. It must not be presented as sign-free index calculus, an asymptotic complexity breakthrough, or a cheaper full ECDLP solve. Its sequential full-query timings include exhaustive failures and witness replay but neither cold total target-log recovery nor a matched external speed frontier.

## Mathematical interpretation: exact versus heuristic

The 31 labelled signed leaves define a formal integer net vector `v∈Z^31`, with `||v||_1≤8` and even parity; opposite-sign pairs fill the unused slots. The **exact combinatorial count** of distinct formal net vectors is `N_8=5,654,193,417`. The four-leaf count is `N_4=618,885`. The original Rust signed-net combinatorial implementation and a separately computed binomial coefficient sum agree.

Under a **hypothetical** ensemble of 31 independent uniformly distributed group scalars `z_i∈F_n`, for any distinct formal vectors `v,w`, `Pr(v·z=w·z)=1/n`, since `v−w` is a nonzero vector modulo prime `n` (each coordinate has absolute value ≤16<n). Thus the exact expected collision energy for this ensemble is

`E[Σ_t c_t²] = N_8 + N_8(N_8−1)/n`,

and Cauchy–Schwarz plus Jensen implies `E[|8A|]/n ≥ N_8/(n+N_8−1) = 0.724744...`. **This bound is for the stated independent-scalar ensemble, not the fixed Dickson base and not the distinctly sampled-x random control.**

As a **heuristic only**, pretending all `N_8` correlated group sums were independent uniform bins would give
`1−exp(−N_8/n)=0.9281363083`, predicting `237.60/256` and `950.41/1024` positives. Observed Dickson coverage was `237/256` in this new source-specific experiment and `951/1024` in the frozen #4623 earlier cohort. The close match is a diagnostic warning **against attributing the measured coverage alone to hidden algebraic structure**; it is not a probabilistic theorem, confidence interval, p-value, proof of uniformity, or a characterization of the factor-base image.

## Original Rust implementation, controls and reproducibility

- [Preregistered protocol](B31_MATCHED_COVERAGE_NULL_PROTOCOL_20261117.md) fixed all four arm constructions, x-normalization, seeds, public-Q sample, first-shift-only rule, relation replay and null mathematics **before** the experiment. New test `b31_coverage_null_tests::actual_prime31_matched_b31_256q_unshifted_coverage` is appended to the existing native `examples/prime31_s9_b31_half_sum_oracle.rs`, reusing `Arithmetic`, `Base::build(PRIME31, 6)`, the original `FourSet::build`, and 512-way Montgomery `recover_batched`; no rewrite of the group law or fallback to Python.
- The curve arithmetic and native crate were pinned to original immutable commit `1df714645aa683cf975bc76fcacb9842cb4780fd`, Rust 1.100.0; candidate source was `d797fad388b78ef79082379679ce8c8812502971` for the [successful exact-source Actions run #44199751750](https://github.com/isogeny-labs/crypto/actions/runs/44199751750). Native **40/40 library tests** and new **1/1 focused test** passed, no failures. CI archive [#14381131375](https://github.com/isogeny-labs/crypto/actions/runs/44199751750/artifacts/14381131375) has GitHub-reported ZIP digest `sha256:44a524ffe3f89b5bb9488827050b51a038efe419b734e1e0e2df49c6aecc7d7d` (not independently ZIP-rehashed).
- [Committed complete JSON](B31_MATCHED_COVERAGE_NULL_RAW_20261117.json) preserves all four bases with 124 affine coordinates, all **1,024 public-target hit/miss bits** (four arms × 256 public Q), pairwise disagreements, exact complement-probe counts, field-work diagnostics, setup and full-query wall. Every returned witness was independently checked by the existing original curve `evaluate` assertion; every negative had exhaustive 4A traversal. No target scalar is passed to the membership oracle.
- Separate producer-written JavaScript readback verified the **exact 256-bit Dickson hit mask** (not only the count) against the first 256 recorded targets in #4623's prior raw 1,024-Q transcript, verified all 31 original Dickson points, all four panels' hit counts and unique x values, recomputed all pairwise contingency tables, and independently recalculated `N_4=618,885`/`N_8=5,654,193,417` by binomial sums. Separate BigInt modular arithmetic checked the Weierstrass equation for all **124 coordinates**. These are auditable cross-checks, not an independent signed PCC verdict or calibrated timing check.
- The four panels required 41,269,889 / 31,294,839 / 33,920,603 / 36,248,934 exact complement probes respectively. No cap or early negative misclassification occurred. Inverse/field-multiplication costs and exact first-witness decisions are saved in the JSON.
- At source-protocol scope this single GitHub runner was not CPU-isolated and the bases were always evaluated in fixed order; descriptive absolute wall alone cannot rank implementations. Research claim standing remains **[pcc:unverified]**; no leaderboard/PCC promotion and no claim of novelty or full-solver performance.

## Related leads already covered and nonduplicating next experiments

**Previously explored:**
- [#4623](https://github.com/isogeny-labs/crypto/pull/4623) achieved rank 31/31 and 1,024/1,024 target logs using existing original Rust generic-equivalent signed 4A+4A; 7,658.159 ms fully charged, versus **historical, different-run** 170.113 ms cached 1,024-log BSGS in [#4532](https://github.com/isogeny-labs/crypto/pull/4532). We specifically did not rerun this full pipeline or assert a matched speed ratio.
- [#4615](https://github.com/isogeny-labs/crypto/pull/4615), [#4618](https://github.com/isogeny-labs/crypto/pull/4618): public shift coverage and arithmetic batching on the existing generic 4A oracle. [#4624](https://github.com/isogeny-labs/crypto/pull/4624) is a **different** synthetic 48-curve interval/Dickson/random coverage audit and x-only Split4 control; its expanded results also fail to establish a universal Dickson advantage. [#4617](https://github.com/isogeny-labs/crypto/pull/4617) measures synthetic label/coverage and rank controls.
- [#4604](https://github.com/isogeny-labs/crypto/pull/4604) owns source-specific sign-free RR9 D9 **negative unit certificates**; earlier B8 Macaulay, seven-quotient and signed-branch routes #4548–#4550/#4556/#4575/#4581 were found too expensive or enumerate algebraic sign branches. [#4596](https://github.com/isogeny-labs/crypto/pull/4596) owns the matched 1,024-Q BSGS/pair-table IC frontier; [#4594](https://github.com/isogeny-labs/crypto/pull/4594), [crypto-autoresearcher #3301](https://github.com/isogeny-labs/crypto-autoresearcher/pull/3301), [#3305](https://github.com/isogeny-labs/crypto-autoresearcher/pull/3305), and [cryptanalysis #1536](https://github.com/isogeny-labs/cryptanalysis/pull/1536) document broader candidate/negative campaigns.
- Primary publications: Petit–Kosters–Messeng, PKC 2016 (https://doi.org/10.1007/978-3-662-49387-8_1); Amadori–Pintore–Sala, *Finite Fields and Their Applications* 51, 2018 (https://doi.org/10.1016/j.ffa.2018.01.009); Kudo and collaborators' prime-field index calculus/hybrid Gröbner work. Shared `research-kb` MCP/CLI unavailable in this connector-only runtime, so novelty search is **bounded**.

**Prioritized, falsifiable queue:**

1. **Actual-curve B27/B29/B31 pre-registered arity/coverage and rank trade-off**, with exactly the frozen 256 public Q and same original Rust oracle: vary special factor-base depth and random/first-x controls, charge total 4A setup and all failed target attempts; feed independent rows into 31-column-equivalent modular rank, accounting for actual base dimension. No special-base win is implied by a cheap occupancy heuristic.
2. **Sign-independent public-residual-only RR9/S9 coefficient and orientation recovery** using the existing Rust sparse F4/quotient/rootfinding engines, without `FourSet`, sign enumeration or secret-support advice. First exact B8 positive and two frozen D9 negative certificates, then Dickson B31 with collisions/repeated roots, infinity and exceptional residual branches. Demand original-ideal certificates for negatives and actual original-curve signed-witness replay for positives. Reject slow approaches before full-solver scale-up.
3. **Target-conditioned compact K961–S5 resultant / selected-root approach**, proving nonenumerative work and not merely materializing `961^4` join alternatives, with same B31 negative controls. Record cost and memory independently.
4. Once any truly algebraic extractor produces complete verified natural relations, **swap it into the existing #4623 rank and 1,024 unknown-log Rust harness**, same corpus, **all** failed shifts, setup, relation collection, linear algebra, rootfinding, verifier and fallback charged; compare isolated randomized-order A/B against equal-precomputation BSGS, tuned rho and mature PKM/APS/F4 external implementations. Seek independently signed PCC standing before any performance/complexity claim.

**Decision:** This completed *original-prime31* matched-base control removes the evidentiary basis for treating **high Dickson31 signed 8A coverage itself** as an established non-generic lead. It does not falsify target-conditioned algebraic Dickson structure, and it does not deliver a faster complete ECDLP solver.
