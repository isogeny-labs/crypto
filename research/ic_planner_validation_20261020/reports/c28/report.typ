#set page(paper: "a4", margin: 2cm)
#set text(size: 10pt)
#show table: set text(size: 8.5pt)
= Run report: session c28 (`ECBS1haabb242baa49`)

*Protocol* PROTOCOL.md (supplementary). *Spec* `specs/c28.json` (`ECS1h275d8f337a1f`), commit `05a0ebd92` (clean), binary `d0527101…7771`,
env class `ECBENV2h2196cccd7a31`, `ecbench run --wait --cpus auto`. Started 2026-10-20T17:01:25Z, finished 2026-10-20T17:05:08Z.

*Curve* `find_prime_order_curve(28, 1)` (ecbench `prime_search`; unregistered, not named by slug). *Workloads* 8 targets (seed 20261021). *Rounds* 3 measured + 1 warm-up, alternate order.
*Runs* 256 planned, 256 written, 256 verified. *Audit* OK, 12 exact replays (`sessions/c28.audit.json`).
*Isolation* (measured runs) double-base: L1 3, L2 21; full-rank: L1 11, L2 13; half-base: L1 2, L2 22; plan: L1 7, L2 17; plan-aa: L1 1, L2 23; random-targets: L1 23, L2 1; rho-neg: L1 1, L2 23; rootfind: L1 24.

== Mean S per arm at w = 0, 0.1, 1 (ecbench `band`)

#table(columns: 4,
  [*arm*], [*w = 0*], [*w = 0.1*], [*w = 1*],
  [double-base], [31.231], [32.578], [44.7],
  [full-rank], [84.026], [99.593], [239.696],
  [half-base], [29.829], [35.458], [86.119],
  [plan], [20.23], [23.032], [48.25],
  [plan-aa], [20.23], [23.032], [48.25],
  [random-targets], [1093.936], [1096.75], [1122.071],
  [rho-neg], [0.926], [1.016], [1.826],
  [rootfind], [8351.384], [9187.915], [16716.691],
)
Paired ratios against `plan`: `sessions/c28/comparisons/`, collected in `analysis/summary.json`.
Records: `sessions/c28/records.jsonl`. Observations only; the interpretation is in the study README.
