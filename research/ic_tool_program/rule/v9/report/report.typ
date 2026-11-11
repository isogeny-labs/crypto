#set page(paper: "a4", margin: 18mm)
#set text(size: 9.5pt)
= Rule v9: the index calculus against a matched rho at today's main

Two runs in one session on one host: *v9*, `ic` from main `f465f2fb`; *v9c*, the control, rule v8's `ic` rebuilt from `370e1919`. Each is timed against its own in-process matched rho (rule v8's batched canonicalisation). Six sizes of ledger §23, 64 targets and four A/A repeats a size, 408 rows a run, interleaved by size. Iso Lab EKS command Job, `c8a.2xlarge` Spot (AMD EPYC 9R45), 4 CPUs; no L2 admission (command Jobs have none). Source: `analysis.json` (each run), `matched.json`, `PROTOCOL.md` (declared before any row; amendment 1 before the rerun's first row). Class: measurement. No acceptance decision.

== Checks
408 of 408 rows pass in both runs; every logarithm is §23's. On 64 of 64 paired targets at every size, the two runs' rho walks take the same steps and recover the same logarithms. Both pins held. Both runs' rho is admissible against the strong walk at all six sizes (0.63--0.79 of its time a step). Validation (`VALIDATION.json`): every scalar verifies (838 and 840 certificates, scalar+1 control rejected); 8 of 8 seeded native replays with the exact binaries that ran are identical; 60 figures recompute; rho stays faster across the §5a band (0, 0.1, 1 GAE).

== Cold IC/rho, 95% bootstrap over targets
#table(columns: 6, align: (right, right, left, left, right, left),
[log2 r],[n],[v9 (main)],[v9c (v8's code)],[v8, 2026-10-19],[v9 / v9c, paired],
[36.64],[47],[4.08 [3.76, 4.44]],[4.03 [3.71, 4.39]],[4.67],[1.013 [1.012, 1.014]],
[38.0],[57],[4.73 [4.40, 5.13]],[4.83 [4.49, 5.24]],[5.70],[0.980 [0.978, 0.981]],
[39.0],[41],[6.14 [5.56, 6.86]],[6.38 [5.78, 7.13]],[7.07],[0.963 [0.962, 0.964]],
[44.26],[53],[7.46 [6.67, 8.39]],[7.71 [6.89, 8.68]],[11.75],[0.968 [0.966, 0.969]],
[44.52],[59],[6.83 [6.03, 7.83]],[7.02 [6.20, 8.06]],[10.65],[0.973 [0.972, 0.974]],
[47.21],[61],[11.13 [9.92, 12.60]],[11.89 [10.59, 13.46]],[18.07],[0.936 [0.935, 0.938]])
Rule v8's column ran on another host (Emerald Rapids) and is quoted, never paired. On this host v8's own code (v9c) reads 0.60--0.86 of v8's figures, so the host moves the ratio far more than the code does.

#image("cold-ratio.svg", width: 100%)

== What changed between v8's code and main (paired by target, in the run's time unit)
The index calculus's online time, v9 over v9c: 1.058--1.074. Rho's online time, v9c over v9: 0.938--0.945 (main's rho takes about 6% more of the unit a step). Index-calculus set-up medians, v9 over v9c (derived from `setup_units_median`): 1.084, 1.051, 1.021, 1.025, 1.033, 0.992. Main is slower than v8's code on this host in this unit, rho more so than the index calculus, so the distance between them narrows: the cold ratio falls 2--6% from 2^38 up (3.2% at n = 53, 6.4% at 2^47.2) and rises 1.3% at 2^36.6.

#image("paired.svg", width: 100%)

== The prediction
Declared: the index calculus's cold time at n = 53, v9 over v9c, in [0.85, 1.00]. Derived from the paired figures (cold-ratio pair 0.968 over the rho time pair 0.944) it is about 1.02--1.03: *the prediction does not hold*. The narrower distance comes from rho's slowdown, not from a faster index calculus. The aimed-subset gather (#2448) does not show as an absolute gain here.

== Slope of ln(cold ratio) on ln r
v9: 0.113 (95% t over six sizes [0.056, 0.169]; bootstrap over targets [0.097, 0.129]). v9c: 0.119 ([0.056, 0.183]; [0.104, 0.136]). Rule v8 measured 0.168 on its host. n and log2 r are not independent in these six curves.

== Deviations, disclosed
The first attempt ran locally and was lost with its container (no rows survive; amendment 1). On EKS, attempt a1 was lost to a Spot interruption after v9's first 29 rows at 2^36.6; a2 resumed from the synced rows on a second node of the same type, so that size of v9 is split across two nodes. Each run's `host.json` names a1's node; each pod's `host.txt` names its own. The rebuilt `ic` binaries differ in hash from the first attempt's (same sources); the hashes that ran are in `binaries.sha256` and were replayed.
