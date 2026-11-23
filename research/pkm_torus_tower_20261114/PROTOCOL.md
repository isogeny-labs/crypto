# Protocol: torus Kummer towers as PKM factor bases (frozen before any measurement)

Frozen 2026-11-14 19:25 UTC, before any closure run. Lead:
`LEAD-ICP-pkm-tower-algebraic-oracle` (PKM 2016, "finding good maps": slide 14
`p − 1` smooth, slide 15 isogeny kernels). Everything measured here is
`[pcc:unverified]` toy-scale stage diagnostic of the solver axis (AGENTS.md §2):
no pipeline is priced, no `S`, no leaderboard row.

## Claim under test

PKM's two map-finding methods are presentations of one object: the Kummer tower
of a one-dimensional algebraic group of order `p ∓ 1` restricted to a coset of its
`2^t`-torsion.

- `G_m` (order `p − 1`, needs `2^t | p − 1`): monomial coordinate `y_{j+1} = y_j²`
  (the repository's Kummer tower, arm K), or Möbius coordinate
  `a = (z + 1)/(z − 1)` with doubling `σ(a) = (a² + 1)/(2a)` (arm S).
- `T₂ = {z ∈ F_{p²} : z^{p+1} = 1}` (order `p + 1`, needs `2^t | p + 1`,
  `p ≡ 3 mod 4`): coordinate `a` with `z = (a + i)/(a − i)`, doubling
  `τ(a) = (a² − 1)/(2a)` (arm T). `τ` is, up to the scaling `x = 2a`, the
  `x`-coordinate part of the Vélu 2-isogeny `y² = x³ − 4x → y² = x³ + 16x ≅ y² = x³ + x`
  on the supersingular `j = 1728` chain that `pkm-aux-search --family supersingular-j1728`
  returns (`research/pkm_isogeny_tower_p192_20261015`, C6). `scripts/torus_check.py`
  verifies these identities numerically before the measurement; if it fails the
  claim is withdrawn and no closure runs.

In each arm the maps `L_j` have degree 2, so `Π deg L_j = 2^t = |V|` as PKM
requires, and no auxiliary curve is searched for.

## Question

Does the raw polynomial-calculus (PC) solving degree `D(m,t)` of the PKM chain
depend on which of the three presentations defines `V`?

## Instrument

`scripts/raw_closure.py`: exact degree-`D` PC closure on the *raw* presentation
(no tower normal form), in `F_p[y_{i,0..t−1}, u_0..u_{m−3}]`:

- tower axioms per block `i`: arm K `y_{i,j+1} − y_{i,j}²` (j < t−1) and
  `1 − y_{i,t−1}²`; arm S `2 y_{i,j} y_{i,j+1} − y_{i,j}² − 1` and
  `2 c y_{i,t−1} − y_{i,t−1}² − 1`; arm T `2 y_{i,j} y_{i,j+1} − y_{i,j}² + 1` and
  `2 c y_{i,t−1} − y_{i,t−1}² + 1`, with `c` the top-level constant that makes
  `V` the preimage of a coset of `μ_{2^t}` of size exactly `2^t`;
- `x_i = g·y_{i,0}` (K) or `x_i = y_{i,0}` (S, T);
- the split Semaev chain `S3(x_0,x_1,u_0)`, `S3(u_{k−1},x_{k+1},u_k)`,
  `S3(u_{m−3},x_{m−1},x_R)` (m ≥ 3) or `S3(x_0,x_1,x_R)` (m = 2), the same chain as
  `research/prime_leads_chase_20261113/L1-pkm/scripts/gen.py`.

`W_D` = smallest `F_p`-space containing the generators of degree ≤ D and closed
under `h ↦ v·h` for every variable `v` and every `h ∈ W_D` of degree ≤ D−1,
truncated to degree ≤ D. `D_PC` = least `D` with `1 ∈ W_D`; `1 ∉ W_{D−1}` is
computed exactly (full closure). Dense Gaussian elimination mod `p` in numpy
(batch reduction against a reduced basis by one matrix product per batch).

Targets: `x_R` random with no decomposition over `E(F_{p²})` (exhaustive check,
as in gen.py), so `1 ∈ ideal`; two seeds per cell. Curves: prime-order toy
curves drawn from the seed.

## Cells

- Arms K and S at `p = 40961` (`2^13 | p − 1`, the L1 prime): same curve, same
  `x_R`, same seed per cell. Arm T at `p = 8191` (`p + 1 = 2^13`) and at a second
  prime `≡ −1 mod 2^13` for replication.
- `m = 2`: `t = 1..5`; `m = 3`: `t = 1..3`. Larger cells only if the
  per-command cap allows (`timeout 1200`, one process, ≤ 6 GB).

## Controls

- Positive control: arm K must reproduce the L1 certified rows
  (`m=2`: 3,4,4,4,4 for t=1..5; `m=3`: 5,5,6 for t=1..3;
  `research/prime_leads_chase_20261113/L1-pkm/results.csv`). THEORY.md states
  raw PC degree equals NF-closure degree for `D ≥ 4`; at `m=2,t=1` (D=3) a
  one-degree raw/NF difference is possible and is reported, not a failure.
- Negative control: a planted (consistent) target per arm at `m=2,t=3` must not
  give `1 ∈ W_D` up to `D = 6`.

## Predictions (recorded now)

- P1: `D_PC` is identical across arms K, S, T at every cell (presentation
  invariance; the closure note's shape-matched null already behaved like the
  Kummer tower).
- P2 (consequence if P1 holds): the supersingular `j = 1728` shortcut is
  acceptable to PKM's relation phase in the only sense that matters for the
  exponent, and the auxiliary-curve search phase (0.8–25 % of `S` in
  `research/pkm_isogeny_tower_ecbench_20261015`; derived `2^74` GAE at P-192)
  is removed entirely. This is a constant/accounting change under the success
  criterion, **not** an exponent change; the degree growth that closes the lane
  is untouched.
- If arm T (or S) is lower than K by ≥ 1 at two or more cells with `t ≥ 3`, the
  presentation matters and a new lead is opened for the L1 instrument; if
  higher, the torus presentation is worse and the shortcut is not free.

## Decision rule

- CONFIRMED (P1): every finished cell agrees across arms.
- REFUTED: any finished cell with `t ≥ 2` disagrees by ≥ 1, replicated on the
  second seed.
- INCONCLUSIVE: positive control fails, or fewer than 4 cells finish per arm.

## Resource limits

One core, `timeout 1200` per command, numpy dense; cells that exceed the cap are
reported as censored (`D ≥ D'`).
