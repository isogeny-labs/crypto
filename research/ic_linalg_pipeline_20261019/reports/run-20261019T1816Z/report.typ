#set page(paper: "a4", margin: 2cm)
#set text(size: 10pt)
#show table: set text(size: 8.5pt)
= Run report: session B (`ECBS1h0c4c69816435`)

*Spec* `specs/B-16k.json` (`ECS1hab3596c535bb`), commit `a75e9b407` (clean), binary `797ea0fb…e91b`,
host env class `ECBENV2h6639ae09b95e` (same 4-vCPU Xeon VM), `ecbench run --wait --cpus auto`.
Started 2026-10-19 18:16 UTC, finished 19:41 UTC.

*Workloads* 4 targets (target seed 20261019, the first four of session A's) on `icv1-fp32-t7319-90da43e9`.
*Rounds* 2 measured + 1 warm-up. *Arms* `rho-neg`, `dense-16384`, `cado-16384`.

*Runs* 36 planned, 36 written, 36 verified, 0 failed or timed out. *Isolation earned*: IC runs L1
(hypervisor steal); `rho-neg` L2. *Audit* OK, 6 exact replays (`sessions/B-16k.audit.json`).

== Measured values (means of 8 measured runs per arm)

#table(columns: 6,
  [*arm*], [*S (measured)*], [*row_ops*], [*LA wall*], [*relations wall*], [*set-up wall*],
  [rho-neg], [1.014 [0.549, 1.551]], [—], [—], [—], [—],
  [dense-16384], [4109.04], [613,491], [22.1 s], [1.15 s], [194.6 s],
  [cado-16384], [4109.04], [918,135], [6.12 s], [0.92 s], [193.0 s],
)
Wall times are descriptive (L1, and fewer than 5 rounds by design). Derived ratios are in `analysis/B-16k.json`.

#figure(image("../../figures/phase_wall.svg", width: 85%), caption: [Phase wall time per arm, sessions A and B (this run: the 16,384 bars).])

Records: `sessions/B-16k/records.jsonl`. Observations only.
