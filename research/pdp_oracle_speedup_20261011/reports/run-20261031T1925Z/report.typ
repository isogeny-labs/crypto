#set page(margin: 1.8cm, numbering: "1")
#set text(size: 9.5pt)
#set heading(numbering: "1.")
#show raw: set text(size: 8pt)

#align(center)[
  #text(size: 15pt, weight: "bold")[PS-6 run report: calibration, five timed ecbench sessions, pair-table memory] \
  #text(size: 10pt)[`research/pdp_oracle_speedup_20261011/ps6_charge/raw/` --- 2026-10-31 from 19:24 UTC (calibration, then the timed sessions to 20:16; tests, replays and memory runs after)]
]

= What ran

`ps6_charge/run_ps6.sh` stages `calibrate`, `timed`, `reaudit`, `tests` and `memory`, inside one hold of the shared benchmark lock for the timed work (`tools/isolated_bench.py busy`), with ecbench's own whole-core reservation (`ecbench run --cpus auto`, a private inner lock). Binary SHA-256 `c8a7e9b9000f...`, commit `7ded1ca84` (clean tree). Host: Intel Xeon 2.10 GHz KVM guest, 4 vCPU, kernel 6.18.44-fc-v185 (`raw/host.txt`, `raw/manifest.json`). Specs `ps6_charge/specs/timed_n15 .. n27.json`: arms `rho-negation`, `s4-baseline`, `s4-baseline-control` (the baseline repeated, the A/A), `s4-grouplaw`, `mitm`, `mitm-negfold`; 8 targets x 5 rounds + 1 warm-up (4 targets at $n = 27$); alternate order.

*Runs requested and completed.* 288 + 288 + 288 + 288 + 144 = 1,296 requested, 1,296 completed and verified (1,080 measured, 216 warm-up), 0 failed, 0 timed out, 0 refused starts. Audits: all `ok` (`--replay-all` at $n <= 21$: 240 replays each; `--replay 12` at $n = 24, 27$). Calibration (`tools/isolated_bench.py run --cpus 3`, 58 s, not contended): 11 interleaved repetitions per rung. Unit tests and ecbench tests (`raw/logs/tests*.log`): `--bin ecbench ps6` 4 passed; `--lib ecbench` 102 passed; `--test ecbench` 16 passed.

*Isolation levels earned* (per run, measured rows): $n = 15$: 220 at L2, 68 at L1; 18: 217, 71; 21: 209, 79; 24: 181, 107; 27: 64, 80. The only L2 blockers recorded were one or more hypervisor steal ticks on the run CPU (389 runs) and run-queue delay above 0.5% of the solve (18 runs); L3 needs a lab host.

= Observations

#figure(
  image("wall.svg", width: 90%),
  caption: [Median wall time of the solve per run, by arm and rung, from the timed sessions (source `ps6_charge/raw/summary.json`, `timed`; isolation levels L1 and L2).],
)

#table(
  columns: (auto, auto, auto, auto, auto, auto),
  align: (right,) * 6,
  table.header[*n*][*baseline ms*][*kernel ms*][*MITM ms*][*rho ms*][*A/A: control / baseline*],
  [15], [14.5], [5.8], [5.1], [38.4], [0.999 (0.966--1.037)],
  [18], [41.2], [6.9], [5.6], [38.2], [1.014 (0.984--1.056)],
  [21], [749], [33.8], [13.8], [40.9], [1.019 (0.993--1.055)],
  [24], [2,521], [86.0], [23.3], [39.2], [1.006 (0.983--1.017)],
  [27], [46,151], [1,156], [132.5], [41.6], [0.999 (0.991--1.010)],
)

Medians over 40 runs per arm (20 at $n = 27$); the A/A column is the paired median of the baseline repeated over the baseline, with a 95% cluster-bootstrap interval over targets. Paired ratios with intervals for every arm pair, the isolation levels of the pairs and ecbench's own `admitted` / `descriptive` status are in `raw/summary.md` and the saved comparisons under each session's `comparisons/`. Kernel over baseline, paired median (95% interval): 0.385 (0.363--0.444), 0.163 (0.154--0.179), 0.044 (0.043--0.046), 0.035 (0.034--0.036), 0.025 (0.024--0.025) at $n = 15 .. 27$. At $n = 27$ every pair that includes the baseline is L1 (no L2 pair), so ecbench calls those comparisons descriptive; the same ratios at $n <= 21$ include L2 pairs. Rho's wall is 38--42 ms at every $n$.

*Memory.* `ps6_tool memory` built the library's pair table on each rung's base: 220, 1,016, 3,721, 18,711 and 69,118 entries at $n = 15 .. 27$; resident growth 0.0, 0.0, 0.1, 0.5 and 2.1 MiB (full and negation-folded builds give the same entry counts and 465 to 138,601 vs 240 to 69,432 group additions); a run's peak resident set is 100--108 MiB in every arm (`max_rss_kib` in each record).

*Replays.* The post-merge binary replayed the five counted sessions of the earlier run (all reproduced).

= Not in this report

No end-to-end claim and no transfer to any larger rung or curve. A baseline run takes 46 s at $n = 27$, so a rung above 27 was not attempted in this run.
