#set page(paper: "a4", margin: 2cm)
#set text(size: 10pt)
#set par(justify: true)
#show link: underline

= A table-free linear 2 + 1 decomposition on the frozen m = 3 cells

Study `research/groebner_free_matched_20261113`, run 2026-11-13T15:41Z, protocol frozen in commit `c90b1d9d54`, benchmark record `20261113T1539Z-ic-m3-linear21-vs-floor`. Stage diagnostic; every number is `[pcc:unverified]` (one session, shared 4-vCPU Xeon container, isolation L0, one core, no validator replay).

== Result

On the same cells and targets as `ic_m3_algebraic_vs_mitm_20261014` (seed 20261014), plus five larger cells, the arm `lin21-ht` decides three-summand decomposability with no Gröbner basis and no pair table. Measured:

- *Exactness.* The floor's verdict on 16,384 of 16,384 pool targets (eight cells with a pair table); 264 of 264 child solves agree and every witness adds up in the group.
- *Counted work.* Slope of log#sub[2] work per relation on n: trials 0.3352 [0.3311, 0.3397], floor additions 0.3346 [0.3313, 0.3382], difference 0.0006 [−0.0004, 0.0016].
- *Wall time.* 2.89 to 10.87 times the floor's online cost per relation; under the frozen 2026-10-14 rule the wall-time slope difference is [0.040, 0.057] ("grows faster"), which is one trial rising from about 250 ns to 500 ns over n = 13 to 41.
- *Memory.* 15.7 MB against 2,558 MB at n = 41. It grows with the base: 519 MB at n = 59 (the prediction "under 100 MB on every cell" failed at n = 53 and 59).
- *Against F4 on the S#sub[3] chain*, rerun in this session: 375 times cheaper per relation at n = 13, 8,500 times at n = 31.
- *Range.* Runs to n = 59, where the pair table would hold 5.5·10#super[11] entries.

It does not grow more slowly than the floor: it tries every base point as third summand.

#figure(image("figures/cost_per_relation.svg", width: 92%), caption: [Cost per relation, log#sub[2] wall ns. Floor and `lin21-ht` over whole pools (2,048 targets; 1,024 at n = 53; 512 at n = 59); `lin21` and F4 on 12 targets per cell. SAT points are from the 2026-10-14 session. Run 20261113T1541Z.])

== Whole pool

#table(
  columns: 9, align: right, stroke: 0.4pt, inset: 4pt,
  [n], [d], [|F|], [p], [lin ÷ floor online \[95%\]], [amortized \[95%\]], [ns/trial], [floor MB], [lin MB],
  [13], [5], [33], [0.494], [2.89 \[2.66, 3.13\]], [0.36 \[0.34, 0.38\]], [280], [6.9], [8.3],
  [17], [6], [63], [0.274], [5.04 \[4.78, 5.29\]], [1.32 \[1.23, 1.41\]], [202], [6.9], [8.2],
  [19], [7], [117], [0.404], [7.03 \[6.23, 8.10\]], [2.46 \[2.21, 2.82\]], [259], [7.4], [8.4],
  [23], [8], [237], [0.247], [9.88 \[9.57, 10.21\]], [5.56 \[5.28, 5.88\]], [260], [8.2], [8.4],
  [29], [10], [1,017], [0.279], [10.87 \[10.60, 11.13\]], [7.68 \[7.37, 7.98\]], [430], [19.5], [8.9],
  [31], [11], [2,101], [0.479], [7.00 \[6.79, 7.22\]], [3.03 \[2.87, 3.20\]], [394], [70.8], [9.1],
  [37], [13], [8,191], [0.488], [7.28 \[7.10, 7.46\]], [4.07 \[3.89, 4.25\]], [528], [722.8], [12.0],
  [41], [14], [16,223], [0.277], [6.37 \[6.27, 6.48\]], [4.53 \[4.38, 4.70\]], [489], [2,558.1], [15.7],
  [47], [16], [65,363], [0.284], [no table], [], [627], [], [36.3],
  [53], [18], [262,523], [0.287], [no table], [], [697], [], [132.9],
  [59], [20], [1,049,853], [0.287], [no table], [], [839], [], [518.7],
)

Measured values from `SUMMARY.json` (`ic_algebraic_floor linsummary`); intervals are a paired bootstrap over targets, 2,000 replicates. Zero disagreements on every floor cell.

#figure(image("figures/ratio_and_trial_cost.svg", width: 100%), caption: [Left: wall-time ratio to the floor per relation. Right: cost of one trial of `lin21-ht`.])

#figure(image("figures/peak_memory.svg", width: 80%), caption: [Peak resident set per cell (`VmHWM`), one cell per process.])

== The arm

With s = x#sub[1] + x#sub[2], p = x#sub[1]x#sub[2] and u = x(R − P#sub[3]), S#sub[3] = p#super[2] + u·p + u#super[2]s#super[2] + b is F#sub[2]-affine in (s, p); this is prior art in the repository (`ic_m2_linear_ladder_20261024`). `lin21-ht` eliminates p by the half-trace, p = u·(s + H(s) + H(b/u#super[2]) + ε), leaving n − 2d + 2 equations in d + 1 bits, and splits by x#sub[1] = s·H(p/s#super[2]).

== What stays open

To tie rho, m ≥ 4 and one scan of the base per relation at m = 4; the floor and the 2 + 1 + 1 form of this arm cost |F|#super[2]. The missing piece is a three-summand decision with no scan. In the symmetric functions of the three abscissae, S#sub[4] is F#sub[2]-affine except for e#sub[3]·(u·e#sub[3]#super[2] + u#super[3]e#sub[2]#super[2] + u#super[2]e#sub[1]#super[2]e#sub[3] + b·u·e#sub[1]#super[2]); `s4check` confirmed the identity on 2,095 witnesses with 0 of 2,095 control zeros. Plain linearisation of those four monomials reaches d < n/31 against d ≈ n/3 needed.

== Limits

One session at L0; the no-table curves have cofactors 1,318, 428 and 22,894; "no table" at n ≥ 47 is by size, not a failed build; prior art was searched in the checkouts because the knowledge base is not installed here; no validator replay, so no benchmarks or leaderboard row. Raw records: `runs/`, `runs.log`; binary SHA-256 in `runs/binary.sha256`.
