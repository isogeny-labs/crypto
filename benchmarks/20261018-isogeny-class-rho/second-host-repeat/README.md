# Second-host repeats: same-host noise for P0 and K

Raw data: [`research/isogeny-class-rho-n45/second-host-repeat/`](../../../research/isogeny-class-rho-n45/second-host-repeat/) (runs 2 and 3). Run 1 is [`second-host/`](../second-host/). Original host: [`run-20261018T0857Z`](../../../research/isogeny-class-rho/runs/run-20261018T0857Z/).

Same commands and seed (`--seed 20261018`, class mode, 100 trials, 3 controls, plants, folds) on the same 4-thread VM, run back to back after run 1. Zero rows differ from the original in `iters`, `restarts`, `dps` or `escapes` in either repeat.

Aggregate ns/iter, this host / original host:

| class | run 1 | run 2 | run 3 |
|---|---|---|---|
| P0 | 0.6074 | 0.9034 | 0.6199 |
| K | 0.7503 | 0.7906 | 0.7607 |

Descriptive only (shared cloud VMs, no isolation). P0 run 2 was 1.49x slower than run 1; K varied by about 5%. The cause of the P0 slowdown was not determined. This does not establish a controlled speed comparison; `isolab-job.json` is an unsubmitted spec for one (no hub or credentials were available in the session that wrote it).

Run `python plot.py` here to recreate the SVG, PNG and PDF.
