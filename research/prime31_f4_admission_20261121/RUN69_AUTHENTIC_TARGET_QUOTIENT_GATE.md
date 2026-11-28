# Prime31 run69 — genuine-RHS target-specific quotient rank, original-Rust control

**Date:** 2026-11-22 UTC. **Owner PR:** [#4901](https://github.com/isogeny-labs/crypto/pull/4901), draft. **Scope:** one deterministic source-identical cohort on the unchanged ordinary prime-field curve; **generic negative control**, not a non-generic ECDLP win.

## Exact question

Run59 proved a synthetic signed-pair quotient/rank stopping rule; run67 showed the difference between group-sum coverage and quotient-visible target logs, but its extra relation rows had no right-hand-side labels. Run68 upgraded to 3,520 **genuine labelled** pair relations and verified all 677 factor-base logs plus 32 independently recovered target logs. Run68 deliberately waited for *full factor-base rank* and used an exhaustive **916,469-key generic pair dictionary**.

**Run69 tests actual early stopping:** given true inhomogeneous `[k]G` relation equations and the checked 88 homogeneous identities, at what authentic query count do **all 32 registered target log linear forms** become uniquely determined, even if some factor-base logarithms remain unknown?

## Exact criterion and proof

Let \(F=\mathbb F_n\), \(n=2147446831\) prime, and write each factor-base log in an affine signed-DSU component as \(\ell_i=s_i z_{r(i)}+b_i\) where \(s_i\in\{\pm1\}\) and \(b_i\in F\). The actual labelled pair relations are \(s\ell_i+t\ell_j=k\) from public group equalities \(sP_i+tP_j=[k]G\); no scalar is supplied to the solver beyond public walk counters. Known root anchors eliminate their corresponding variables. Specializing the EC-verified 88 free rows produces a *consistent* linear system \(Az=d\) over the remaining unanchored root coordinates.

For a first signed-three-sum public witness of target \(Q\) at public shift \(j\), \(Q+[j]G=\epsilon_1 P_a+\epsilon_2 P_b+\epsilon_3 P_c\). Its unknown log is the affine form

\[
\ell_Q=\epsilon_1\ell_a+\epsilon_2\ell_b+\epsilon_3\ell_c-j=c^T z+\gamma.
\]

**Lemma:** \(\ell_Q\) is uniquely determined by \(Az=d\) iff \(c\in\operatorname{rowspan}(A)\) (assuming consistency). If \(c=\lambda^TA\), then \(\ell_Q=\lambda^Td+\gamma\); otherwise some \(v\in\ker(A)\) has \(c^Tv\ne0\), and changing \(z\) by \(v\) changes the target expression. Thus exact RREF elimination, rather than full base rank, decides whether each target log is known. Every returned target scalar is independently checked with original Rust \([\ell_Q]G=Q\).

The proof applies only to the **current relation subspace**. It is not a lower bound on all possible algebraic ECDLP strategies and does not assume statistical independence.

## Original Rust protocol

- Original crate: `research/prime31_ic_20261115/native`, original `Arithmetic/PRIME31/Base`, frozen `a=847514846,b=2087444724,p=2147483647,G=(625037715,2100627251),n=2147446831,h=1`.
- Original 509 Dickson points, frozen 168 additional y^7 points, exact B677. Check every free group identity, including coefficient `+2` at row 30, before collecting.
- Reuse run68's **generic pair table as an explicitly forbidden-to-qualify witness/collector reference**. For each public unknown \(Q\), find one signed-three-sum witness without leaking hidden target scalar into the search. Reconstruct 32 targets from the same deterministic harness cohort as run68 to provide an exact paired control.
- Walk \([k]G\) starting k=1, check every shift against the pair table, add **genuine RHS** affine signed-DSU relations. At every 32 successes, reduce all 32 target expressions against the specialized free-row RREF. Stop when **all 32 are determined**, even if the base rank remains deficient.
- Independent \([x]G=Q\) original-Rust checks, then post-solve harness-scalar and separate same-process BSGS checks. Charge table/setup, witness discovery, all failed relation shifts, actual RREF, group checks and BSGS. Cap collection at 12 million shifts and target witness discovery at 1024 shifts; any cap hit is **censored failure**, not a win.
- Source: [prime31_run69_genuine_partial_rank.rs](../prime31_ic_20261115/native/examples/prime31_run69_genuine_partial_rank.rs). Run: `cd research/prime31_ic_20261115/native && cargo run --locked --release --example prime31_run69_genuine_partial_rank`. Dedicated [source-specific workflow](../../.github/workflows/prime31-run69-genuine-partial-rank.yml) uses Rust 1.100.0.

## Admission and interpretation gates

**PASS (correctness only):** 32/32 target logs independently verified, all relations have authentic RHS, and the source prints actual query count, true rank, remaining free variables/full-rank boolean, and fully charged time. Compare authentic query count with **run68 8,267,474 shifts** on the identical cohort. A source-specific GitHub CI success is needed before treating any new metric as measured.

**FAIL/CENSORED:** compile error, contradictory RHS/free rows, invalid signed group witness, target coefficient not row-visible after 12M shifts, or mismatch against \([k]G=Q\) or BSGS. A partial-rank solver returning 31/32 is not complete.

**NOT A NON-GENERIC WIN:** both run68 and run69 use an exhaustive generic pair-sum dictionary and generic three-sum witness enumeration. This is an exact stage-integration experiment only, not a novel index-calculus oracle. Even an improved query count is not a qualified performance comparison against the best index calculus, negation-map BSGS or optimized rho.

## Previous leads and active owners (reviewed before implementation)

- [#4901](https://github.com/isogeny-labs/crypto/pull/4901) runs 55–68: run59 synthetic quotient-rank, run64 projective S3 component, run65 shifted-support correlations, run66 three-sum coverage, run67 synthetic target visibility, run68 genuine RHS and full-rank generic calibration.
- [#4692](https://github.com/isogeny-labs/crypto/pull/4692): actual **public-only RR9/F4** and 46-positive/18-negative blind corpus. Its source report says checkpoint gate blocked when transport credential unavailable; do **not** claim its blind corpus is executed.
- [#4822](https://github.com/isogeny-labs/crypto/pull/4822): sparse four-variable RR9/Krylov implementation and partial tests; not an established complete Prime31 non-generic solver.
- [#4812](https://github.com/isogeny-labs/crypto/pull/4812): transformed-slot Horner/F3 chart; 512 known-witness algebraic correctness controls, **no blind extraction**.
- [#4809](https://github.com/isogeny-labs/crypto/pull/4809): existing native Rust F3 quadratic root extraction; no duplication of that separate lead here.
- [#4816](https://github.com/isogeny-labs/crypto/pull/4816): independently built small-field nonenumerative extractor with negative Prime31 cost scaling; do not transfer its small-field advantage to Prime31.
- Primary prior work: Semaev, [ePrint 2004/031](https://eprint.iacr.org/2004/031); Amadori–Pintore–Sala (2018), [DOI 10.1016/j.ffa.2018.01.009](https://doi.org/10.1016/j.ffa.2018.01.009), one-relation variant; Kudo–Yokota–Takahashi–Yasuda, [hybrid acceleration and limitations](https://kyushu-u.elsevierpure.com/en/publications/acceleration-of-index-calculus-for-solving-ecdlp-over-prime-field/). Shared research-KB MCP and local Rust compiler were unavailable in this execution, so linked primary records and exact-source GitHub files are the fallback; no exhaustive novelty claim.


### Conditional query-cost admission envelope (derived, not a benchmark)

If a future qualifying extractor needed the *same* 5,378,231 Prime31 queries to recover 32 targets, then matching the **within-run reference** BSGS totals of 158.320 ms / 235.428 ms would require charging at most **29.44 / 43.77 nanoseconds per query for the entire hypothetical pipeline**, **including** all setup, coordinate generation, polynomial extraction, failures, rank and verification. Since setup is nonzero this is a mathematically generous **upper envelope**, not a measured oracle throughput or a universal lower bound. Even run64's single-pair projective S3 evaluation is a component with no unknown-index roots, and the four-variable RR9/F4 rootfinder must not inherit its nanosecond timing. With the present B677 two-summand density, an F4 backend handling millions of queries faces an extraordinarily demanding threshold. It is rational to prioritize genuinely higher-arity nonenumerative extraction with increased independent-rank yield per attempt over further generic pair-table micro-optimization.

For a new relation predicate (three-, four- or eight-leaf) that changes the query count, **recompute** this envelope from its own actual authenticated RHS rank trajectory; do not extrapolate the 5.38M count to other algorithms.

## Next decisive experiment

Wire **a nonenumerative public-only RR9/F3 extractor** from #4692/#4822/#4812 into the existing authentic `(coefficients,RHS)` recovery interface. Use its registered blind 46-positive/18-negative corpus; prohibit any precomputed generic pair table in the candidate; charge negatives/timeouts/OOMs, true independent rank, and all group checks. Only if it yields sufficient rows, complete cold one-target and 32/128/1024-target unknown-scalar solves on unchanged Prime31 and compare against the fastest existing index-calculus implementations, optimized BSGS and negation rho on the **same host**. Keep PR draft pending this gate.

## Outcome

**Source-specific original-Rust PASS twice.** [First Actions #45060772593](https://github.com/isogeny-labs/crypto/actions/runs/45060772593), job 137325960852, and [second Actions #45060777274](https://github.com/isogeny-labs/crypto/actions/runs/45060777274), job 137325975373, both compiled and ran source blob `fa997afaed7d9fec915c1e6a5f53449240e8cdc6` unchanged. Both printed `RUN69 PASS`, independently checked 32/32 true-RHS quotient-recovered logs with the original Rust group law, agreed with hidden harness scalars *only after solving*, and matched separate same-process original-arithmetic BSGS.

| Actual metric | Run69 CI 1 | Run69 CI 2 |
|---|---:|---:|
| Public relation shifts (all failures counted) | 5,378,231 | 5,378,231 |
| Genuine labelled pair hits | 2,272 | 2,272 |
| Independent signed-DSU pair rows | 651 | 651 |
| Unanchored DSU roots before free-row elimination | 26 | 26 |
| Factor-base full rank at stop | **No** | **No** |
| Recovered and group-verified target logs | **32/32** | **32/32** |
| Factor base + generic dictionary setup | 224.803 ms | 353.121 ms |
| First public target-witness discovery | 43.651 ms | 65.119 ms |
| Genuine RHS walk + target-quotient computations | 1,191.009 ms | 1,758.414 ms |
| Complete pipeline including harness | **1,618.285 ms** | **2,412.716 ms** |
| Same-process reference BSGS for same 32 logs | 158.320 ms | 235.428 ms |
| Complete generic control / BSGS | **10.22× slower** | **10.25× slower** |

**Actual mathematical improvement:** Relative to frozen run68's full-factor-base-rank stop on the *same* 32 public targets and `kG` walk, exact target-specific quotient stopping saves **2,889,243 of 8,267,474 shifts = 34.9471%**. It also needs 2,272 rather than 3,520 true-RHS pair matches (35.45% fewer), and stops without knowing all 677 base logs. This is no longer a synthetic rank prediction: the output 32 scalars were recovered and verified.

**Cost qualification:** The two executions used the same deterministic cohort and curve, not independent random cohorts. Absolute runtimes varied with the runner; run68 and run69 whole-pipeline timings came from *different* workflow executions and must not be read as a matched-host speedup ratio. Within each run69 execution the generic reference solver remains about **10.2× slower** than that execution's non-optimized BSGS. No admissible RR9/F3 public-only nonenumerative extractor has been substituted, nor any best-index-calculus or optimized negation-rho match.

Frozen [RUN69_RESULTS.json](RUN69_RESULTS.json) records both exact source-specific run IDs and all measured cost components. Keep PR draft; target quotient is validated as a reusable downstream optimization, **not** a cheaper qualifying non-generic prime-field ECDLP solver.
