# EXP-PFDR-2d36f9-R2-FAM: the break-even ladder on three other structured families (result: O-SUB-BE)

Protocol [PROTOCOL.md](PROTOCOL.md) (frozen in commit b159698c17 before data; Amendment 1 after an aborted first attempt, kept in `aborted-attempt1/`). Report [REPORT.pdf](REPORT.pdf).
Reproduce: `scripts/run_all.sh` (about 17 minutes, 4 workers, 19,887 jobs), `scripts/make_fig.py`, `scripts/make_report.py`, PDF via headless Chromium.
Code: R2 driver with `family_base` (Dickson, geometric, quadratic x-progressions); patch `implementation.patch`, hashes `PROVENANCE.sha256`.

Reading `O-SUB-BE`: six valid structured lines (3 families x theta 0.30, 0.35; smallest rung count 2901 >= 2500), selftest and C3-C6 pass (C6 n = 10, mean 0.22, sd 0.88), no ALIVE line, no failed job.
Slopes of log2 kappa on log2 N: -0.005 to +0.007, largest upper 95% limit 0.0147 against 1/3. Plants recovered (0.149, 0.314-0.325, 0.486-0.491), Z/N 0.699 / 0.644.
With PR #3188's window / label-GAP lines: twelve structured x-base lines, none showing kappa growth over 16-32 bits.
Not covered: the FAM-1 mitm-engine arms (engine absent; small_x = the x-window already measured; subgroup needs a proper curve subgroup, these curves have prime order) and the bbc53e base (= the gamma plant control).
