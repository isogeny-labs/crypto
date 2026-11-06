# Protocol: trace-zero factor-base toy test (frozen before data)

2026-11-06. Draft frozen by commit; no code or data exists yet. Changes after the
freeze go in dated amendments below. Origin: an unverified note [pcc:unverified]
that a factor base of traces Tr(Q) = Q + πQ + … + π^(k−1)Q, with x(Q) in a small
F_p-subspace V of F_{p^k}, might show structure; the note expects it not to beat rho.

## Question

Does the trace-based factor base F_V = { Tr(Q) : Q ∈ E(F_{p^k}), x(Q) ∈ V } ⊂ E(F_p)
give more m-fold decompositions of a random target R ∈ E(F_p) than a random factor
base of the same size?

## Setting

- p ∈ {11, 13, 17, 19, 23, 29, 31}; k ∈ {2, 3}; E: y² = x³ + ax + b over F_p, 200
  random non-singular (a, b) per (p, k). Seeds derived from (p, k, a, b) by SHA-256.
- V ⊂ F_{p^k}: an F_p-subspace of dimension d, d = 1 and d = 2, 20 random V per curve.
- m ∈ {2, 3}. Yield Y(F, m) = number of unordered m-multisets of F ∪ −F summing to R,
  averaged over 50 random targets R.

## Arms (same instrument, same targets, interleaved order)

1. **Trace arm**: F_V as above.
2. **Random control (negative)**: |F_V| points of E(F_p) chosen uniformly (no structure).
3. **Gaudry control**: F = { P ∈ E(F_p) : x(P) ∈ V ∩ F_p } for k = 1 comparison, at the
   same size where V ∩ F_p is large enough (reference for the usual subspace gain).
4. **Positive control**: F_V with a planted bias: points chosen from a subgroup of index 2
   of E(F_p). The detector must flag it (Y ratio ≈ 2^(m−1) expected).
5. **A/A**: the trace arm run twice with different seeds; the ratio must be 1 within noise.

## Statistic and decision

Ratio ρ = Y(trace) / Y(random), per (p, k, d, m) cell; report the median and a 95%
bootstrap interval over curves. Effect size is the log ratio, not a p-value.

- Prediction H0: ρ = 1 within the A/A band in every cell.
- Falsified (structure found) if the interval excludes 1 in the same direction in at
  least 3 of 4 (k, d) cells at p ≥ 17, and the positive control passes.
- Inconclusive if the positive control fails or the A/A band is wider than ±5%.

## Out of scope

No cost model, no rho comparison, no summation-polynomial solving, no claim about
asymptotics. A positive ratio at toy sizes would justify only a larger follow-up.
Any later cross-method comparison needs the §5a priced band.

## Execution

Local, small (seconds per cell, no EKS). The Rust or Python instrument is written
after this freeze. Conductor and research-kb were not reachable in this session
(`conductor: command not found`), so the KB check below is a grep of the repository.

## Sources checked

- `repo:docs/ic/ECDLP_RESEARCH_TECHNIQUES.md` lines 29, 64–78: trace-zero is integrated
  only for Koblitz subspace bases; a general trace-zero-variety pipeline does not exist.
- `repo:docs/ecdlp-hardness-corpus/ATTACK_GRAPH.md` row 380 (`THM-RP-TRACE-ZERO-DOUBLING`):
  trace-zero bases lie in 2E and double the yield at the same degree (proved internal,
  unreviewed). This is the Koblitz case; the arm here is a different construction.
- `repo:research/ghs_q16_tracezero_genus2_20261028`: related GHS work, not read in full.
- KB `search_knowledge`: unavailable.

## Amendment A1 (2026-11-06, before any code or data)

Found while designing the instrument. The registered statistic is degenerate.
Averaged over all targets R, the mean yield of m-multisets of S = F ∪ −F is
C(|S|+m−1, m)/|E(F_p)|: it depends only on |S|, so a size-matched control always
gives ratio 1. Structure can only show in how the sums are distributed over R.
Changes (all before data):

1. **Statistics** replace the yield ratio: (a) |S|, the number of distinct non-identity
   points the trace map reaches; (b) coverage cov_m, the fraction of non-identity R in
   E(F_p) that are a sum of exactly m elements of S, for m = 2, 3.
2. **Null for the trace arm**: draw the same number of points as there are lifts Q with
   x(Q) ∈ V, uniformly from E(F_p), drop the identity, close under negation. This tests
   whether Tr on lifts is equidistributed. Comparison is the difference
   Δ = trace − random, per (curve, V), averaged per curve.
3. **Targets**: exhaustive over E(F_p) (at most about 45 points), not 50 random.
4. **Positive control**: draws restricted to the subgroup H = 2E(F_p) (or 3E(F_p) if
   |E(F_p)| is odd). Curves with H = E(F_p) are skipped for this arm. Expected: Δ_pos < 0
   in cov_m.
5. **Gaudry control** (arm 3) dropped: it is a k = 1 construction and not the same
   instrument. The A/A arm is the trace arm on an independent second draw of V.
6. **Decision rule** with Δ in place of the ratio: structure is claimed if, for the same
   m, the 95% bootstrap interval of the pooled (p ≥ 17) mean Δ excludes 0 in the same
   direction in at least 3 of the 4 (k, d) cells and the positive control interval
   excludes 0. Inconclusive if the positive control fails or the A/A interval half-width
   exceeds 0.05 in coverage. Cells where the random arm has mean cov_m > 0.99 are
   saturated and reported as uninformative, not as support for H0.
7. **Data**: per-curve rows go to the scratchpad and are pinned by SHA-256 in
   `summary.json`; they are over the 64 KiB data-file limit. The artifact store route is
   not available in this session.

## Amendment A2 (2026-11-06, after a 32-row smoke run, before the registered run)

The smoke run (4 curves × 3 V, p ∈ {11, 17}, outputs not kept) showed cov_2 = cov_3 = 1.0
for nearly every arm: S + S already covers E(F_p). Under A1 item 6 most cells would be
saturated. Added, with the same controls and decision rule: a **small-base arm**. From each
arm's draws take s = 3 random distinct non-identity points (skip if fewer exist), then
compute |S|, cov_2, cov_3 as before. The primary decision statistic is cov_2 and cov_3 of
this small-base arm; the full-base statistics stay as secondary. The smoke run informed
only this change and no decision threshold.

## Amendment A3 (2026-11-06, before the registered run)

The first launch hung: p = 11 and 13 have only 110 and 156 non-singular (a, b), fewer
than the 200 curves registered, and the curve sampler looped. No row was written. For
those primes all non-singular (a, b) are used. Both primes are below the p ≥ 17 pooling
threshold, so no decision statistic is affected.
