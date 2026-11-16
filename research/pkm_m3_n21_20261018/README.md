# PKM Kummer tower, m = 3, N = 21, msolve rerun (LEADS.md L4): stopped as superseded

Lead L4 of [`../ecdlp_exponent_assessment_20261018/LEADS.md`](../ecdlp_exponent_assessment_20261018/LEADS.md).
Class: **replication, stopped**; no reading of its own.

- **What ran.** `run.sh` (launched detached by `launch.sh`) on the two
  `m = 3, N = 21` systems of `ic_next10_20261014/03-pkm-degree`, msolve
  0.11.0 built from source, one thread, 14 GB address-space cap, 6 h wall.
  Target 0 (`pkm_kummer_p2013265921_m3_t7_x221026520`) ran 5 h 32 min;
  target 1 was not started.
- **Why it stopped.** The same systems were run on `main` the same day on a
  27 GB host (`research/pkm_m3_n21_msolve_20261018`, PR #2175): both
  inconclusive at `D ≥ 7`, target 0 stopped by its wall at 4.3 h inside the
  sixth degree-7 round at 15.3 GB. This run could not go further on a
  15.7 GB host, so it was stopped by hand to free the host for L1.
- **What it adds.** An instrument check: msolve 0.11.0 reproduces the
  per-round new-element counts of #2175's msolve 0.6.5 on target 0
  (2 354, 4 269, 5 421 in the degree-7 rounds), with matrices of the same
  shape (98 282 × 101 435, 170 445 × 194 842, 203 174 × 228 589). The
  msolve version is not a variable in the `N = 21` reading.
- **Files.** `runs/*.log` (msolve `-v 2` trace), `runs/*.mem` (resident
  set sampled at the stop), `progress.txt`. The next step is #2175's:
  msolve `-t 8` on a 64 GB host with a 24 h wall, or the native
  `f4_fp_tower` cell on a 192 GB host.
