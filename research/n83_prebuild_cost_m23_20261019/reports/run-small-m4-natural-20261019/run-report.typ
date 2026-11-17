#set page(paper: "a4", margin: 14mm)
#set text(font: "New Computer Modern", size: 9pt)
#let d = json("../../small-m4-v1/final/audit.json")
= N83 m=4: completed bounded public-query card
_small-m4-v1 · 2026-10-19 18:54:41–20:48:34 UTC · receipt audit PASS_

All four public searches ended at exactly 100,000 conflicts with UNKNOWN_search_cap, normal exit and no OOM. Two preflights, two constructed controls and two separate general-field replays passed before public search. The native audit checks all 132 archived files, pinned identities, container limits, serial order, phase coverage and progress counters.

#align(center, image("natural.svg", width: 88%))

#table(columns: (0.6fr,1.1fr,1.1fr,1.1fr,1.2fr,0.9fr), inset: 4pt,
  [*Arm*],[*Wall (s)*],[*XOR (s)*],[*Decisions*],[*Peak bytes*],[*Samples*],
  ..d.arms.map(a => (upper(a.label),str(calc.round(a.wall_s,digits:3)),str(calc.round(a.xor_s,digits:3)),str(a.counters.completed_decisions),str(a.peak_cgroup_bytes),str(a.progress_samples))).flatten())

Each base's repeated decision/conflict/restart/pass/row-operation counters agree exactly. K recorded 779,187 XOR passes and 476,855,934 repivot visits; D recorded 764,997 and 467,890,580. Repivot visits include fully assigned rows. Recorded XOR elapsed divided by worker wall is 97.508–97.561%; this is not a CPU-time fraction. The worker's root stage finished in 89.345–93.593 ms. Each final progress tuple equals the returned worker counters.

== Identity, limits and accounting

Curve: `icv1-f2m83-tm6151469093347-debefd74`, a=0, order 2417851639230796216685689, cofactor 4, polynomial z^83+z^45+z^2+z+1. K2048 contains 2,048 columns / 339,968 points; D12 contains 2,001 / 332,166. All arms use public target 0: (1834913121178197242856238, 8797964190732846213981377). Same deterministic solver and free selectors/phases, 1,790 s internal deadline, 1,800 s outer cap. One point, two repeats per base; no independent target sample or between-base ranking.

EC2 m8i.8xlarge Spot, CPU 31, one CPU quota, hard 4 GiB, zero swap, network disabled, inputs read-only. Other workloads shared the host: L0 observations. Worker wall charges import/index, corpus validation, model/root work and capped search. Base construction/export, prior full-object replay, Docker launch, later audit, complete relation/rank collection, linear algebra and matched rho are outside this interval. Full cold/reused IC cost and best-base selection remain open; candidate and best-runtime fields are null.

== Sources and next check

Study `research/n83_prebuild_cost_m23_20261019/`: frozen `SMALL_M4_PROTOCOL.md`, host-card v2, `small-m4-v1/final/README.md` and `audit.json`, this directory's `RUN_REPORT.md`. Native receipt auditor: `examples/n83_small_m4_audit.rs`. Worker commit: `611aad60e4d9952d6239a3382a77184da3640eeb`. Archive SHA-256: `b319353bf163c91b3180d6bf863b5129c580d3b7852751f345e4fd257b5abee9`.

Next proposed implementation: cache fully assigned, satisfied XOR rows until backjump or matrix rebuild, check incremental/backtracking correctness, then freeze fresh cache-off/on comparisons and holdouts. No improvement is measured here. Natural yield/rank remain censored. The earlier m=2/m=3 finite events and D18 export stay separately documented; canonical leaderboard and boundary rows do not move.
