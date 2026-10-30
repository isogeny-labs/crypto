# Independent factor-base / graph replication

This directory is the benchmark-browser copy of the
[source study](../../research/factor_base_replication_20261024/REPORT.md).

The run contains five operation-count arms over an independently derived cohort
of 1,260 public known-answer targets, paired with same-process source-negation
rho. All 6,300 candidate and 6,300 rho answers verified. At the representative
price, the complete-cohort candidate/rho ratios are 2.522734 (full/16), 2.513395
(full/32), 2.549755 (full/8), 2.995143 (half/16), and 3.984298 (quarter/16).
Every arm is above rho in all 36 full-cohort cells. The 16-bit-only full/8 slice
has 23 cells below and 13 above rho.

Files:

- [report PDF](report.pdf)
- [SVG graph](factor-base-replication.svg)
- [150 dpi PNG graph](factor-base-replication.png)

This copy is retained under “Studies awaiting benchmark admission.” The study
has a native validator and matched same-process rho, but no independent-host
replay certificate or isolated wall comparison, so it has no `VALIDATION.json`
and no admitted-results table row.
