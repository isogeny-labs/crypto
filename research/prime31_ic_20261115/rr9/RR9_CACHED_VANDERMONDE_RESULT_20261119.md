# Prime31 B8 RR9 cached Vandermonde: complete chart-building optimization

**Source-specific PASS, 2026-11-19 UTC.** [PR #4783](https://github.com/isogeny-labs/crypto/pull/4783) updated on top of [#4780](https://github.com/isogeny-labs/crypto/pull/4780); [passing GitHub Actions run #44465566138](https://github.com/isogeny-labs/crypto/actions/runs/44465566138). The exact-source code/protocol commit of this run is `936d12053c64b65f41b74f6a1ab8e7604218fdaa`, with unchanged native Prime31 arithmetic pinned at `1df714645aa683cf975bc76fcacb9842cb4780fd`. Preregistered [protocol](RR9_CACHED_VANDERMONDE_PROTOCOL_20261119.md) was committed **before code**, not after seeing measurements. Independent signed PCC: **[pcc:unverified]**.

## Exact identity and operation reduction

For four distinct signed leaves `Q_i=(x_i,y_i)` and a public target `R=(r,s)`, original RR9 rows are `h_i=[(x_i-r)x_i^j | (y_i+s)x_i^j]_{j=0}^3`. Set the ordinary four-by-four Vandermonde matrix `V_{ij}=x_i^j`, and `z_i=(y_i+s)/(x_i-r)`. If no `x_i=r`,

\[
A=\operatorname{diag}(x_i-r)V,\qquad B=\operatorname{diag}(y_i+s)V,\qquad
M=A^{-1}B=V^{-1}\operatorname{diag}(z_i)V .
\]

Writing `v_i` for Vandermonde row i and `c_i` for column i of `V^{-1}` gives the reusable rank-one expansion

\[
M=\sum_{i=0}^{3}z_i\,(c_i v_i),\qquad
\det A=\det V\prod_{i=0}^{3}(x_i-r).
\]

Changing `Q_i` to `-Q_i` changes `y_i`, not `x_i`; the 16 sign patterns of an unsigned four-leaf tuple therefore **reuse the same four outer-product matrices**. On B8 there are `8^4 = 4096` unsigned abscissa tuples per half rather than `16^4 = 65536` signed charts. Build the two caches once (8,192 unsigned tuples), compute only 64 target-denominator inversions for all eight slots, and construct the target's full 131,072 signed four-row charts using field products/additions.

This is exact **classical Vandermonde linear algebra** combined with existing #4780 Grassmann graph charts, not a claim of mathematically novel subexponential ECDLP. It reduces inversion-bearing chart construction, **not** the subsequent relation-search pair comparisons.

## Registered original-curve experiment

Original unchanged curve `p=2^31-1=2147483647`, `E:y²=x³+847514846x+2087444724`, group order `n=2147446831`, cofactor one, `G=(625037715,2100627251)`; base x values `[0,1,2,3,5,6,8,9]`; existing weighted signed supports `[1,3,5,7,11,13,17,19]`. Two public residuals per arm: the ordinary generator `G` and a nontrivial public sum of the first eight weighted signed leaves. Each arm constructs **262,144** chart entries (two targets × two halves × `16^4` tuples).

| Paired same-GitHub-runner full B8 half-table generation | Fresh Gaussian `A` | Cached unsigned Vandermonde |
|---|---:|---:|
| A1/B1 | 175.625104 ms | **25.700974 ms** |
| A2/B2 | 166.112302 ms | **25.721692 ms** |
| **Mean** | **170.868703 ms** | **25.711333 ms** |
| **Warm chart-generation ratio** | 1× | **6.646× faster** |

- **Target-independent cache setup: 7.953809 ms.** Charging it once to the two-target cached mean gives 33.665142 ms, or **5.076×** faster for this particular two-target **chart-building-only** workload.
- Exactly matching producer digests for all four runs: `1790301562`. Tests compare all original B8 chart entries, **not only hashes**.
- Nominal target-dependent inversion counts: baseline `2 × 16^4 × 4 = 524288` chart pivots per target, versus **64** leaf denominators per target once target-independent cache exists. The cache setup itself performs inversion-bearing Vandermonde preprocessing; it is separately counted and timed.
- Memory/setup, existing signed supports and full EC group-sum relation-search work are **not** part of the timed inner chart arm. The listed ratio is not a whole-extraction, wall-time ECDLP or asymptotic improvement.

## Independent correctness and durable evidence

[Raw GitHub artifact #14502390564](https://github.com/isogeny-labs/crypto/actions/runs/44465566138/artifacts/14502390564) has ZIP SHA-256 `3e64baefd91c74cc4c0405ee5129c833a310c8293234a224129bfb5af7f3bea7`; inside JSON SHA-256 `38a283660e2b80eb4109e3c7747d6da704357b6a56401f30c5a6c46244942aa1`. [Frozen machine-readable summary](RR9_CACHED_VANDERMONDE_SUMMARY_20261119.json).

- The original Rust library passed **40/40** tests. The example passed **8/8**, including complete equality of 262,144 cached Gaussian charts and determinant factors across two public residuals and both half tables, and the explicit singular `x_i=r` reject gate.
- An independently written Python BigInt verifier reconstructed **128** original-curve half charts and **64** original-curve full 8×8 determinant/Schur equalities, checking every sampled point's curve equation, without using Rust field arithmetic or producer summary digests. Both the GitHub runner and a separate raw-artifact replay passed.
- Current signed PCC acceptance is not obtained, despite independent arithmetic replay; do not mark `[pcc:verified]`.

## Competitive interpretation / prior-lead audit

This addresses the **offline/online chart-construction inversion burden**, previously untouched by the #4783 Sherman–Morrison row replacement. It does **not** solve #4692's algebraic **unknown signed index** problem. The remaining B8 outer loop is exactly `(2B)^8 = 16^8 = 4,294,967,296` algebraic candidate pairs for a fully exhausted public target. At 30 multiplication products per pair under #4780's determinant, that is `128,849,018,880` inner products per *negative* target, independently of whether cache generation is free. No faster complete prime-field ECDLP result follows.

This audit explicitly checked:

- [#4776](https://github.com/isogeny-labs/crypto/pull/4776) 70-term exterior-four, [#4780](https://github.com/isogeny-labs/crypto/pull/4780) 30-product Grassmann graph determinant, [#4783](https://github.com/isogeny-labs/crypto/pull/4783) one-leaf rank-one update, and [#4779](https://github.com/isogeny-labs/crypto/pull/4779) unrelated fixed Plücker span rank.
- [#4692](https://github.com/isogeny-labs/crypto/pull/4692) owns public-only nonenumerative F4/remainder-tower extraction; [#4728](https://github.com/isogeny-labs/crypto/pull/4728) has generic-table rank31/1024-log four-window solver, [#4774](https://github.com/isogeny-labs/crypto/pull/4774) matched signed BSGS (3.343 s vs 7.835 s for generic W8), and [#4773](https://github.com/isogeny-labs/crypto/pull/4773) conditional two-prefix elimination.
- [Autoresearcher SA-04 / SA-18 negative norm and determinant leads](https://github.com/isogeny-labs/crypto-autoresearcher/blob/d54c01d325b9e4508d51f56dc113833be27ddc9a/experiments/semaev-alternatives-20261105/SUMMARY.md), and [cryptanalysis Rust prime-field baseline](https://github.com/isogeny-labs/cryptanalysis/blob/6b64ee6ca07f92db014cf24d277439730ac31f4b/suite/src/cryptanalysis/ec_index_calculus.rs).
- Petit–Kosters–Messeng, [PKC 2016](https://doi.org/10.1007/978-3-662-49387-8_1) and Amadori–Pintore–Sala, [FFA 2018](https://doi.org/10.1016/j.ffa.2018.01.009), as prior finite-field index-calculus research. **KB unavailable:** shared KB tool and runtime CLI absent; this is a bounded audit, not proof of novelty.

## Ranked next decisive experiments

**P0: public-only nonenumerative RR9/F4 extraction (§#4692).** Input public `R` and signed factor-base supports **only**; algebraically discover all unknown leaf choices without generic EC four-sum tables or 16^8 pair scanning. Score the frozen **46 positive / 18 negative** natural B8 targets, including memory, failures, exact cost, independently checked witnesses, and strict custody of grader-only planted indices. Use the cached Vandermonde basis if actual profiling shows F4/remainder matrix construction as a significant bottleneck; otherwise keep it isolated.

**P1: precompute barycentric differences without per-tuple inversion.** Use `V^{-1}` from Lagrange coefficients and precomputed pairwise inverses `1/(x_i-x_j)`. The 8×8 unsigned leaves in eight slots yield at most 1,792 inter-slot x-differences, fewer than the current 8,192 unsigned `V^{-1}` constructions. Measure cache cold setup, memory, target-online work, and independent full B8 equality; explicitly stop if a one-target cost regresses. This is *another preprocessing lever*, not a remedy for exhaustive B8 pairwork.

**P2: whole-solver cost gate.** If and only if P0 passes and extraction is cheaper, replace the 4+4 oracle in #4728, recover 31/31 factor-base rank and 1,024/1,024 blinded logs using existing Rust EC operations. Fully charge cold and amortized setup, rejected targets, field/EC operations, memory, algebraic solver and linear algebra. On a single host compare with strongest APS/Semaev IC, signed shared BSGS (#4774) and optimized rho, not just weak generic baselines.

**Result classification:** verified source-specific **6.646× warm / 5.076× cache-included two-target** chart-building speedup; **no** demonstrated novel non-generic complete ECDLP advantage; signed PCC still pending.
