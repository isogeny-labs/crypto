# Twist E26: unadmitted solver-worker diagnostics

[Source report](../../research/twist_routes_1_3_20261021/reports/block-20261025/RESULTS.md) and [PDF](RESULTS.pdf).
The study directory is the source of truth. These copies are not an admitted
benchmark result and carry no VALIDATION.json or matched-rho claim.

The 90-workload pilot retains 631 candidate cells, including its A/A control.
The figures show certified/incomplete cells and the one fixed p=59, B=8, m=2
instruction comparison. Counts are simulated guest instructions Ir, with
separate native process models and stock-msolve separate-worker records.
Construction, certification, root/sign lifting and complete IC are excluded.
No wall time or calibrated speedup is admitted. Host: Intel Xeon Platinum
8573C, managed Linux x86_64, L0. Same-input A/A checks pass; no confidence
interval or independent-host replication is claimed.

[Counts figure](COUNTS.svg) · [instruction figure](INSTRUCTIONS.svg).
Both PNG copies use the same Graphviz source at 150 dpi; editable sources and
recorded plot values are beside the study report.
