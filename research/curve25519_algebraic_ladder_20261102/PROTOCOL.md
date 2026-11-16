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

## Amendment 1 (2026-11-02, before any Q-B or Q-C data): hybrid guessing

Owner request: add hybrid guessing to the bits form. `pkm-semaev` gains
`guess=G` (bits form only, `G < bits`, `G ≤ 12`). For a target, the oracle
enumerates the `2^G` assignments of the top G bits of the first summand,
runs F4 on each reduced system, and stops at the first consistent branch.
The target is refuted only when all branches refute. Every branch's F4
multiply-adds are charged; `pkm_guess_branches_solved` counts the systems
solved. Unit test: for `G ∈ {1, 2}` the verdicts equal the grid's and the
exact decomposable set on random and planted targets.

### Q-C: guessing sweep

The bits form on the k = 20 miniature at the t that the Q-B ladder shows
cheapest in S at w = 0.1, with `G ∈ {0, 1, 2, 3, 4}` at that t. The same
4 targets, seed and timeout as Q-B. The cost target is the owner's
`C(G) = 2^G·C_solve(after G)` plus grid recovery and verification, read
as F4 multiply-adds per call (`ecbench` solver block) and as S across the
band.

Prediction **C1**: total F4 multiply-adds per target first fall with G,
because each branch has G fewer Boolean variables and a lower solving
degree. They then rise once `2^G` dominates. The best G lies in 1 … 3. The
relation verdicts are independent of G, since refutation is exact.

## Amendment 2 (2026-11-02, after the timing smoke, before any Q-B, Q-C or Q-D data)

The timing smoke (`sessions/smoke-k20`, one target, `interrupted` after 4 of
8 runs; used for sizing only) showed:
- `iso-t3`: 131,202 F4 calls at 43k multiply-adds each. It exhausted its
  10⁸-trial cap with no relation set (1 class, matching Q-A).
- `bits-t3`: 85,005 calls at 87k each for 2 relations, S = 378.
- `iso-t6`: no finish within 30 minutes on one target.

At these costs the frozen Q-B ladder (14 arms × 4 targets, 1800 s timeouts)
would take days. Q-B and Q-C are therefore replaced by a stage measurement
with the same instrument (the `pkm-semaev` oracle, same code path, called
directly) plus the exact Q-A yields:

### Q-D: F4 cost per call and per relation

`curve25519_family f4cost <k> <form> <t> <G> <targets> <seed>`, k = 20,
forms `bits` (with `G ∈ {0, 1, 2, 3}` where `G < t`) and `isogeny`, t = 3 … 9.
200 random subgroup targets per cell, seed `2026110800 + t`, and a per-cell
wall cap of 600 s; a capped cell reports how many targets it finished.
Reported per cell:
- F4 calls, multiply-adds per target, the maximum solving degree, and
  undecided systems;
- the hit rate, against the exact Q-A λ;
- the projected multiply-adds per relation, `(multiply-adds per target)/λ`.

One verified end-to-end ecbench run then measures the cheapest cell, to
confirm that the stage figure carries over to S. Predictions:
- **B2** stands as stated.
- **C1** is restated per target: total multiply-adds per target are
  minimised at G ∈ {1, 2, 3}.
