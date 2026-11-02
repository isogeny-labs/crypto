# Protocol: Semaev S_3 in Ed25519's Edwards coordinate (written before data)

Date 2026-11-02. Branch `lead5-edwards-semaev`. Frozen before any cell below was run.

## Question

Does writing Semaev's S_3 (m = 2) in the Edwards coordinate y of the
birationally equivalent twisted Edwards curve (u = (1 + y)/(1 - y), x_W = u + A/3),
with y = sum 2^j b_j, b_j^2 = b_j over [0, 2^t), make the algebraic
index-calculus oracle (`pkm-semaev`, F4 over the bit tower) cheaper per relation than
the same oracle on the Weierstrass interval x_W in [0, 2^t), on the k = 20
Curve25519 miniature?

## Open questions (stated as open, not as premises)

1. **Support of other Edwards-structured sets is open.** Measured so far is only:
   at k = 20, on one curve (no spread), the single interval [0, 2^t) in
   Edwards y carries about the same exact support as the same interval in
   Montgomery u (t = 10: 499 vs 533 signed classes) and the y^2 interval about
   half (245). That says nothing about other windows, offsets `lo`, sets
   structured in y, other sizes or other curves. No offset variant is measured
   here, so offsets stay open.
2. **Algebraic cost is open.** Whether the Edwards presentation changes the
   F4 cost (solving degree, matrix size, sparsity) is what this run measures at
   m = 2 only. The Möbius-equivalence argument below is a heuristic, not a proof.

## Construction

- Shift: x_W(T_0) = A/3, the unique rational root of x^3 + a x + b on these
  curves (cyclic 2-part, A^2 - 4 non-residue); derived exactly in `prepare`
  as the degree-1 gcd(x^3 + ax + b, x^p - x) (override: `edwards_shift=`).
  The harness also asserts it equals the miniature's own shift.
- Summand i: x_i = N_i/D_i, N_i = (1 + s) + (1 - s) y_i, D_i = 1 - y_i.
  S_3's coefficients in the third abscissa are multiplied by D_1^2 D_2^2
  (formulas in `s3_in_u_frac`); x_R stays a variable and is specialised
  numerically per target, so no (1 - y_R)^2 factor is needed. y = 1 (u = infinity)
  is excluded from V and from the base; the cleared system still admits y_1 = 1
  when x_R is itself in V (documented on `Membership::EdwardsBits`).
- Base: every point whose Edwards y is in [0, 2^t) minus {1}
  (`edwards_factor_base`, harness side). V is the same set of abscissae.

## Preregistered expectation

Edwards S_3 has no lower solving degree at m = 2 (the transform is a Möbius change
of variable on each summand), so any F4 difference is expected to be a constant
factor from the sparsity of the cleared polynomial and from the different
number of points in the interval. This is a prediction, to be checked, not a premise.

## Design

- k = 20 miniature (`build_miniature(20)`), oracle `pkm-semaev:m=2,mode=full`,
  form `bits` (Weierstrass interval) vs `edwards-bits`, equal t in {4,5,6,7},
  hybrid guess G in {0,3}: 16 cells.
- 100 random subgroup targets per cell; seed 2026111500 + t (the same stream
  for both forms and both G at a given t).
- Per-cell wall cap 600 s (the cell stops early and reports `capped`);
  memory cap through `max_nnz=30000000` F4 matrix entries (a system past it is
  counted as undecided and decided on the grid).
- Command: `curve25519_family f4cost 20 <form> <t> <G> 100 <seed> 600 30000000`.
- Cells run sequentially, two at a time, in interleaved form order.

## Reported per cell

F4 multiply-adds per target; solving degree (max); polynomial size = terms
of the specialised S_3 in tower normal form (per specialisation); hit rate
(relations returned / targets, each re-summed to the target); undecided count;
wall time. Per-relation cost = multiply-adds per target / lambda, where lambda
is the measured exact support from
`research/curve25519_algebraic_ladder_20261102/analysis/support.jsonl` (`xw`)
and `support-edwards.jsonl` (`ey`) at k = 20, re-measured with
`curve25519_family support 20 {xw,ey} t` for each cell (copied to
`analysis/support-cells.jsonl`). lambda is from a single curve and has no
spread; the measured hit rate is reported beside it as a cross-check, with the
binomial error (100 targets, low-count hit rates are very noisy).

## Scope of conclusions

Only at k = 20, one curve, m = 2, t in {4..7}, G in {0,3}, intervals starting at
0, one seed per cell. Nothing about m = 3, other offsets or sets, other k
or curves, or the 255-bit setting. Per-relation figures price only F4
multiply-adds, not the grid recovery or lifting. No comparison to rho is made
here (there is no complete DLP in this run).

## Amendment 1 (pre-data, 2026-11-02)

Written after rebasing onto main and before any cell ran. Main's `f4cost`
gained `[m] [mode] [lifts]` at argument positions 9 to 11, so the
`max_nnz` argument moved from position 9 to position 12. Cells, seeds, caps,
targets, forms, t, G and predictions are unchanged. The exact command line is

    curve25519_family f4cost 20 <form> <t> <G> 100 <seed> 600 2 full 0 30000000

with `<form>` in {bits, edwards-bits}, `<t>` in {4,5,6,7}, `<G>` in {0,3},
`<seed>` = 2026111500 + t. The "two at a time" line of the Design is replaced
by strictly sequential execution (interleaved form order), under
`ulimit -v 3500000` and `timeout 900` per cell. The `support` subcommand is
re-run per cell as written; the lambda used for per-relation cost is read from
the cell-matched support output.
