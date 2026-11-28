# PLAN: generic plane quartics, Diem IC with a c*q^(1/2) factor base, double large primes, sparse linear algebra

Written 2026-11-15 before the first measured run of this study. Repo HEAD at writing time:
`3c24520f7cdc1c47bac26cc7778d058e655b79a8`. Local development check at small parameters. It is not a lab
benchmark: no isolab-eks, no benchrec, one shared 4-CPU VM. Every number is `[pcc:unverified]`.
The only runs made before this file existed are debugging smoke runs at q = 1021 (3 instances, c in {1,2,4},
used to find and fix the duplicate-line bug below). Their output was discarded and is not evidence.

## Question

The generic plane-quartic Diem tool (`examples/quartic_ic.rs`, study `tian_n3_quartic_route_20261024`) beats
rho at zero-priced native work and loses at 0.1 and 1 GAE per native operation because its linear algebra is
dense (about q^1.4). The N4 study (`quartic_n4_la_20261021`) took the same pipeline on the GHS-weak family to
exponent 1.045. Does the generic quartic reach the same place when its relation graph is solved with sparse
linear algebra?

## Arms (same instance, same group arithmetic, same counters, one process)

* **IC-sparse** (new, `src/bin/quartic_diem_sparse.rs` with helper `src/qds_core.rs`): factor base of
  `b = ceil(c * sqrt q)` random rational points plus the pinned base point O; pairs of base points in a random
  order; the line through a pair meets the quartic in two more points X, Y (quadratic in the line parameter,
  square-root table); rows `x_i + x_j + x_X + x_Y - h = 0` (at most four points plus the hyperplane unknown h,
  weight <= 5); large-prime columns removed by minimum-weight elimination (`diem_quartic::eliminate_lp`);
  the factor-base system solved by sequential Wiedemann on the normal equations mod ell
  (`diem_quartic::wiedemann_solve`, 8 held-out rows checked); logs of large primes by propagation over the
  edge graph; target stage of the base tool; result checked `kG = Q` in the prime-order subgroup.
  Stopping rule of collection: `ceil(1.25 b) + 30 + 8` independent cycles in the ground component (lines
  through three or four base points are counted once; this fixes a double count in the base tool). On a
  rank failure the target grows by max(10, b/8) cycles and collection continues (every operation charged).
* **IC-dense A/B**: the same eliminated rows solved by the base tool's dense Gauss-Jordan (`nullspace`), run
  when b <= 800; its kernel must equal the Wiedemann solution (agreement flag).
* **rho**: base tool `run_rho` (distinguished points, 32 multipliers) on the same subgroup, same `Pic^0`
  arithmetic; **rhoneg**: its negation-map variant. Measured at q = 1021, 4093, 16381.

## Cells (frozen)

* q in {1021, 4093, 16381, 65521, 131071} (largest primes below 2^10, 2^12, 2^14, 2^16 and 2^17 - 1).
* Instances: curve seeds 7000, 7001, 7002 per q (3 per cell). Each instance runs c in {1, 2, 4} and the A/A repeat
  of c = 2 with a second IC seed (`cseed ^ 0x1D`; main seed `cseed ^ 0x1C`).
* Primary configuration for the predictions: c = 2 (fixed before the data; the sweep is secondary).
* rho: q = 1021: 20 plain + 20 negation-map runs per instance; q = 4093: 5 + 5; q = 16381: 1 + 1. No measured rho at
  65521 and 131071 (one run costs about 15 min and 30+ min). There rho is **modelled**: steps = kappa sqrt(ell),
  kappa the pooled mean of steps/sqrt(ell) over all verified measured plain-rho runs, additions per step and
  native operations per step from the measured runs at the largest measured q. Ratios against the model are
  labelled "modelled rho".
* Stopping rule: a q whose cells are not finished within 45 minutes of wall time is cut; a rho process over 40
  minutes is killed and reported as a blocker.

## Metrics and pricing

Per phase: pair trials, edges, kept rows, rows after elimination, row weights before and after elimination,
M, I, native NAT (hash probes, table lookups, union-find steps, elimination entries), LA operations mod ell
(elimination + Wiedemann + Berlekamp-Massey; dense for the A/B), group additions in the target stage, wall time.
Total cost in the repo's counted unit (GAE, one counted group addition):
`S(w) = (M + cI*I) / (gM + cI*gI) + w * (NAT + LA)`, w in {0, 0.1, 1}, cI = 3 (sensitivity: cI = 2 log2 q),
gM, gI the mean M and I of one group addition measured on the same instance (100 successful additions).
rho uses the same formula with its own M, I, NAT. LA share of S: `w * LA / S(w)`.
Exponent: least squares of ln(mean S) on ln q over q >= 2^12, bootstrap interval over instances (2000 draws).
Distance to matched rho is reported as S/rho at the three prices; ratios are ranked only where the order holds at
all three prices.

## Preregistered predictions

The thresholds are the lead's; evaluated for c = 2.

* **P1**: LA share of S below 20% at q >= 2^14, at w = 0.1 and at w = 1 (confirmed only if both hold).
* **P2**: exponent of S in [0.95, 1.15] on q >= 2^12, at each of w = 0, 0.1, 1.
* **P3**: S/rho below 1 at all three prices for every q >= 2^14 (65521 and 131071 against modelled rho).

Forecast written before the runs, from the operation counts of the debugging smoke runs (not a prediction in
the statistical sense): Wiedemann costs about 6 w b^2 mod-ell operations with w the post-elimination row weight
(about 20), i.e. about 250 b^2 at b = 64, and b^2 = c^2 q. At w = 1 that is of the order of rho's q^1.5 at
q = 2^14 to 2^17. So expect P2 to hold, P1 to be refuted at w = 1 (and probably at w = 0.1, where LA still
exceeds the relation phase), and P3 to hold at w = 0 and 0.1 but not at w = 1 for q <= 2^15, with the w = 1
ratio falling toward 1 at q = 2^17. The dense A/B should show cost growing as b^3 (q^1.5) and a crossover with
Wiedemann near b of 300 (q about 2^14 to 2^15 at c = 2).

## Controls

* every IC unit must verify kG = Q; failures are reported, not retried (except the in-unit rank retry above);
* A/A: two IC seeds of c = 2 on the same instance; the paired ratio S(seed 1)/S(seed 2) is reported;
* dense-vs-Wiedemann kernel agreement on every unit with b <= 800;
* rho verified `kG = Q` (tool check); rho calibration steps/sqrt(ell) reported (expected 1.0 to 1.5);
* unit tests for the arithmetic written here (row builder, propagation, pair decoding, end-to-end small q).

## Claim hygiene

No improvement claim. Generic random smooth quartics, prime-order subgroup of the Jacobian over F_q,
sizes q <= 2^17, shared VM, three instances per cell, rho above 16381 modelled. The gap to rho is a distance,
not a gate.

## Amendment 1 (2026-11-15, written after the main ladder reached q = 65521 and before any exploratory arm was run)

The first cells (q <= 16381, c = 2) showed that the row weight after elimination grows with q (21, 32, 48 at
q = 1021, 4093, 16381), so Wiedemann costs about 6 w b^2 with w growing, and that the matrix handed to Wiedemann
is not sparse (density w/b about 19% at q = 16381). Two **exploratory** row modes are added, run on the same
instances, seeds, c in {2, 4}, pricing and verification, and kept outside P1 to P3 (the primary arm stays the
frozen one above; no arm is selected after the fact):

* `bfs`: breadth-first potentials from the base points (the base tool's construction) returned as sparse rows;
  all non-tree edges of the ground component, no elimination. Target as in the primary arm (1.25 b + 38 cycles).
* `light`: collect `6 b + 38` cycles (QDS_LIGHTMULT = 6), build the breadth-first rows, hand the lightest
  `ceil(1.25 b) + 38` rows to Wiedemann after a coverage guard (each base column in at least two chosen rows);
  on a rank failure add max(10, b/8) lightest rows. This is the "choose cycle rows by weight under a rank guard"
  item of the N4 README, with the extra relation collection charged.

Prediction for the exploratory arms (before running them): `light` lowers the post-selection row weight by
about 2x and the Wiedemann operations by about 2x at q >= 16381 against the primary arm, and does not change
the verdicts of P1 (LA share stays above 80% at w = 0.1 and 1) or P3 at w = 1.

## Amendment 2 (2026-11-15, written after the main IC cells for q <= 65521 and all rho runs were in, before any q = 131071 cell was in)

* **Calibrated price added.** AGENTS.md section 5a (2026-11-12) asks for a calibrated weight `w_cal` for mod-p
  and mod-ell mulmod counters in addition to the band. `quartic_diem_sparse calib <q> 7000` measures, best of 5 and
  best of 7 on the shared VM, the time of one `Mont` multiply-add mod ell and of one `Pic^0` addition on the same
  curve; `w_cal` is their ratio. Orders are reported at w = 0, w_cal and 10 w_cal next to the band. This is an
  addition made after seeing that IC/rho is about 100x below parity at calibrated cost and above parity at 0.1
  and 1; it does not replace the band, which still governs. All native operations (mod-ell and hash) are priced
  at the same `w_cal`. q = 131071 uses the `w_cal` of the largest calibrated q (65521).
* **Instances at q = 131071.** One group order search at this size costs about 10 CPU-minutes per curve tried
  and several curves are needed to find a prime-order subgroup with cofactor <= 64, so the three instances run
  in three processes (`QDS_INST0`); the cell keeps however many instances finished within the 45-minute rule and
  the report states the count. Exploratory arms at 131071 run on instance 0 only, if at all.
* Modelled rho uses the plain (not negation-map) rho: at the measured sizes the negation map costs about twice
  the group operations per step (an inversion per step in this arithmetic), so its S is higher at every price.

Note added with the last runs: with the machine idle after the main cells finished, two things were run beyond
Amendment 2: one rho run per instance at q = 65521 (so that cell has measured rho for plain rho, 3 runs) and the
exploratory `light` arm at q = 131071 on all three instances (`QDS_INST0`), not instance 0 only.

Second note, after the first rho runs at q = 65521: their steps/sqrt(ell) were 0.59, 0.38 and 0.29 (pooled
kappa 1.23), with the smaller values on instances of larger cofactor. Additional plain-rho runs were started on
instances 1 and 2 of q = 65521 (14 and 6 more runs, run indices from 1) to see whether the modelled rho is
biased high at larger q; instance 0 (about 6 minutes per run) was not repeated for lack of time. The report shows
ratios against the pooled-kappa model, against measured rho at q = 65521 and a sensitivity row with kappa taken
at the largest measured q.
(Update: the extra runs on instance 1 were stopped for lack of CPU; only instance 2 of q = 65521 got 14 more plain
runs. Instances 0 and 1 have their single first run.)
