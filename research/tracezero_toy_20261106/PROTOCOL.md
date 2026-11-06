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
