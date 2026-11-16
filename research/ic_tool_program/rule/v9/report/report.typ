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

== What changed between v8's code and main (paired by target)
*Read in wall-clock time, not in the run's unit.* Each process prices time in its own unit, `unit_ns` (one batched `add_many` addition, measured by the binary under test). The unit belongs to the binary: v9's is about 7% smaller than v9c's (9.51 against 10.24 ns at n = 53) on every target, at every size, on both nodes. Ratios of units across the two binaries (`matched.json`'s `ic_online_matched_over_control`, `rho_online_control_over_matched`, `setup_units_median`; the bars below) are therefore not time ratios. Within one process the unit cancels, so the cold ratios above are unaffected (7.454 in ns against 7.458 in units at n = 53).

In wall-clock time (independent review, recomputed from the raw rows): rho takes identical steps on every target and is about 1.5--2% faster a step in v9; the index calculus's set-up is faster in v9 (4.7% at n = 53, 7.8% at 2^47.2). That is what narrows the distance to rho: the paired cold ratio v9/v9c falls 2--6% from 2^38 up (0.968 at n = 53, 0.936 at 2^47.2) and rises 1.3% at 2^36.6.

#image("paired.svg", width: 100%)
The bars are in each binary's own unit and overstate main's slowdown by the unit difference; the red bars (cold ratios) are unit-free.

== The prediction
Declared: the index calculus's cold time at n = 53, v9 over v9c, in [0.85, 1.00]. The protocol did not fix the unit. In wall-clock time the ratio is 0.953 (median cold time 192.9 against 202.8 ms over 64 targets; every one of the 64 targets in [0.899, 0.974]): inside the band. In each binary's own unit it is 1.025: outside. Because the unit depends on the binary, only the wall-clock reading is a time ratio; neither reading was named in advance, and both are reported. The aimed-subset gather (#2448) and the other set-up changes since v8 show as a 4.7% faster set-up at n = 53.

== Slope of ln(cold ratio) on ln r
v9: 0.113 (95% t over six sizes [0.056, 0.169]; bootstrap over targets [0.097, 0.129]). v9c: 0.119 ([0.056, 0.183]; [0.104, 0.136]). Rule v8 measured 0.168 on its host. n and log2 r are not independent in these six curves.

== Deviations, disclosed
The first attempt ran locally and was lost with its container (no rows survive; amendment 1). On EKS, attempt a1 was lost to a Spot interruption after v9's first 29 rows at 2^36.6; a2 resumed from the synced rows on a second node of the same type, so that size of v9 is split across two nodes. Each run's `host.json` names a1's node; each pod's `host.txt` names its own. The rebuilt `ic` binaries differ in hash from the first attempt's (same sources); the hashes that ran are in `binaries.sha256` and were replayed.

Independent review (`independent-review.md`): admit with caveats. It recomputed all 12 cold ratios from the raw rows, re-verified all 1,678 certificates with its own GF(2^m) code, redrew the replay sample and replayed two records itself. Not independently re-derived: the bootstrap intervals (reproduced with this programme's tool). The replay certificate's `host` field names the earlier replay host; v9's replays ran on `fc-v378` (`validation/raw/native-replay.json`, `hosts`). Each run's `host.json` leaves `rustc`, `repository_commit` and `cargo_lock_sha256` empty. Arm order within a size was always v9 then v9c; the A/A drift in ns is up to ±1.7%.
