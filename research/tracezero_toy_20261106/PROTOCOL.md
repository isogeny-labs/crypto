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

## Amendment A4 (2026-11-06, POST HOC, after the registered run)

The registered run (A1–A3; `summary.json`, rows SHA-256 pinned there) gave "no structure
detected" under the decision rule (2 of 4 cells moved, both d = 1, both negative). But the
d = 1 cells show a small significant shortfall of the trace arm (Δ|S| ≈ −0.16, Δcov_2 ≈
−0.027, interval excluding 0). The null has a defect: lifts come in ±Q pairs, so trace draws
are sign-symmetric, while the random null draws independent points. A symmetric-pair null
(draw n/2 points and their negatives) is run as a **post hoc secondary** (`TZ_SYM=1`,
`summary_symnull.json`). It does not replace the registered verdict.

## Amendment A5 (2026-11-06): pre-registered replication of the k = 2, d = 1 residual

Frozen before the replication data. The post hoc A4 run left one residual: k = 2, d = 1,
Δcov_2 = −0.0086 [−0.0125, −0.0050] under the sign-symmetric null.

- **Run**: `TZ_SALT=rep1 TZ_SYM=1`, k ∈ {2, 3}, d = 1, p ∈ {37, 41, 43, 47, 53, 59, 61},
  200 curves × 20 V-pairs per (p, k), all independent seeds (salt changes every derived
  seed). k = 3 is the reference arm where A4 showed no effect.
- **Statistic**: small-base cov_2 (primary), cov_3 and |S| (secondary), Δ = trace − random
  with the symmetric null, curve-level means, pooled over the seven primes, 95% bootstrap.
- **Prediction (from A4)**: if the residual is real, k = 2 pooled Δcov_2 < 0 with an interval
  excluding 0, and k = 3 contains 0. If it is a fluke or small-p, k = 2 contains 0.
- **Validity**: positive control excludes 0 and A/A half-width ≤ 0.05, as before.
- **Reading**: replicated only if the k = 2 interval excludes 0 on the negative side. This
  tests one cell, so it says nothing about other (k, d) and no cost claim follows.

## Amendment A6 (2026-11-06): pre-registered rerun without degenerate V

Frozen before the data. A4/A5 and `explore_line_classes.py` explain the k = 2, d = 1 residual
by degenerate V (V contains a nonzero g with g^p = g or g^p = −g). This run excludes them:
`TZ_EXCL=1 TZ_SYM=1 TZ_SALT=rep2`, both V draws (trace and A/A) resampled until non-degenerate.
For k = 3 only g ∈ F_p can occur; for d = 2 the whole span is checked.

- **Runs**: (a) d = 1, k ∈ {2, 3}, p ∈ {17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61};
  (b) d = 2, k ∈ {2, 3}, p ∈ {17, 19, 23, 29, 31}. 200 curves × 20 V-pairs; independent seeds.
- **Statistics and rule**: as A1/A2, pooled over p ≥ 17 per (k, d) cell, symmetric null.
- **Prediction**: if the explanation is complete, every (k, d) cell's Δcov_2 and Δcov_3 interval
  contains 0 and the registered verdict is "no structure detected". If any interval excludes 0
  the residual is not fully explained by degenerate V.
- **Validity**: positive control excludes 0; A/A half-width ≤ 0.05.

## Amendment A7 (2026-11-06, before the A6 part (b) data; part (a) results seen)

A6 part (b) hung with no rows: for k = 2, d = 2 the subspace V is the whole field F_{p²},
which always contains F_p, so excluding degenerate V rejects every draw. That cell cannot be
run under A6 and is dropped (this also means the earlier registered k = 2, d = 2 cell was
V = F_{p²} on every draw, not a random subspace). Part (b) becomes k = 3, d = 2, p ∈
{17, 19, 23, 29, 31}, otherwise as A6. Part (a) is unchanged and its results are reported
as they came out (see README).
