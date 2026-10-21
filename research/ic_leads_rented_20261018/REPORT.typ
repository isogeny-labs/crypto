#set page(paper: "a4", margin: 1.8cm)
#set text(size: 9.5pt)
#set table(stroke: 0.4pt)
= IC leads on rented hardware: launch-ready cards
2026-10-18. Study directory `research/ic_leads_rented_20261018/`. No experiment pod was launched (RunPod balance about \$21). The only pod was the PMU probe below.

== Cards, sorted by information per dollar
Planning estimates (*derived*). RunPod secure-cloud prices were queried 2026-10-18 21:13Z. p is each card's stated chance of reaching a non-inconclusive row of its frozen decision rule, a judgment.

#table(columns: 7, align: (left, left, left, right, right, right, right),
  [*id*], [*run*], [*host*], [*\$/h*], [*hours*], [*expected \$ (range)*], [*p*],
  [P6], [cross-host F4 counter drift, aarch64 half], [owner's Mac], [0], [≈1], [0], [0.8],
  [P5], [K at degree 4 with perf counters], [owner's Linux, PMU], [0], [18.5], [0], [0.8],
  [P1], [closure A3: decisive A1 count, 3,000 draws (running since 21:50Z)], [cpu3m-32-256], [1.76], [5.5], [9.70 (7.00–10.60)], [0.8],
  [P7], [closure C1, degree 5, draw 0 (running since 21:50Z)], [cpu3m-64-512], [3.52], [5.0], [17.60 (9.50–43.00)], [0.6],
  [P4], [equal-advice BL rho vs IC, n = 73, 83], [cpu3g-32-128], [1.28], [7.5], [9.60 (6.90–11.50)], [0.6],
  [P4-min], [equal-advice, n = 83 only], [cpu3g-32-128], [1.28], [3.5], [4.50 (2.80–5.90)], [0.6],
  [P2-min], [PKM msolve -t16, t7 target 0, 6 h], [cpu3m-16-128], [0.88], [6.6], [5.80], [0.3],
  [P2a], [PKM msolve -t16, N = 21, both targets], [cpu3m-32-256], [1.76], [22.5], [39.60 (10.60–40.50)], [0.5],
  [P2b], [PKM msolve -t16, N = 24, both targets], [cpu3m-32-256], [1.76], [22.5], [39.60 (10.60–40.50)], [0.35],
  [P3], [PKM native f4_fp_tower, 192 GB], [cpu3m-32-256], [1.76], [60], [106 (49–211)], [0.6],
)
Expected rented totals: P1 + P7, both launched by the parent session at 21:50Z, \$27.30; adding the minimal P4 and P2 (the about \$12 budget left) \$37.60. With the full P4, P2a and P2b, \$116.10; adding P3, \$222.10.

#figure(image("cards.svg", width: 100%), caption: [Expected cost of each rented card. Bars show the range, grey ticks the lifetime cap, and colour/p the stated chance of a decision. Source: `cards.json`, drawn by `plot.py`.])

== Measured in this PR
- *PMU probe* (pod `ax0ykpqnzxvbqh`, cpu3c-2-4, \$0.06/h, about 3.5 min, terminated). The host is an AMD EPYC 9655 and exposes a `cpu` PMU event source, but `perf_event_paranoid = 4`, so `perf_event_open` of cycles and instructions gives EPERM. Status: unsupported. This makes P5 an owner-host card. Negative control (authoring container): ENOENT, with no `cpu` source. Code-path control: `--selftest` on software events succeeds.
- *Parallel draw generation* (`dreg_ladder --shard`). 4 shards reproduce committed draws 0–7 and 40–43 of `m4-15-6`, including unsatisfiable draw 42, identically except for `secs`, at about 46 s per draw per process. On a tiny m = 4 cell, the merged shards equal the sequential run on draws 20–59. A corrupted committed file stops `gen-a3` with `REPLAY_MISMATCH` before any fresh draw.
- *Boot path of P1*, run locally with the clone replaced by a link: the gate passed 4/4, 4 fresh draws were made, the control closure was killed by memory (exit 137, recorded as the protocol says), and the tally ran.
- *msolve -t gate (P2)*. The m3_t5 control at `-t 4` reproduces the committed answer file. Its 1,917 s wall on a contended 4 vCPU box is not a speed-up figure.
- *Equal-advice job (P4)*. The n = 41 smoke ran end to end. The frozen `run.sh` exits 143 on success, and its A/A control is noise-sensitive on contended hosts. Both are reported, not changed.
- *Cross-host F4 dump (P6)*. The x86-64 half at commit `2245b75` (rustc 1.93.1) covers 64 runs and 58,420 nodes. Seq 58 gives 301,961 words, equal to the recorded x86 value, and the 24 non-warm-up m2 rows match 24/24. The macOS half is the owner's (`cross_host/mac_dump.sh`).
- *K perf instrument (P5)*. With software events, the interval counters reproduce the committed engine counters exactly (4,861 columns, 9,130,386 word ops, 609,340 rho steps).

None of this is an improvement to the IC baseline or a cost row. The cards only make the next measurements launchable.
