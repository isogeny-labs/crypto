#set page(paper: "a4", margin: 1.6cm)
#set text(size: 9pt)
#set par(justify: true)
#show heading: set text(size: 11pt)

= N4 v2: plane-quartic index calculus on the GHS-weak family, q = 2#super[8]..2#super[22], against plain and negation-map rho

2026-10-25. Branch `claude/quartic-n4v2-20261020`. Frozen protocol `PROTOCOL.md`, commit *82a9e87de3* (before any data); pilot parameters `PARAMS.md` (93bee805c4); pre-grid addendum (1416ae3f92). Grid run `grid-20261025` (294 jobs, 2026-10-26 01:58-03:12 UTC), Apple M4 Pro, shared host, L0 isolation, 4 `nice -n 10` processes. *Scope: the GHS-weak family E0/F#sub[q#super[3]] with \#E0(K) = 4 ell only*; no generic-curve, deployed-curve or key-recovery claim; no external implementation compared. Measured values are cold single-target costs in F#sub[q]-multiplication equivalents (M-eq), means over 3-5 instances per q (IC) and 2-150 runs per q (rho), 95% bootstrap intervals; unpriced native operations priced at w = 0, 0.1, 1 GAE (section 5a band).

== Results

- *Bend, frozen rule on e#sub[high] (k = 17..22, w = 0)*: B0, S, L, D, DL1 between 1 and 6/5; DL below 1; none "toward 1" or "toward 6/5" (w = 1: all between). Values: B0 1.141 [1.139, 1.142], S 1.108 [1.050, 1.153], L 1.162 [1.138, 1.184]; D 1.009 [1.007, 1.011], DL1 1.017 [1.015, 1.021], DL 0.988 [0.986, 0.989]. _Interpretation:_ the double-large-prime arms steepen from 0.75-0.90 (low range) and settle near 1; the single-large-prime arms stay below 6/5. Intervals resample 3 units per size and are narrow; Xeon conversion constants move D's e#sub[high] to 1.048. Full range k = 12..22: B0 1.209, DL 0.983, rho 1.504 [1.470, 1.532], rho-neg 1.543 [1.494, 1.583].
- *Crossover in M-eq vs rho-neg*: DL band-resolved lower from q = 991 at every larger size passing the calibration rule, interpolated q\* = 555 [389, 752] (w = 0), 276 [232, 319] (w = 1). B0: resolved from 16363, q\* = 6371 [5383, 8267].
- *Crossover in wall time vs rho-neg* (frozen measure): DL resolved from 8191, q\* = 5704 [1495, 7613]; B0 resolved only at 2097091, q\* = 2.77e5. CPU time (addendum): DL q\* = 1889 [1733, 4373], B0 q\* = 1.84e5.
- *Against frozen B0* (same instances, q = 2097091, w = 0): S 1.001 [1.000, 1.001] (engineering: bit-for-bit replay, O(N) memory), L 0.784 [0.767, 0.816], D 0.081 [0.081, 0.082], DL 0.042 [0.042, 0.042]. Best measured pipeline DL1 (post hoc): DL1/DL = 0.536 [0.536, 0.536] at q = 4194271, DL1/B0 = 0.022 at q = 2097091.
- *Negation map*: rho-neg/rho-plain = 0.715 [0.656, 0.777] at w = 0 (k >= 12, sizes passing calibration); at w = 1 rho-neg is the costlier reference.
- *Controls*: 501/501 IC units and 2442/2442 rho runs verified by kG = Q on E0; relations_bad = holdout_bad = 0; replay identical for S/L/D/DL at 12 sizes, B0 differs at all 11 replayed sizes. rho calibration excludes plain at k = 22 and rho-neg at k = 19, 20, 22 (frozen rule).
- *Predictions*: P1, P2, P5, P6 hold; P3 holds except the 1% relation-mul match below k = 14; P4 fails on SGE dimension (0.74-0.78, not 0.5) and on block/scalar LA (3.6x, not at most 3x).

== Graphs

#figure(image("reports/cost_vs_q.svg", width: 82%), caption: [Cold cost vs q, IC arms and both rho arms (w = 0). Run grid-20261025; rho cells failing the calibration rule omitted.])
#figure(image("reports/ratio_vs_q.svg", width: 100%), caption: [IC / rho with 95% bootstrap (solid w = 0, dotted w = 1 GAE). Run grid-20261025.])
#figure(image("reports/exponents.svg", width: 88%), caption: [Fitted exponents by range with 95% bootstrap; lines at 1, 6/5, 3/2.])
#figure(image("reports/vs_b0.svg", width: 82%), caption: [Each change against the frozen B0 on the same instances (paired bootstrap over seeds).])
#figure(image("reports/phases.svg", width: 100%), caption: [Phase shares of cold M-eq by k for B0, S, L, D, DL.])
#figure(image("reports/wall_vs_q.svg", width: 82%), caption: [Wall seconds per cold solve (practicality note; shared host).])

== Limits

Conversion constants are host-measured (robust calibration, PARAMS.md); the lmul/mul ratio steps from 1.55 to 2.37 at k = 17 and moves LA-dominated exponents (sensitivity with N4's Xeon constants in `results/summary_xeonconst.md`: B0 high-range 1.179, DL 1.029). Bootstrap intervals cover unit and run sampling only. rho is single-trail with one inversion per step; rho at k >= 19 has 2-12 runs per size. B0 is not run at k = 22 (N4's 21-bit point packing and 64-bit mod-ell addition). The k = 22 rho supplement (seeds 2, 3) is post hoc and reported separately. The first grid attempt (2026-10-21) was lost and is not used.
