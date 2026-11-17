#set page(paper: "a4", margin: 2cm)
#set text(size: 10pt)
#show table: set text(size: 8.5pt)
= Run report: session A (`ECBS1hf4d23305c958`)

*Spec* `specs/A-1k-8k.json` (`ECS1h529376370841`), commit `a75e9b407` (clean), binary `797ea0fb…e91b`,
host env class `ECBENV2h43ce23acf702` (4-vCPU Xeon VM, Linux 6.18), `ecbench run --wait --cpus auto`,
run CPU 3. Started 2026-10-19 16:29 UTC, finished 18:16 UTC.

*Workloads* 8 targets (target seed 20261019) on `icv1-fp32-t7319-90da43e9`. *Rounds* 5 measured + 1 warm-up, alternate order.
*Arms* `rho-neg` (reference); `dense-N` and `cado-N` for N ∈ {1,024, 2,048, 4,096, 8,192}
(`ic.pipeline`, `prime-abscissa:size=N`, `mitm:negation_folded=1`; `incremental-gauss-full-rank` vs `sparse-cado`);
`cado-4096-aa` (control).

*Runs* 480 planned, 480 written, 480 verified, 0 failed or timed out. *Isolation earned*: IC runs L1
(hypervisor steal on the run CPU); `rho-neg` L2 (38) and L1 (2). *Audit* OK, 24 exact replays
(`sessions/A-1k-8k.audit.json`).

== Measured values (means of 40 measured runs per arm)

#table(columns: 6,
  [*arm*], [*S (measured)*], [*row_ops*], [*LA wall*], [*relations wall*], [*set-up wall*],
  [rho-neg], [0.870 [0.678, 1.077]], [—], [—], [—], [—],
  [dense-1024], [136.07], [28,749], [61 ms], [6.06 s], [0.16 s],
  [cado-1024], [136.07], [43,991], [44 ms], [6.08 s], [0.15 s],
  [dense-2048], [129.19], [61,209], [223 ms], [3.37 s], [1.05 s],
  [cado-2048], [129.19], [92,929], [113 ms], [3.31 s], [1.05 s],
  [dense-4096], [291.21], [129,316], [793 ms], [1.87 s], [5.83 s],
  [cado-4096], [291.21], [194,343], [355 ms], [1.72 s], [5.86 s],
  [cado-4096-aa], [291.21], [194,343], [363 ms], [1.77 s], [5.90 s],
  [dense-8192], [1044.38], [281,907], [5.77 s], [1.58 s], [37.1 s],
  [cado-8192], [1044.38], [422,323], [1.44 s], [1.24 s], [37.2 s],
)
S intervals from `ecbench table` (`analysis/table-A.md`). Wall times are descriptive (L1).
Derived ratios are in `analysis/A-1k-8k.json` (`scripts/analyze.jq`).

#figure(image("../../figures/la_wall_vs_n.svg", width: 80%), caption: [LA wall per run against N; sessions A and B (this run: N ≤ 8,192).])

Records: `sessions/A-1k-8k/records.jsonl`. Observations only; the interpretation is in the study README.
