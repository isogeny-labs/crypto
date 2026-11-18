# R3 result: toy crossover diagnostic (2026-11-18)

**EMPIRICAL, local and unsigned.** Existing PR #4763 is the parent; exact producer run.py and independent audit.py remain unchanged. All 80/80 arms on 16 fresh seed cases passed audit, none censored. A/A deterministic metrics agreed.

| B | n seeds | median half/full cold EC adds | median half/full EC + 0.1 Fr | median half/full EC+Fr | median half/rho EC adds |
|---:|---:|---:|---:|---:|---:|
| 24 | 8 | 1.0449511 | 1.0417490 | 1.0546886 | 10.77924 |
| 40 | 8 | 0.7905589 | 0.7967650 | 0.8455410 | 18.75198 |

Previously: B=16 median 1.1194056 (R2, 8 cases); B=32 median 0.8705116 (R2, 24 cases). On THIS 11-bit curve there is a sample-dependent crossover between B24 and B32; B40 improves 20.9% over full on group additions, but is 18.75x the measured toy rho. This is not a native or extrapolatable speedup.

**Phase measurements (means, B24 half vs full):** pair table 150 vs 300 EC adds; relation collection 369.5 vs 176.875 EC adds; descent 3.125 vs 1.25 EC adds; LA 305.5 vs 268.125 modular ops; matrix construction 6067 vs 13762 ns (noisy, unisolated). **B40 half vs full:** table 410 vs 820 EC adds; collection 301 vs 157.375; descent 2.625 vs 1.5; LA 537.875 vs 451.75 modular ops; matrix 17061 vs 75597 ns (diagnostic only). Factor-base generation identical per paired case (fully charged in total).

**Reproduce:** python3 run.py --base 24 --seeds 8 --seed0 0x1c2026111860 --out raw/r3_b24.json; python3 run.py --base 40 --seeds 8 --seed0 0x1c2026111870 --out raw/r3_b40.json; python3 audit.py raw/r3_b24.json raw/r3_b40.json. The raw witness JSON and phase timings are retained in the downloadable conversation archive; per-seed costs are committed beside this report. No PCC signed verification. Workspace/Conductor blockage recorded in R3 protocol. Native B~p^(1/3) remains next; do not promote leaderboard.
