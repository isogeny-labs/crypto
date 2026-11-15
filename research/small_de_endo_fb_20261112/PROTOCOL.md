# Small D_E with an endomorphism-compatible factor base: does solving cost drop?

Status: **draft protocol, written before any data** (2026-11-12). Freeze it by
hash before the first measured run; after that, add dated amendments only.

## Question (owner, 2026-11-12)

Over `F_p`, choose a curve with a small-norm endomorphism `ψ` (small `|D_E|`).
Pick coordinates and a factor base adapted to `ψ`. Does that reduce the
*actual* cost of solving the summation-polynomial decompositions and of the
whole relation phase? Coefficient sparsity or a low first-fall degree is not
evidence. The standard `S_m` keeps individual degree `2^{m-2}`, so the test is
the measured solving cost.

## Prior work this does not repeat

- `research/gaudry_quartic_d4_20261013`, `research/d4_symmetrised_s5_20261015`:
  the D₄-symmetrised S₅, one phase constant, no end-to-end `S`.
- `research/curve25519_t0_symmetry_20261102`: a symmetry stage effect.
- `research/automorphism_ideas_20261021` DGM-01: automorphism orbit folding of
  factor-base logs. It is a design and was never run.
- `examples/aut_orbit_coverage.rs`: the j = 0 column coverage count.
- Not covered by any of these: an endomorphism of degree > 1 (`D = −7, −8,
  −11`, after Tsakou–Ionica's extension-field mechanism), and an end-to-end
  relation cost on a prime field against a rho that uses the same symmetry.

## Arms (same `p`, prime `r ≍ p`, the same target stream per cell)

- **A0, generic:** a curve with large `|D|`, standard `S_m`, and the standard
  `x`-interval factor base.
- **A1, small D_E with the standard base:** an ablation that separates the
  curve from the adaptation.
- **A2, small D_E with adapted coordinates and base:** an invariant function
  of `ψ` as the coordinate (for example `x³` for `j = 0`, `x²` for `j = 1728`,
  or the `x`-map of the degree-2 or degree-3 endomorphism for `D = −7, −8,
  −11`). The factor base is closed under `ψ`, and logs fold along the orbits
  using the scalar action `ψ(G) = [λ]G`, checked in code before use.
- **Rho references:** plain rho, plus rho with the same automorphism group
  (the √|Aut| reduction) wherever A2 uses an automorphism. A2 is never compared only
  against a rho without that symmetry.

`D_E ∈ {−3, −4, −7, −8, −11}`, `m ∈ {2, 3}` (`S_3`, `S_4`), and `p` from about
`2^10` to as large as the solver allows inside the cap.

## Measured, per arm and cell

1. Solving degree reached by the Gröbner/Macaulay solve, and the largest
   matrix (rows × columns).
2. Decomposition probability per trial, and trials per relation.
3. Factor-base columns after folding, and relations needed.
4. Total relation-phase cost in counted field operations, plus the LA cost,
   priced at 0, 0.1 and 1 GAE per unpriced operation (AGENTS §5a).
5. End-to-end log recovery, with each log certified as `[k]G = T`.

The transfer or curve construction is a one-time cost and is reported
separately; it is never in a per-instance total.

## Decision rule

A2 counts as a gain only if its total relation-phase cost, or its
end-to-end cost, is below A1's in every cell of a `D_E`, across the §5a band,
with every log certified. A2 against A0 and against the matched rho is
reported as the distance remaining, not as a gate.

## Prediction (frozen before data)

- For `D = −3, −4`, A2 lowers columns by `|Aut|/2` and LA by about its square.
  Decomposition probability and solving degree are unchanged within noise.
  Against rho with the same automorphism, the gap stays at least the plain-IC
  gap. P = 0.7.
- For degree-2 or degree-3 endomorphisms, no factor base of useful size is
  closed under `ψ` without growing it by about `deg ψ` per orbit step, so A2
  costs no less than A1. P = 0.65.
- No cell ranks A2 below the matched rho. P = 0.85.

## Amendment 1 (pre-data), 2026-11-12: exact cells, bases, solvers, costs and stop rule

Status: written before the first measured run. The original sections above are
unchanged. `frozen.sha256` holds the sha256 of this file as of the end of this
amendment; after that, dated amendments only.

**Smoke test, not data.** While writing `small_de.rs` I ran the binary
unrecorded, in the session scratchpad, at 12, 16, 20, 24 and 28 bits with one
repetition and 4 targets, to debug it. Those runs showed three defects, all
fixed before this freeze: the CM acceptance test required the conductor of
`Z[pi]` to be 1 instead of the crater test; the Velu map is `+-psi` (the
isomorphism fixes a sign), so both signs of `lambda` are tested; and a trial
point `R = [a]P0` with `P0` in `<G>` (or a non-cyclic cofactor group) keeps its
cofactor class fixed, which leaves the relation matrix shift-invariant (only
difference relations; 33% of columns determined). Trials now add a random
cofactor-part element. The smoke runs are not used as data and are not
recorded. I saw their rough sizes (folded arms had fewer columns and fewer
relations); the three frozen predictions above were written before and are not
changed. The control arm and its prediction below were added after seeing them
and are labelled that way.

### Cells

- `p` has exactly `b` bits, `b in {12, 16, 20, 24, 28}`; `m in {2, 3}`;
  `D in {-3, -4, -7, -8, -11}` plus a generic curve (`D = 0`). One curve per
  `(b, D)`, so the curve-to-curve variance inside a `D` is not measured
  (limitation).
- Construction (`construct`, seed 20261112): scan primes upward from a
  seeded start in `[2^(b-1), 2^b)`. CM: `kronecker(D, p) = 1`; candidate curves
  `y^2 = x^3 + b` (`b = 1..48`, `D = -3`), `y^2 = x^3 + a x` (`a = 1..48`,
  `D = -4`), and for `D = -7, -8, -11` the curve of `j = -3375, 8000, -32768`
  (`y^2 = x^3 + 3kx + 2k`, `k = j/(1728 - j)`) and its quadratic twist.
  Generic: 24 random `(a, b)` per prime. Accept the first curve with
  `#E = h r`, `r` prime, `h <= 4` (smallest), `r > p/5`. The trace comes from
  the crate's Shanks-Mestre counter (`endo_census::count_trace`). CM curves must
  have `d_K = D` and `j(E) = j_D mod p` (crater, `End = O_K`) by
  `endo_census::analyse` and `on_crater`; the generic curve must have
  `|d_K| > 100` or `d_K` beyond the census bound 4400. The generator is
  `G = [h]P0` for a random `P0`; `[r]G = O` is checked.
- Construction is a one-time cost, timed apart, never in a per-instance total.

### Endomorphism and its scalar action

`lambda` is a root mod `r` of the characteristic polynomial: `x^2+x+1`
(`D=-3`), `x^2+1` (`-4`), `x^2-x+2` (`-7`), `x^2+2` (`-8`), `x^2-x+3` (`-11`);
for `-7, -8, -11` the map is `+-psi`, so `lambda` or `-lambda`.
`psi(G) = [lambda]G` is checked against an independent scalar multiplication
before the curve is used (`endo_for`; unit test `lambda_action_each_d` checks it
on several subgroup points and additivity on full-group points).

- `D = -3`: `(x, y) -> (omega x, y)`, `omega = (-1 + sqrt(-3))/2`.
  `D = -4`: `(x, y) -> (-x, i y)`.
- `D = -7, -8, -11`: Velu isogeny with a rational kernel of order 2
  (a root of the cubic) or order 3 (a root of the 3-division polynomial; the
  kernel points may lie over `F_{p^2}`), then the isomorphism `E' -> E`,
  `(x, y) -> (s x, u^3 y)`, `s = b a' / (b' a)`, `u^2 = s`. Charged as
  `deg - 1` group additions plus 2 field multiplications per image.

### Factor bases (all with the same number of points within a cell)

`n = ceil(#E^(1/m))`. A2 is built first; A1 (or A0) is the `x`-interval base
with exactly A2's point count, so decomposition probability is comparable.

- **x-interval** (A0, A1): `x = 1, 2, ...` keeping `x` with `x^3+ax+b` a
  nonzero square; one point per `x` (the square root returned by the crate's
  Tonelli/`(p+1)/4` routine), one column per point.
- **A2, `D = -3`** (`u = x^3` is the orbit invariant): seeds are scanned as
  above; each seed contributes its orbit `{P, zeta P, zeta^2 P}` (three
  `x`-classes `x, omega x, omega^2 x`), until at least `n` points. One column
  per orbit, `L(zeta^j P) = lambda^j L(P)`. Closed under `zeta` exactly (unit
  test). **`D = -4`** (`u = x^2`): orbit `{P, iota P}`, classes `x, -x`.
  Membership tests are one hash probe on `x` of any orbit member (the table
  holds all members, so the invariant `u` is not computed and not charged; this
  can only favour A2 by two multiplications per test).
- **A2, `D = -7, -8, -11`** (no finite closed base of useful size exists:
  `psi` acts on the order-`r` subgroup as `lambda`, whose order is about `r`
  unless it is small; the run records whether `lambda^k = 1` for some
  `k <= 1000`): the base is **layered**, `L = 4` layers. Seeds are the first
  `ceil(n/4)` `x`-interval points `P_s`; the base is
  `{psi^i(P_s) : i = 0..3}`, one column per seed, `L(psi^i P_s) = lambda^i L(P_s)`.
  It is closed under `psi` except at its top layer. Growth per orbit step: one
  point per seed (the base grows linearly, `4 |S|` points for `|S|` columns),
  while the `x`-degree of `psi^i`, as a rational function of the seed, grows as
  `(deg psi)^i`, 2 or 3 per step. A point whose `x` is already present is
  dropped (counted). Membership is one hash probe.
- **A3 (control, added after the smoke test)**: a generic curve (`D = 0`) with a
  layered base using multiplication by a random scalar `lambda_0` in place of
  `psi` (`L = 4`). It has no small `D_E`; it shows what folding alone buys.
  Its prediction (written after the smoke test, P = 0.8): `A3 / A0` equals the
  `A2 / A1` of `D = -7, -8, -11` within noise, i.e. any gain of the layered
  arms is from the folding, not from `D_E`.

### Logs and folding

`L(P) = log_G [h] P` is the homomorphism that discards the cofactor part. An
unknown `U_col` per column; `L(point) = coef * U_col (mod r)`. A relation
`R = sum eps_k P_{i_k}` gives `sum eps_k coef_k U_{col_k} = a (mod r)` for
`R = [a] P0 + K` (`[h] R = [a] G`, `K` a random element with `[h]K = O`).
Unit tests check `L(point) = coef * L(seed)` against a brute-force discrete log
for every folded base.

### Decomposition (solving)

- `m = 2`: all `(i, j)`, signs with `R = +-P_i +- P_j`. For each `i` the `S_3`
  quadratic in the second variable has the roots `x(R +- P_i)`; they are
  evaluated from the known `y_i, y_R` (one shared inversion per trial by
  Montgomery's trick, 7 field multiplications per `i`, no square root; the unit
  test `s3_quadratic_roots_are_the_sum_and_difference` checks `S_3` vanishes at
  them), and each root is a membership test (2 per `i`). Hits are confirmed with
  a priced group addition. The degree of the univariate solved is 2.
- `m = 3`: the `S_4` quartic in the first variable is solved through its
  factorisation into two nested `S_3` quadratics: for each base point `P_3` and
  sign, `U = R +- P_3` (a priced addition), then the `m = 2` sweep of `U` over
  `i >= i_3`. Degree of the univariate: 4, solved as 2 x 2. **No Groebner or
  Macaulay matrix is built**, so "largest matrix" is not applicable to the
  solving step; matrix sizes are reported for the relation elimination only.
  Triples are found once (`i_3 <= i_2 <= j`).
- Every decomposition of a trial `R` is kept. Decomposition probability per
  trial = share of trials with at least one hit; relations per trial and trials
  per relation are reported.

### Relation phase, linear algebra, stop rule

Trial points: a walk `R_{t+1} = R_t + J_s`, 16 jumps `J_s = [alpha_s] P0 + K_s`,
one priced addition (plus one for `K` when `h > 1`) per trial. The relation set
is eliminated by sparse modular Gaussian elimination (lightest column first,
sparsest pivot row; `la_ops` = multiply-adds, `invs` = inversions) at the
checkpoints `ceil(1.5^k * columns)`, `k = 0, 1, ...`; stop at the first
checkpoint with at least 90% of the columns determined (a column is determined
when back-substitution forces its value), or at the first checkpoint with
`1.5^k >= 12` (then `rule_met = false`, reported). All elimination attempts are
charged. Repetitions of the relation phase per arm: 20 (12 bits), 20 (16), 10
(20), 6 (24), 3 (28).

### Target stage and certificates

16 targets per cell, `T = [t]G`, `t` from a fixed seed, the same stream for
every arm and rho on that curve. Descent from the first repetition's known
logs: trials `R = [a] P0 + T'' + K` (`[h] T'' = T`) until a decomposition into
points of known columns; `t = sum eps L - a`. Every log is certified by an
independent `[k]G == T` (not charged) and must equal the planted `t`; a target
not solved within 200000 trials is a failure and counted.

### Costs

Priced: group additions and doublings (1 GAE each), including the base build
(`psi` evaluations, scalar multiplications), the trial walk, the
confirmations. Native, unpriced: field multiplications and squarings,
inversions, membership tests, elimination multiply-adds. Total at weight
`w in {0, 0.1, 1}` GAE per native operation = priced + `w` * native. Matched
rho: `rho_walk_with` on the same curve and targets, plain (`PointClasses`),
negation (`NegationClasses`), and for `D = -3` / `D = -4` the full group
(`CubicClasses`, `QuarticClasses`, `A = 6, 4`; their `[kappa]G` verification
ledger is charged); its priced cost is the run's GAE, its native count is
`canonicalisations_uncharged`. Rho for `D = -7, -8, -11` and generic uses
`A = 1, 2` only (no usable automorphism group). Secondary, labelled: field
operations (`muls + invs`, inversion = 1) with IC group additions converted at 4
field operations each (the crate's `FIELD_OPS_PER_ADD`).
Single-instance total = relation phase + mean target stage; amortised over 16
targets is also reported.

### Decision rule, made exact

For a `D`, A2 is a **gain** if in every `(b, m)` cell of that `D` its
relation-phase total is below A1's at all three weights, by more than twice the
pooled standard error of the repetitions (otherwise that cell is a tie), with
all targets certified; the same for the single-instance total. A cell where
`rule_met` fails in either arm is reported and counts against a gain claim.
Distance to the matched rho (ratio of IC single-instance total to the matched
rho's, per weight) is reported for every row and is not a gate. Rows are ranked
only where the order holds at 0, 0.1 and 1.

### Scorecard items (the three frozen predictions, checked as follows)

1. `D = -3, -4`: columns fall by `|Aut|/2` (3 and 2), relation-phase LA by about
   the square, decomposition probability and solving work per trial unchanged
   within noise, and the single-instance gap to the matched rho (with `A = 6`,
   `4`) at least the plain A1 gap.
2. `D = -7, -8, -11`: A2 costs no less than A1 (relation phase, all weights).
3. No cell ranks any IC arm below its matched rho across the band.
