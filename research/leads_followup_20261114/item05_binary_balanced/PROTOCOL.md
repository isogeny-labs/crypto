# Item 5 protocol: does the last fall degree stay at 4 on balanced binary systems as k grows?

Frozen 2026-11-14, before any closure at k >= 16 was run. Nothing here is validated; every result is [pcc:unverified].

## Question
Descended two-summand binary system, balanced regime n = 2k-1 (structure e = r_eff - m_eff = 1), unit instances (|Z| = 0).
Law B: LFD = 2 + [k+e>=3] + [k+e>=9] + [k+e>=19].  With e = 1 this is 4 for k <= 17 and 5 for k >= 18.
Post-hoc law: max(2 + [k+e>=3] + [k+e>=9], A(e)) = 4 for every k here (A(1) = 3).  A_Z (|Z| <= 470): 4.
They disagree exactly at k >= 18.

## Instances (deep chain builder, unchanged)
`scripts/ic_bounds_bin_big_run.py::build_instance` (ml-cryptanalysis, same builder as make_cells.py), via `tools/build_balanced.py`:
base random (`random_nonstable_subspace`), curve ordinary (random a,b), target random-rational, n = 2k-1, k = 17, 18, 19, 20,
two instances per k, seeds 20261114+t, t = 0,1,... taken in order; an instance is kept only if |Z| = 0 (field enumerator)
and the system is not const (the builder's `unit_class`).  Draws rejected for |Z| > 0 are logged in `logs/build.log`
(selection by |Z| = 0 is the specified regime, not a result-dependent choice; it is made before any closure).
Controls: C3b_rand_ordinary_ran_1 (k=12) must reproduce the chain's recorded quotient ranks [0,0,46,1334,12951] (done, see RESULTS.md);
C3f_rand_ordinary_ran_0 (k=16, the chain's own instance) is run at D <= 4 for timing and as the k=16 point.  NOTE: the fetched chain snapshot
(ml-cryptanalysis/experiments/ic-bounds-binary-deep/fetched/chain-status.json) is an unfinished snapshot (886 s, finished=false) with
results only through C3b (k = 12) and C2b; there is no recorded k = 13..16 or C6b result locally, so the k = 16 run cannot be compared to a record.

## Measurement
`ladder --max-degree 4 --device cpu` (exact bit-packed GF(2) closure, degrees 0..4; the ladder stops at the first degree where the constant
column is a pivot).  Reading per instance:
* 1 in V_4 (status exact-unit, certificate_degree <= 4): d_lf <= 4 certified; the strict-fall list gives the last strict fall (LFD).  Hit for both laws if LFD = 4.
* 1 not in V_4 (not unit at D=4): d_lf >= 5 (lower bound); degree 5 run only if it fits (columns C(2k,<=5) >= 2.4e5 at k=17: it does not on this host in 2.5 h; it would be dropped and reported).
Stop-on-one inside a degree is not supported by the engine (no edit allowed); the ladder's per-degree unit test is the stop.

## Predictions (frozen)
| k | n | e | law B | post-hoc | A_Z |
|---|---|---|---|---|---|
| 16 | 31 | 1 | 4 | 4 | 4 |
| 17 | 33 | 1 | 4 | 4 | 4 |
| 18 | 35 | 1 | 5 | 4 | 4 |
| 19 | 37 | 1 | 5 | 4 | 4 |
| 20 | 39 | 1 | 5 | 4 | 4 |
Predicted quotient ranks at D=4 for a unit instance: equal to the column count C(2k,<=4) (k=16: 41449, 17: 52956, 18: 66712, 19: 82993, 20: 102091); degree-3 quotient rank = count of the closure at D=3.

## Falsifiers
* Law B falsified (as a k-driven floor) by one certified exact-unit instance with certificate degree 4 at k >= 18 (|Z| = 0, closed_all, repairs reported).
* Post-hoc / A_Z falsified by one instance at k >= 18 with 1 not in V_4 (then d_lf >= 5, subject to the lower-bound caveat).
* k = 17 is a below-threshold control: LFD 5 there would falsify both laws at once.
* Not a falsification: a run killed or not finished (reported as dropped).  A closure with repairs > 0 is reported with the repair count.

## Execution rules
Host 4 vCPU shared, OMP/OPENBLAS threads = 2, `ulimit -v 6000000`, engine flags `--batch 128 --work-gb 0.5 --tile-mb 32 --col-block 4096`
(tuned on the k = 12 control: 18.7 s vs 42 s at the defaults used first; output hash is flag independent, checked on the control).  Order: k increasing; per k all instances before the next k.
Time rule: a run expected over 2.5 h is not started; scaling estimate N^3 from the k = 12 control (18.7 s at N = 12951).
