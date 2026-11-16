# Protocol: the conjugate-pair family (frozen before data)

2026-11-06. This is the third piece checking the claims that came from
another session (an unverified assertion). The earlier pieces are
[`lift_dense_torsion_pairing_20261106`](../lift_dense_torsion_pairing_20261106/README.md)
and
[`lift_dense_orbit_parity_20261106`](../lift_dense_orbit_parity_20261106/README.md).
This file is committed before any code or computation in this directory.

## The claim

> **Conjugate-pair case (new).** Two roots are Frobenius-conjugate. This
> needs n even, n ≥ 4, and full 2-torsion over K. There is no descent and
> the columns are useful. A generic such curve over F_{q⁴} has 6 of these
> lines, found by solving one F_q-linear equation. All 146 lines I
> constructed behaved as predicted. Only 3 of about 1,050 points were
> dead, and the number of distinct columns was at chance level.

## Construction (derived here before data)

- **Setting.** E : y² = (x − e₁)(x − e₂)(x − e₃) with e_i ∈ K = F_{q^n},
  q = p odd, and n even. K₀ = F_{q^{n/2}}, and σ is the q-Frobenius. A line
  is x₀ + u·F_q, and t_i = (e_i − x₀)/u.
- **Root pattern.** The orbit criterion (verified in the previous piece)
  holds for the pattern "t_a and t_b in one orbit, t_c alone with
  t_c ∈ K₀". For t₂ = σ^k(t₁) the pair is in one orbit with r = 2 and
  s | n, so 2n/s is even. For t_c ∈ K₀ the orbit has r = 1 and s | n/2,
  so n/s is even.
- **Linear system.** Fix c ∈ {1, 2, 3} for the lone root and an ordered
  pair (a, b) of the other two. Put w = 1/u, x₀ = e_c − γu with γ ∈ K₀,
  and d_i = e_i − e_c. Then t_c = γ ∈ K₀ automatically, and t_b = σ^k(t_a)
  becomes

  d_b·w + γ − σ^k(γ) − σ^k(d_a)·σ^k(w) = 0,

  which is F_q-linear in (w, γ) ∈ K × K₀.
- **Solution space.**
  - It contains the shift (w, γ) = (0, F_q), which is the same line.
  - Generic dimension count: n + n/2 unknowns over F_q, n equations, so
    n/2 dimensions. Modulo the shift and F_q-scaling, that leaves a
    projective space of dimension n/2 − 2.
  - n = 4: exactly one line per (c, (a, b), k) with k ∈ {1, 3}, provided
    w ≠ 0.
  - (c, (a, b), k) and (c, (b, a), n − k) give the same line. So n = 4
    gives 3 · 2 = **6 lines** (k = 1 for both orders of the pair). This
    agrees with the claim.
  - k = n/2: σ^{n/2} fixes γ, so the equation reduces to
    σ^{n/2}(d_a w) = d_b w. That needs N_{K/K₀}(d_b/d_a) = 1, so generically
    there are no lines.
  - n = 2: the only k is n/2, so generically **0** lines.
  - n = 6: P¹(F_q) per system, with k ∈ {1, 2} and both orders. That
    predicts 3 · 4 · (q + 1) lines, before deduplication and degenerate
    w = 0. This is reported only.

## Instrument

This is a new std-only Rust crate, `conjpair/`. Its field, polynomial,
root, curve and SHA-256 code is copied from the previous piece's `orbit/`
crate, which was checked there (unit tests and A/A). The new code is:

- the linear system over F_p: Gaussian elimination on B-coordinates;
- line deduplication by the sorted set of the line's x-values;
- point counting by baby-step giant-step (BSGS) in the Hasse interval, with
  the candidate intersected across 8 random points. It is cross-checked
  by full enumeration at (13, 4);
- the projection [#E/ℓ^v]P;
- a planted positive control for the column detector.

Lines are also evaluated with the previous piece's `line_eval`:

- the criterion and its orbits;
- the counts of squares, nonsquares and zeros of f along the line;
- whether the root set is σ^{n/2}-stable.

## Cells and arms

| cell (p, n) | curves | role |
|:--|--:|:--|
| (13, 2), (101, 2) | 8 each | control: predicted 0 lines |
| (13, 4), (101, 4) | 16 each | main: predicted 6 lines per curve |
| (13, 6) | 4 | report only: line count against 12(q + 1) |

Each curve has e₁, e₂, e₃ uniform, distinct in K, which gives full 2-torsion.
j ∉ K₀ is recorded per curve. Each cell uses a fixed seed string
(SplitMix64).

## Predictions (decision rules, coded in the summarizer)

**C1 (count, n = 4).** On ≥ 90 % of curves the k ≠ n/2 systems give
exactly 6 distinct lines with w ≠ 0. No curve gives more than 6. The
k = n/2 systems give 0 lines on ≥ 90 % of curves.

**C2 (behaviour, exact).** Every constructed line (n = 4 and n = 6):

- satisfies the criterion;
- has its lone root in K₀ and the pair in one σ-orbit;
- is constant (all-lift or all-twist);
- is **not** σ^{n/2}-stable;

and its curve has j ∉ K₀. Any exception falsifies "no descent" or the
construction.

**C3 (n = 2 control).** 0 lines on every n = 2 curve.

**C4 (dead points, n = 4).** For each all-lift constructed line, take
dead = [#E/ℓ^v]P = O, where ℓ is the largest prime of #E(K).

- Pooled per cell, the observed dead count is ≤ E + 3√E + 3, where
  E = Σ_lines k/ℓ^v is the chance expectation.
- This is the claim "only 3 of about 1,050 dead", against about half on
  descent lines.

**C5 (distinct columns, n = 4).** Among the non-dead lifted x on all-lift
constructed lines, count the pairs whose projections agree up to sign.
Pooled per cell, that count is ≤ E₂ + 3√E₂ + 3, with
E₂ = Σ_lines C(k_alive, 2)·2/ℓ^v. This is the claim "distinct columns at
chance level".

**C5+ (positive control, exact).** On each such line, adding P + T_c for
every lifted P must not raise the number of distinct projections: each
new point collides with its source. If it does raise it, the detector is
broken and C5 is void.

**C6 (n = 6, report only).** The number of distinct lines per curve
against 12(q + 1) = 168.

**Group orders.** The BSGS order must equal the enumerated order on every
(13, 4) curve, and [#E]P = O must hold on 8 points of every curve. If
either fails, C4 and C5 are void.

## What would change the lead

- If C1–C5 hold, the conjugate-pair family stands as claimed at toy scale:
  6 lines per generic F_{q⁴} curve with full 2-torsion, no descent, and
  columns that are neither dead nor paired beyond chance.
- That makes it the only family in these three pieces that is a candidate
  lever on generic curves. Its cost (2^m yield per trial, against 6 lines
  per curve, and the share that is all-lift) is still unmeasured.
- If C1 or C2 fails, the claimed family does not exist as stated.

## Controls, A/A, analyst

- **Controls.** The n = 2 cells are the negative control for existence.
  C5+ is the planted positive control for the column detector.
- **A/A.** Two runs, with byte-identical rows required.
- **Data size.** Rows over 64 KiB go to the artifact store, pinned by its
  manifest, with the validator report pinning the manifest (AGENTS.md
  §13).
- **Analyst.** The producer, using coded rules.

## Amendments

**A1 (2026-11-06, after data; it changes no decision rule).** These are
post-hoc analyses. The C1 and C3 verdicts stay FAIL as registered.

1. **The 19-line (13,4) curve.** Two of its systems (one system and its
   mirror) have nullity 3 instead of 2, which gives a pencil of q + 1
   lines. The protocol's "none > 6" assumed the generic nullity.
2. **The (101,2) curve.** It has j ∈ F_q by chance, so its k = n/2 systems
   have nullity 2. The instrument's `half_k_lines` counts solution points
   (projective points with w ≠ 0), not distinct lines. Modulo scaling and
   shift those collapse to about one line (derived, not deduplicated in
   code).
3. **New tables.** A null-space dimension tally per cell, and the n = 6
   behaviour count.
4. **Before data.** BSGS was changed after a failing unit test and before
   the run: it now uses exact point orders and returns None when the order
   is ambiguous. That change is in the instrument commit `30a9b09fb5`.
