# Known-log arbitrary-target recovery: rendered evidence

These are browsing copies of the [source study](../../research/ecbench_known_log_targets_20261021/README.md),
its [results PDF](RESULTS.pdf), and eight run reports and graphs. The plotting
source and every measured value remain in that study. Each SVG and its 150 dpi
PNG come from the same rendering invocation.

At the largest toy fixture, cold modeled abscissa / known-log cost is
15.6487–16.9821 across the three native-work prices on primary targets and
16.6589–18.2168 on holdouts. Negation rho remains cheaper. Wall comparisons are
L0 and unadmitted; native work is not fully calibrated. The host is virtual
Linux x86_64 on AMD EPYC 9V74, five visible logical CPUs, one thread per solve.

This entry is **unvalidated research evidence**. `benchmarks/README.md` receives
no result row because the independent replay and validator artifacts required
by [the validation gate](../../docs/VALIDATION_GATE.md) are not yet available.
The [named problem](../../problems/known-log-recovery-cost-and-validation.md)
tracks those gaps. The original failed cofactor control remains visible beside
the corrected run; no frozen session is edited.
