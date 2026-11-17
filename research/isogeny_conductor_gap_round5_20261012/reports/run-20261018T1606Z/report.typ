#set page(paper: "a4", margin: 1.8cm)
#set text(size: 10pt)
= Run report: e7b_k0
_2026-10-18, 16:06:42 to 16:09:45 UTC. research/isogeny_conductor_gap_round5_20261012, lead E7b._

*Spec* specs/e7b_k0.json. *Session* ECBS1heff1e81e9e36 at commit 6574178b6 (git_dirty false), audited with `ecbench verify --replay 12`: ok, 80 of 80 verified, 12 replays reproduced. Isolation earned L2 on 44 records, L1 on 36. *Workloads:* the a = 0 Koblitz curves over F#sub[2#super[19]] and F#sub[2#super[23]] (h = 4), 4 targets × 2 rounds. *Arms:* rho.negation; blind F4 descent and blind MITM on binary-subspace:dimension=9 with and without trace_kernel=1.

#table(columns: 7, align: right,
  [curve], [arm], [relations / trials], [F4 ops / relation], [max degree], [S], [wall / run],
  [n = 19], [f4 default], [808 / 3,488], [4,329,947], [3], [4.93], [2.93 s],
  [n = 19], [f4 trace-kernel], [952 / 2,777], [4,964,166], [3], [4.90], [5.15 s],
  [n = 23], [f4 default], [758 / 47,189], [4,634,927], [3], [5.43], [7.63 s],
  [n = 23], [f4 trace-kernel], [929 / 29,782], [3,329,596], [3], [3.86], [6.74 s])

Derived (TABLES_E7.md): yield trace-kernel ÷ default 1.48 [1.37, 1.60] (n = 19) and 1.94 [1.77, 2.14] (n = 23); F4 ops per relation default ÷ trace-kernel 0.87 and 1.39. MITM S 195.75 → 160.82 and 53.13 → 49.69; rho S 1.72 and 1.18. Wall times are host observations.

#figure(image("e7b_yield_ops.svg", width: 95%), caption: [Session ECBS1heff1e81e9e36: relations per trial (log scale) and F4 solver operations per relation, default against trace-kernel base.])
