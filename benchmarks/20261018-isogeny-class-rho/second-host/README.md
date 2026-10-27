# Second-host class-mode replay

Full run data and host receipts: [`research/isogeny-class-rho-n45/second-host/`](../../../research/isogeny-class-rho-n45/second-host/). Original host trials: [`research/isogeny-class-rho/runs/run-20261018T0857Z/`](../../../research/isogeny-class-rho/runs/run-20261018T0857Z/).

The comparison matches all 22,700 rows by `(arm, vertex, source, generator, seed)`. There were zero differences in iterations, restarts, distinguished points, or escapes. The graph reports the ratio of aggregate `ns / iter` on the second host to the original host: P1 0.6120, P0 0.6074, B1 0.6165, K 0.7503. These are recorded cross-host timing ratios only; they are not a controlled performance comparison and do not establish a speedup.

Run `python plot.py` from this directory to recreate the SVG and PNG. The plot reads both raw trial CSV sets from `research/`.
