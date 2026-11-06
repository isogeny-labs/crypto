# Protocol: yield and cost of the conjugate-pair factor base (frozen before data)

2026-11-06. This follows
[`lift_dense_conjugate_pair_20261106`](../lift_dense_conjugate_pair_20261106/README.md),
which found 6 conjugate-pair lines per generic F_{q⁴} curve with full
2-torsion. About half of them are all-lift, and their columns are neither
dead nor paired beyond chance. This file is committed before any code or
computation in this directory.

## Question

How do these numbers compare against the standard base on the same curves
and targets, in a complete index-calculus solve?

- relations per decomposition trial;
- relations per column;
- trials to a full-rank system;
- linear-algebra size.

The inferred claim under test is "2^m yield per trial". The algebra predicts
2^m per trial and 2^{m−1} per column.

## Instrument

This is a new std-only Rust crate, `cpcost/`, with its own arithmetic in
K = F_p[z]/(m_K) of degree n = 4. The modulus is checked by Rabin's test.
Everything is computed in K; the roots are chosen in K, which gives full
2-torsion.

- **Line construction.** Conjugate-pair lines come from the same F_q-linear
  system as the previous piece, re-implemented in K-coordinates. The
  previous piece's count (6 lines on a generic curve) is a unit check.
- **Order.** #E(K) is counted by enumeration at q = 13. At q = 101 it uses
  BSGS with exact point orders, and is checked by [#E]P = O on 8 points.
  Let ℓ be the largest prime of #E. A curve is kept only when its ℓ-part is
  cyclic of order ℓ, so v = 1.
- **Target.** P = [#E/ℓ]·(random point) of order ℓ, and Q = [s]P for a
  secret s. Trial j draws a_j, b_j uniformly mod ℓ and decomposes
  R_j = [a_j]P + [b_j]Q.
- **Decomposition** (Gaudry–Diem, m = n = 4). The factor base FB is every
  lifted point ±P_x with x on the line. A relation is a 4-multiset of FB
  points summing to R_j.
  - All relations of a trial are found exactly by meet-in-the-middle over
    the table of 2-multiset sums, built once per line.
  - That is brute force. Its group-addition count is recorded, but it is
    **not** the cost of a summation-polynomial solver.
- **Linear algebra.** Columns are the distinct nonzero projections
  [#E/ℓ]P_x up to sign; rows use signed coefficients, and the unknown s gets
  its own column.
  - Relations are collected until the system has full rank D + 1, with a
    cap of 20,000 trials.
  - Then the system is solved mod ℓ, and [s]P = Q is checked.
- **Seeds.** Fixed strings, SplitMix64.
- **Timing.** No wall-clock time is measured, so this makes no benchmark or
  speed claim.

## Arms (the same curve, P, Q and trial targets for every arm)

| arm | factor base |
|:--|:--|
| `conj` | the first all-lift conjugate-pair line of the curve; a curve with none is skipped and counted |
| `std` | the standard line F_q (x₀ = 0, u = 1) |
| `rand` | one random line x₀ + uF_q |

## Cells

(p, n) ∈ {(13, 4), (101, 4)}. Curves are drawn with e₁, e₂, e₃ uniform and
distinct in K, until 16 are kept per cell. Kept means the order is known,
v = 1, and at least one all-lift conjugate-pair line exists. Every curve
drawn is recorded with the reason it was skipped.

## Measured quantities per (curve, arm)

- **k** is the number of lifted x on the line. **D** is the number of
  columns.
- **Y**, relations per trial, is distinct 4-multisets averaged over the
  trials made.
- **T** is trials to full rank. **N_rel** is the number of relations used.
  Also recorded: solved (bool) and the brute-force group additions.

## Predictions (decision rules, coded in the summarizer)

**Y1 (yield per trial).** Pooled per cell,
Y(conj)/Y(std) ∈ [0.6, 1.6] × (k_conj/k_std)⁴. The algebra predicts
(k_conj/k_std)⁴ ≈ 2⁴ = 16. Y(rand)/Y(std) ∈ [0.5, 2].

**Y2 (per column).** ρ = Y/D. Pooled per cell,
ρ(conj)/ρ(std) ∈ [0.6, 1.6] × 2³ = [4.8, 12.8].

**Y3 (complete solve).** Every kept curve is solved by `conj`, with [s]P = Q
verified. `std` and `rand` are reported where they solve within the cap.

**Y4 (trials to full rank).** Pooled per cell, the median of
T(std)/T(conj) over curves where both solve lies in [4, 16].

**Y5 (applicability, report only).** The share of drawn curves with no
all-lift conjugate-pair line. Each kept curve also has full 2-torsion over
K, so the family needs a cofactor divisible by 4 and never applies to
prime-order curves. That is algebra, stated here, not measured.

## Cost model (reported, derived, not a decision rule)

Write c_dec for the cost of one decomposition trial. A summation-polynomial
system of the same shape is assumed for every affine F_q-line, so c_dec is
the same for all arms; that assumption is not measured here. The two derived
cost models are:

- **relation phase:** T·c_dec;
- **linear algebra:** D² (dense) or D·(row weight) (sparse).

The cost model is reported as a ratio between arms, with the assumption
named. It is an IC-versus-IC factor-base comparison, so there is no rho
comparison and no leaderboard row.

## Controls, A/A, analyst

- **Positive control.** `conj` must recover s exactly; a wrong s falsifies
  the solver.
- **Negative control.** `rand` is a generic line with about half its x
  lifting, which should behave like `std`.
- **A/A.** Two runs, with byte-identical rows required.
- **Data size.** Rows over 64 KiB go to the artifact store, pinned through
  the manifest.
- **Analyst.** The producer, using coded rules.

## Amendments

(none)
