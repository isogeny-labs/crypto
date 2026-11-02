# Curve25519-family algebraic track beyond t = 2 — protocol

Frozen 2026-11-02 before any data in this directory. It follows
[`problems/curve25519-family-algebraic-ic-tower.md`](../../problems/curve25519-family-algebraic-ic-tower.md):
`pkm-semaev` accepted only the Kummer tower `g·μ_(2^t)`, which needs
`2^t | p − 1`. Curve25519-family primes have `p ≡ 5 (mod 8)`, so `t ≤ 2`.

## What changes in the code

`pkm-semaev` gains a membership layer (`Membership`):

- **kummer** (unchanged): `x = g·y_0`, `y_j² = y_(j+1)`, `y_(t−1)² = 1`.
- **isogeny** (PKM's isogeny form, any p): `x = y_0`, and each Vélu step
  `y_(j+1) = y_j + τ_j/(y_j − ξ_j)` is cleared to
  `y_j² = y_j·y_(j+1) − ξ_j·y_(j+1) + ξ_j·y_j − τ_j`. The top level is
  `y_(t−1)² = (c + ξ)·y_(t−1) − cξ − τ`. Base `prime-isogeny:t=T`.
- **bits** (the owner's interval encoding, any p): `x = lo + Σ 2^j b_j`,
  `b_j² = b_j`, on any base whose abscissae lie in `[lo, lo + 2^T)`, with
  oracle parameter `bits=T`.

The F4 engine (`f4_fp_tower`) already takes all three rule shapes. The
grid reference and the lift are unchanged. New unit tests check, for both
new forms, that F4 equals the grid and the exact decomposable set on random
and planted targets, and that every relation sums to its target.

## Q-A: quotient support (exact, `curve25519_family support`)

For each form and t, on the k = 20 and k = 24 miniatures, report: physical
points; classes after torsion compression (`[8]P`); classes after signed
compression; `M` (distinct projected signed 2-sums); `λ = M/ℓ`; and the
naive count taking every physical pair (its overstatement factor). Forms:
`xw` (Weierstrass interval), `xm` (Montgomery interval), `w` (the
T₀-invariant `x + 1/x` interval, i.e. the 2-isogenous curve's coordinate),
`isogeny`, and `kummer` where it exists. t = 2 … 10.

Predictions:
- **A1.** Interval forms have about `2^(t−1)` signed classes. The `w` form
  has about the same number of classes per interval width as `xm`, but twice
  the physical points per class. That is the 2:1 T₀ compression, so it
  costs about 2× in physical points and nothing in classes.
- **A2.** The naive pair count overstates `M` by about 8 (the cofactor)
  times the per-class multiplicity.

## Q-B: end-to-end ecbench ladder (algebraic track, m = 2, mode=full)

k = 20 miniature, 4 public targets, 1 round, target seed `2026110700`,
timeout 1800 s per run. Arms:
- `iso-t{3..9}`: `prime-isogeny:t=T` + `pkm-semaev:m=2,mode=full`;
- `bits-t{3..9}`: `prime-abscissa:size=N_T` + `pkm-semaev:m=2,bits=T`,
  where `N_T` = the number of abscissae in `[0, 2^T)` carrying a point
  (from Q-A, form `xw`);
- `kummer-t2`: the previous ceiling;
- `inc`: the generic incumbent; `rho-neg`: matched rho.

Reported per arm: verified runs; S at w = 0 / 0.1 / 1 (`ecbench band`);
F4 multiply-adds per call; solving degree; relations and trials. A run that
times out is reported, not dropped.

Predictions:
- **B1.** Both new forms run and verify at t > 2.
- **B2.** S at w = 0 falls with t up to the point where trials per relation
  stop dominating. S at w ≥ 0.1 is dominated by F4 multiply-adds, which grow
  with t. No algebraic arm comes within 10× of `inc` at w = 0.1.
- **B3.** At equal t, bits uses as many tower variables as isogeny (t per
  summand). Its S₃ has degree 4 in the bits against degree 4 in `y_0`, so
  their F4 costs are within 4× of each other.

These are measurements of the algebraic track's distance, not improvement
claims against the generic incumbent. Hybrid guessing, chained S₃ and shared
preprocessing (owner's item 6) are next steps, and only once Q-B shows which
model is cheapest.
