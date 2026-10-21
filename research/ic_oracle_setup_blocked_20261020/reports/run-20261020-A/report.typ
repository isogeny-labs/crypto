#set page(paper: "a4", margin: 2cm)
#set text(size: 10pt)
#show table: set text(size: 8.5pt)
= Run report: session A (`ECBS1h9e263e582e51`)

*Spec* `specs/A-1k-8k.json` (`ECS1h51f498729d24`), commit `e59331c96` (clean), binary `c1378033…41c5`,
host env class `ECBENV2h6639ae09b95e` (4-vCPU Xeon VM, Linux 6.18), `ecbench run --wait --cpus auto`,
run CPU 3. Started 2026-10-20 06:57 UTC, finished 08:14 UTC.

*Workloads* 8 targets (target seed 20261019) on `icv1-fp32-t7319-90da43e9`. *Rounds* 5 measured + 1 warm-up, alternate order.
*Arms* `rho-neg` (reference); `scalar-N`, `batch-N` and `blocked-N` for N ∈ {1,024, 2,048, 4,096, 8,192}
(`ic.pipeline`, `prime-abscissa:size=N`, `incremental-gauss`; oracles `mitm:negation_folded=1`,
`mitm-prime-batch:m=2`, `mitm-prime-blocked:m=2`); `blocked-4096-aa` (control).

*Runs* 672 planned, 672 written, 672 verified, 0 failed or timed out. *Isolation earned*: IC runs
mostly L1 (hypervisor steal; 0–11 of 40 per arm reached L2); `rho-neg` L2 (39) and L1 (1).
*Audit* OK, 24 exact replays (`sessions/A-1k-8k.audit.json`).

== Measured values (means of 40 measured runs per arm)

#table(columns: 5,
  [*arm*], [*S (measured)*], [*set-up wall*], [*relations wall*], [*LA wall*],
  [rho-neg], [0.966 [0.797, 1.125]], [—], [—], [—],
  [scalar-1024], [31.851 [30.428, 33.108]], [216.5 ms], [733.6 ms], [10.0 ms],
  [batch-1024], [31.851], [117.5 ms], [720.1 ms], [9.8 ms],
  [blocked-1024], [31.851], [55.2 ms], [804.8 ms], [10.2 ms],
  [scalar-2048], [71.816 [71.193, 72.342]], [1.435 s], [362.5 ms], [33.1 ms],
  [batch-2048], [71.816], [739.3 ms], [361.3 ms], [32.4 ms],
  [blocked-2048], [71.816], [237.9 ms], [420.0 ms], [33.9 ms],
  [scalar-4096], [260.011 [259.826, 260.190]], [8.398 s], [209.9 ms], [127.6 ms],
  [batch-4096], [260.011], [3.817 s], [203.1 ms], [119.5 ms],
  [blocked-4096], [260.011], [1.037 s], [245.0 ms], [137.7 ms],
  [blocked-4096-aa], [260.011], [1.056 s], [242.1 ms], [126.4 ms],
  [scalar-8192], [1026.139 [1026.010, 1026.256]], [43.60 s], [134.3 ms], [450.8 ms],
  [batch-8192], [1026.139], [20.48 s], [135.2 ms], [463.1 ms],
  [blocked-8192], [1026.139], [5.294 s], [148.4 ms], [456.6 ms],
)
S intervals from `ecbench table` (`analysis/table-A.md`); the three oracles of a size share one S.
Wall times are descriptive (L1). Equality (432 of 432 executions identical, A/A included) and the
derived ratios are in `analysis/A-1k-8k.json` (`scripts/analyze.jq`).

#figure(image("../../figures/setup_wall_vs_n.svg", width: 80%), caption: [Set-up wall per run against N; sessions A and B (this run: N ≤ 8,192).])

Records: `sessions/A-1k-8k/records.jsonl`. Observations only; the interpretation is in the study README.
