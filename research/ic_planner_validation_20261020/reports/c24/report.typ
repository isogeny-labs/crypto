#set page(paper: "a4", margin: 2cm)
#set text(size: 10pt)
#show table: set text(size: 8.5pt)
= Run report: session c24 (`ECBS1h4087e7eb1b5a`)

*Protocol* PROTOCOL.md (supplementary). *Spec* `specs/c24.json` (`ECS1h4606425f06e9`), commit `05a0ebd92` (clean), binary `d0527101…7771`,
env class `ECBENV2h2196cccd7a31`, `ecbench run --wait --cpus auto`. Started 2026-10-20T17:00:53Z, finished 2026-10-20T17:01:22Z.

*Curve* `find_prime_order_curve(24, 3)` (ecbench `prime_search`; unregistered, not named by slug). *Workloads* 8 targets (seed 20261021). *Rounds* 3 measured + 1 warm-up, alternate order.
*Runs* 256 planned, 256 written, 256 verified. *Audit* OK, 12 exact replays (`sessions/c24.audit.json`).
*Isolation* (measured runs) double-base: L1 2, L2 22; full-rank: L1 2, L2 22; half-base: L1 2, L2 22; plan: L1 3, L2 21; plan-aa: L1 4, L2 20; random-targets: L1 15, L2 9; rho-neg: L1 2, L2 22; rootfind: L1 11, L2 13.

== Mean S per arm at w = 0, 0.1, 1 (ecbench `band`)

#table(columns: 4,
  [*arm*], [*w = 0*], [*w = 0.1*], [*w = 1*],
  [double-base], [19.572], [20.436], [28.217],
  [full-rank], [43.4], [51.236], [121.764],
  [half-base], [20.116], [23.827], [57.224],
  [plan], [13.145], [14.923], [30.924],
  [plan-aa], [13.145], [14.923], [30.924],
  [random-targets], [545.121], [546.811], [562.022],
  [rho-neg], [1.047], [1.14], [1.985],
  [rootfind], [1975.881], [2174.31], [3960.17],
)
Paired ratios against `plan`: `sessions/c24/comparisons/`, collected in `analysis/summary.json`.
Records: `sessions/c24/records.jsonl`. Observations only; the interpretation is in the study README.
