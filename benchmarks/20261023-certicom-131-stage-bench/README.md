# Certicom 131-bit CPU stage diagnostic, 2026-10-23

[Report PDF](REPORT.pdf) · [SVG chart](stage-rates.svg) · [150 dpi PNG](stage-rates.png)

The [study directory](../../research/certicom_131_stage_bench_20261023/REPORT.md)
contains the raw CSV, isolation admission and run record, analysis source,
frozen protocol and exact limits. The generic CPU rho-state-step medians are
207,822/s (ECC2K-130), 207,602/s (ECC2-131) and 87,055/s (ECCp-131).
S3-root-query medians are 48,080/s, 48,186/s and 9,522/s. Five samples
per cell ran on an Intel Xeon 6975P-C with CPUs 14 and 30 reserved.

These are exploratory stage measurements: the default isolation admission
failed, and the successful diagnostic used a relaxed 0.5-other-CPU threshold.
There is no complete logarithm recovery or matched full-cost rho/IC comparison.
Consequently this entry is **not admitted to the results table** in
`benchmarks/README.md` under the repository's `VALIDATION.json` gate. It
remains browsable here with its limitations and no fastest-tool claim.
