# Protocol: counted cost of Smith's reduction, end to end

Study `smith_transfer_cost_20261115`. Driver `src/bin/smith_transfer_cost.rs`;
transfer `smith_canonical::CountedTransfer`; solvers `genus_ladder` (on `H`)
and `quartic_ladder` (on `C`), unchanged. Written before any run on the
primes of the grid. A pilot at `p = 131` (not in the grid) was run first, only
to check that the driver produces certified records and to size the grid;
its numbers are in `pilot/` and are not results.

Every figure of this study is `[pcc:unverified]`. Synthetic toy curves; no
statement about a deployed curve. Shared 4-vCPU host: the counted units are
the result, wall time is informational only. No leaderboard or
`benchmarks/README.md` row (pre-gate).

## Question

For genus-3 hyperelliptic DLP instances to which Smith's isogeny applies,
what does solving cost, in one counted unit, by
(i) index calculus on `H`, (ii) rho on `H`, (iii) transfer to the plane
quartic `C` by `Psi` and Diem's index calculus on `C`?

## Instances

For each prime `p`, curve seeds `cs = 1, 2, ...` in order,
`f = genus_ladder::random_f(p, 3, cs)` (monic, degree 7). A seed is accepted
when, in this order:

1. `f` is squarefree and `#J_H(F_p) = h r` with `r` prime, `h <= 8`
   (`JCurve::jacobian_order`, `split_order`): "admitted";
2. `smith_prop24::analyze_curve` gives `full_success(false)` and a valid
   normal-form trigonal map with square `lc(s)`
   (`smith_canonical::find_trigonal`): "Smith-positive", resp. usable;
3. `CountedTransfer::new` succeeds: one quartic through 20 image points,
   smooth (`PlaneQuartic::new`), a rational image point for `P0`;
4. `#C(F_p) = #H(F_p)`.

The first `K = 3` accepted seeds per prime are the instances; the search
stops there or at 4000 seeds. Every rejection is counted by reason. A zeta
mismatch at step 4 would be a failure of the construction and is reported as
its own record. `G` and `T = [t]G` in the subgroup of order `r` are those of
`genus_ladder::instance_from(curve, order, h, r, target_seed = cs)`.

Grid: `p = 257, 521, 1031, 2053, 4099, 8209`, `K = 3`, seeds `1, 2` per
instance (6 runs per arm and prime). `p = 8209` is part of the grid; if its
instance search does not finish (4000 seeds or the session's time), the
prime is reported as not run and the fits use the primes that have data.

    smith_transfer_cost run --primes 257,521,1031,2053,4099,8209 --curves 3 \
        --seeds 2 --cc 2.0 --qc 3.0 --max-h 8 --threads 4 --rho-cap 1.2e8 \
        --seed-search 4000 --out runs.jsonl
    genus_ladder_analyze runs.jsonl analysis.json

## Arms (same `G`, `T`, same seed)

| arm | what runs |
|---|---|
| `H_C_double_lp+pre` | `genus_ladder::run_ic`, `Arm::DoubleLp`, `nb = ceil(2 p^(2/3))`, prefilter on, restart 4096 |
| `H_rho_neg` | `genus_ladder::run_rho` (negation map) |
| `C_transfer_diem_2lp` | `CountedTransfer::transfer` of `G` and `T`, then `quartic_ladder::run_diem`, `QArm::DoubleLp`, `nb = ceil(3 sqrt p)` |
| `C_rho_neg` | `quartic_ladder::run_rho` on the transferred classes; skipped above `1.2e6` expected steps; transfer not charged |

## Counted unit

The analyzer's readings, per run:
`fpm_mid = cantor_mul + 10 cantor_inv + rel_mul + 9 la_mul` (primary),
`fpm_low` (inversion 1, `la` 1) and `fpm_high` (inversion 30, `la` 9). For
the `C` arms `pic_mul -> cantor_mul`, `pic_inv -> cantor_inv`, as in the
ladder driver. For `C_transfer_diem_2lp` the transfer is charged in full:
`rel_mul += transfer_mul`, `cantor_inv += transfer_inv`, and both are also
recorded apart with their parts.

Counted in the transfer: building `T(t)`; the points of `H` and their image
points (square roots in `F_p` and `F_p^2`, Lagrange data, `T(t) b`); the
15-column fit; the `PlaneQuartic` base-point set-up; the conic conditions
and `class_from_conics`; the `Pic^0` additions; the Cantor additions and the
split tests and root finding used to move `G` and `T` onto rational points.

Not counted (all independent of `p` up to `log p` factors, and all small
against any arm at these sizes; stated again in RESULTS.md):
finding Smith's trigonal map (`analyze_curve`: factoring `f` and linear
algebra over extensions of degree at most 8); the fixed-degree polynomial
set-up in `Trigonal::new` (`F mod G`, `s`, its square root, the deltas);
the smoothness certificate inside `PlaneQuartic::new`. Instance search,
group orders, the zeta check and the checks of the transferred classes are
set-up or verification on every arm and are charged to none.

## Predictions

T1. Every run certifies (`certified` and `ok` true): the logarithm found on
`C` equals the planted `t` on `H`. `ord_ratio` (`ord G / ord Psi(G)`) is 1
on every instance (`r` is an odd prime and the kernel is a 2-group); a
ratio other than 1 would be reported and the comparison made modulo
`ord Psi(G)`.

T2. `C_transfer_diem_2lp` costs less than `H_C_double_lp+pre` on all three
FPM readings at every `p >= 1031`, and the ratio
`C_transfer_diem_2lp / H_C_double_lp+pre` (FPM mid) falls from `p = 1031`
to the largest prime.

T3. The transfer (`transfer_mul + 10 transfer_inv`) is below 5% of the
arm's FPM mid at every `p >= 1031`.

T4. Growth exponents in `p` (analyzer OLS fits of ln mean cost on ln p, FPM
mid, all primes with data): `H_C_double_lp+pre` in [1.0, 1.4];
`C_transfer_diem_2lp` in [0.6, 1.0]; `H_rho_neg` in [1.3, 1.6].

T5. Smith-positive instances occur at the census rate among admitted
curves: pooled over the grid, (Smith-positive) / (admitted) lies in
[0.05, 0.20] ("about 1 in 10").

## Analysis

* `genus_ladder_analyze`: cell means with hierarchical bootstrap intervals
  (curves, then runs within a curve; 2000 resamples) and exponent fits.
  Its `_over_rho` columns do not apply (the rho arm names differ).
* `render.py` (this directory) lays out the analyzer's output and computes
  the between-arm ratios, which the analyzer does not: ratio of cell means
  with a paired hierarchical bootstrap (same resampled curves and seeds in
  numerator and denominator, 2000 resamples, 95% percentile interval), for
  `C_transfer_diem_2lp / H_C_double_lp+pre`, each IC arm over `H_rho_neg`,
  `C_transfer_diem_2lp / C_rho_neg`, and the transfer share.
* A ratio is called below 1 (or above) only if its interval is on that side
  at all three FPM readings; otherwise unresolved.
* Rho references: `H_rho_neg` is the matched rho of the instance; the
  distance of each IC arm to it is reported as a ratio, not as a gate.

## Verdict rules

* T1 fails on any uncertified or wrong run, or any transfer failure.
* T2 holds if the ratio's interval is below 1 on the three FPM readings at
  each `p >= 1031` with data, and the mid ratio at the largest prime is
  below the one at 1031; partly holds / fails otherwise, stated per prime.
* T3, T4, T5: hold if the measured value (T4: the point estimate; its
  interval is reported) is inside the stated range.
* With 3 curves and 2 seeds per cell the intervals are wide; "unresolved"
  is an expected outcome and is reported as such.

## Not covered

Curves that are not Smith-positive (most of them); deg f = 8; the cost of
finding the trigonal map; primes of cryptographic size (the exponents are
fitted over one and a half decades of `p` at most); wall time.
